use std::{
    ffi::{OsStr, OsString},
    fs,
    os::windows::{
        ffi::{OsStrExt, OsStringExt},
        process::CommandExt,
    },
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    thread,
    time::Duration,
};

use anyhow::{Context, Result, bail};
use windows::{
    Win32::{
        Foundation::{CloseHandle, ERROR_ALREADY_EXISTS, GetLastError, HANDLE, HLOCAL, LocalFree},
        Security::WinTrust::{
            WINTRUST_ACTION_GENERIC_VERIFY_V2, WINTRUST_DATA, WINTRUST_DATA_0, WINTRUST_FILE_INFO,
            WTD_CACHE_ONLY_URL_RETRIEVAL, WTD_CHOICE_FILE, WTD_REVOCATION_CHECK_NONE,
            WTD_REVOKE_NONE, WTD_STATEACTION_IGNORE, WTD_UI_NONE, WinVerifyTrust,
        },
        Storage::FileSystem::{
            MOVEFILE_DELAY_UNTIL_REBOOT, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH,
            MoveFileExW, REPLACEFILE_WRITE_THROUGH, ReplaceFileW,
        },
        System::Threading::{
            CreateMutexW, INFINITE, OpenProcess, PROCESS_SYNCHRONIZE, ReleaseMutex,
            WaitForSingleObject,
        },
        UI::Shell::CommandLineToArgvW,
        UI::WindowsAndMessaging::{MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MessageBoxW},
    },
    core::{PCWSTR, PWSTR},
};

pub const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn local_app_data() -> Result<PathBuf> {
    env_path("LOCALAPPDATA")
}

pub fn roaming_app_data() -> Result<PathBuf> {
    env_path("APPDATA")
}

fn env_path(name: &str) -> Result<PathBuf> {
    std::env::var_os(name)
        .map(PathBuf::from)
        .with_context(|| format!("{name} is not defined"))
}

pub fn same_path(left: &Path, right: &Path) -> bool {
    let normalize = |path: &Path| {
        fs::canonicalize(path)
            .unwrap_or_else(|_| path.to_path_buf())
            .to_string_lossy()
            .trim_end_matches(['\\', '/'])
            .to_lowercase()
    };
    normalize(left) == normalize(right)
}

pub fn run_hidden(program: &Path, args: &[OsString], wait: bool) -> Result<Option<ExitStatus>> {
    let mut command = Command::new(program);
    command.args(args).creation_flags(CREATE_NO_WINDOW);
    if wait {
        Ok(Some(command.status()?))
    } else {
        command.spawn()?;
        Ok(None)
    }
}

pub fn kill_process(image_name: &str) {
    let _ = Command::new("taskkill.exe")
        .args(["/F", "/T", "/IM", image_name])
        .creation_flags(CREATE_NO_WINDOW)
        .status();
}

pub fn message(title: &str, body: &str, error: bool) {
    let title = wide(title);
    let body = wide(body);
    unsafe {
        MessageBoxW(
            None,
            PCWSTR(body.as_ptr()),
            PCWSTR(title.as_ptr()),
            MB_OK
                | if error {
                    MB_ICONERROR
                } else {
                    MB_ICONINFORMATION
                },
        );
    }
}

pub fn wide(value: impl AsRef<OsStr>) -> Vec<u16> {
    value.as_ref().encode_wide().chain(Some(0)).collect()
}

pub fn split_command_line(value: &OsStr) -> Result<Vec<OsString>> {
    let command_line = wide(value);
    let mut count = 0;
    let arguments =
        unsafe { CommandLineToArgvW(PCWSTR(command_line.as_ptr()), &mut count as *mut i32) };
    if arguments.is_null() {
        bail!("CommandLineToArgvW failed");
    }

    let pointers = unsafe { std::slice::from_raw_parts(arguments, count as usize) };
    let output = pointers
        .iter()
        .map(|argument| unsafe { os_string_from_wide(*argument) })
        .collect();
    unsafe {
        LocalFree(Some(HLOCAL(arguments.cast())));
    }
    Ok(output)
}

unsafe fn os_string_from_wide(value: PWSTR) -> OsString {
    let mut length = 0;
    unsafe {
        while *value.0.add(length) != 0 {
            length += 1;
        }
        OsString::from_wide(std::slice::from_raw_parts(value.0, length))
    }
}

pub struct InstanceMutex(HANDLE);

impl InstanceMutex {
    pub fn acquire(name: &str) -> Result<Self> {
        let name = wide(name);
        let handle = unsafe { CreateMutexW(None, true, PCWSTR(name.as_ptr()))? };
        if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
            unsafe {
                WaitForSingleObject(handle, INFINITE);
            }
        }
        Ok(Self(handle))
    }
}

impl Drop for InstanceMutex {
    fn drop(&mut self) {
        unsafe {
            let _ = ReleaseMutex(self.0);
            let _ = CloseHandle(self.0);
        }
    }
}

pub fn publish_file(source: &Path, destination: &Path) -> Result<()> {
    let source_wide = wide(source.as_os_str());
    let destination_wide = wide(destination.as_os_str());
    let replace = destination.exists();
    for attempt in 0..=50 {
        let result = if replace {
            unsafe {
                ReplaceFileW(
                    PCWSTR(destination_wide.as_ptr()),
                    PCWSTR(source_wide.as_ptr()),
                    PCWSTR::null(),
                    REPLACEFILE_WRITE_THROUGH,
                    None,
                    None,
                )
            }
        } else {
            unsafe {
                MoveFileExW(
                    PCWSTR(source_wide.as_ptr()),
                    PCWSTR(destination_wide.as_ptr()),
                    MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
                )
            }
        };
        match result {
            Ok(()) => return Ok(()),
            Err(error) if attempt < 50 && is_transient_file_lock(&error) => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(error) => {
                let operation = if replace { "replace" } else { "publish" };
                return Err(error).with_context(|| {
                    format!(
                        "atomically {operation} {} with {}",
                        destination.display(),
                        source.display()
                    )
                });
            }
        }
    }
    unreachable!()
}

fn is_transient_file_lock(error: &windows::core::Error) -> bool {
    matches!(error.code().0 as u32, 0x8007_0005 | 0x8007_0020)
}

pub fn verify_authenticode(path: &Path) -> Result<()> {
    let wide_path = wide(path.as_os_str());
    let mut file = WINTRUST_FILE_INFO {
        cbStruct: size_of::<WINTRUST_FILE_INFO>() as u32,
        pcwszFilePath: PCWSTR(wide_path.as_ptr()),
        ..Default::default()
    };
    let mut data = WINTRUST_DATA {
        cbStruct: size_of::<WINTRUST_DATA>() as u32,
        dwUIChoice: WTD_UI_NONE,
        fdwRevocationChecks: WTD_REVOKE_NONE,
        dwUnionChoice: WTD_CHOICE_FILE,
        Anonymous: WINTRUST_DATA_0 {
            pFile: &mut file as *mut _,
        },
        dwStateAction: WTD_STATEACTION_IGNORE,
        dwProvFlags: WTD_CACHE_ONLY_URL_RETRIEVAL | WTD_REVOCATION_CHECK_NONE,
        ..Default::default()
    };
    let mut action = WINTRUST_ACTION_GENERIC_VERIFY_V2;
    let status = unsafe {
        WinVerifyTrust(
            Default::default(),
            &mut action,
            &mut data as *mut _ as *mut _,
        )
    };
    if status == 0 {
        Ok(())
    } else {
        bail!(
            "{} does not have a trusted Authenticode signature (0x{:08x})",
            path.display(),
            status as u32
        )
    }
}

pub fn wait_for_process(process_id: u32) {
    let Ok(handle) = (unsafe { OpenProcess(PROCESS_SYNCHRONIZE, false, process_id) }) else {
        return;
    };
    unsafe {
        WaitForSingleObject(handle, 30_000);
        let _ = CloseHandle(handle);
    }
}

pub fn delete_after_reboot(path: &Path) {
    let path = wide(path.as_os_str());
    unsafe {
        let _ = MoveFileExW(
            PCWSTR(path.as_ptr()),
            PCWSTR::null(),
            MOVEFILE_DELAY_UNTIL_REBOOT,
        );
    }
}

pub fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        bail!("{message}")
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    #[test]
    fn instance_mutex_excludes_a_second_owner() {
        let name = format!("Local\\QuiTwinTestMutex-{}", std::process::id());
        let first = InstanceMutex::acquire(&name).unwrap();
        let (sender, receiver) = mpsc::channel();
        let thread = std::thread::spawn(move || {
            let _second = InstanceMutex::acquire(&name).unwrap();
            sender.send(()).unwrap();
        });

        assert!(receiver.recv_timeout(Duration::from_millis(100)).is_err());
        drop(first);
        receiver.recv_timeout(Duration::from_secs(2)).unwrap();
        thread.join().unwrap();
    }
}
