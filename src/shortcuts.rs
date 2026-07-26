use std::{
    ffi::OsString,
    fs,
    os::windows::ffi::OsStringExt,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use windows::{
    Win32::{
        System::Com::{
            CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
            CoTaskMemFree, CoUninitialize, IPersistFile,
        },
        UI::Shell::{
            FOLDERID_Desktop, FOLDERID_Programs, IShellLinkW, SHGetKnownFolderPath, ShellLink,
        },
    },
    core::{Interface, PCWSTR, PWSTR},
};

use crate::{
    discord::{Host, Install},
    platform,
};

pub fn create(install: &Install, host: &Host, update_only: bool) -> Result<()> {
    let _com = ComApartment::new()?;
    let target = install.root.join("Update.exe");
    let icon = install.root.join("app.ico");
    let icon = if icon.is_file() {
        icon
    } else {
        host.executable.clone()
    };
    let title = install.channel.display_name();
    let paths = shortcut_paths(title)?;

    for path in paths {
        if update_only && !path.is_file() {
            continue;
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        create_one(
            &path,
            &target,
            &format!("--processStart {}", install.channel.executable_name()),
            &host.directory,
            &icon,
        )?;
    }
    Ok(())
}

pub fn remove(install: &Install) -> Result<()> {
    let title = install.channel.display_name();
    for path in shortcut_paths(title)? {
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("remove shortcut {}", path.display()))?;
        }
    }
    let menu_directory = known_folder(&FOLDERID_Programs)?.join("Discord Inc");
    let _ = fs::remove_dir(menu_directory);
    Ok(())
}

fn shortcut_paths(title: &str) -> Result<[PathBuf; 2]> {
    Ok([
        known_folder(&FOLDERID_Programs)?
            .join("Discord Inc")
            .join(format!("{title}.lnk")),
        known_folder(&FOLDERID_Desktop)?.join(format!("{title}.lnk")),
    ])
}

fn create_one(
    destination: &Path,
    target: &Path,
    arguments: &str,
    working_directory: &Path,
    icon: &Path,
) -> Result<()> {
    let link: IShellLinkW = unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }?;
    let target = platform::wide(target.as_os_str());
    let arguments = platform::wide(arguments);
    let working_directory = platform::wide(working_directory.as_os_str());
    let icon = platform::wide(icon.as_os_str());
    unsafe {
        link.SetPath(PCWSTR(target.as_ptr()))?;
        link.SetArguments(PCWSTR(arguments.as_ptr()))?;
        link.SetWorkingDirectory(PCWSTR(working_directory.as_ptr()))?;
        link.SetIconLocation(PCWSTR(icon.as_ptr()), 0)?;
    }
    let persist: IPersistFile = link.cast()?;
    let destination = platform::wide(destination.as_os_str());
    unsafe { persist.Save(PCWSTR(destination.as_ptr()), true) }?;
    Ok(())
}

fn known_folder(id: &windows::core::GUID) -> Result<PathBuf> {
    let value = unsafe { SHGetKnownFolderPath(id, Default::default(), None) }?;
    let path = unsafe { os_string_from_wide(value) };
    unsafe {
        CoTaskMemFree(Some(value.0.cast()));
    }
    Ok(PathBuf::from(path))
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

struct ComApartment;

impl ComApartment {
    fn new() -> Result<Self> {
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok() }?;
        Ok(Self)
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
