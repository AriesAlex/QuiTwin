use std::{env, path::PathBuf};

use windows::{
    Win32::{
        System::Com::{
            CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
            CoUninitialize,
        },
        UI::Shell::{
            CLSID_ProgressDialog, IProgressDialog, PROGDLG_AUTOTIME, PROGDLG_NOCANCEL,
            PROGDLG_NOMINIMIZE,
        },
    },
    core::{IUnknown, PCWSTR},
};

use crate::platform;

pub fn show_error(message: &str) {
    platform::message("QuiTwin", message, true);
}

pub fn show_success(message: &str) {
    platform::message("QuiTwin", message, false);
}

pub fn play_success_sound() -> bool {
    let system_sound = env::var_os("WINDIR").map(PathBuf::from).map(|windows| {
        windows
            .join("Media")
            .join("Windows Proximity Notification.wav")
    });
    system_sound.is_some_and(|sound| platform::play_sound_file(&sound))
        || platform::play_sound_memory(include_bytes!("../assets/success.wav"))
}

pub struct Progress {
    dialog: Option<IProgressDialog>,
    com_initialized: bool,
    last_line: String,
}

impl Progress {
    pub fn new(line: impl Into<String>) -> Self {
        let line = line.into();
        let mut progress = Self {
            dialog: None,
            com_initialized: false,
            last_line: String::new(),
        };
        unsafe {
            if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_ok() {
                progress.com_initialized = true;
                if let Ok(dialog) = CoCreateInstance::<_, IProgressDialog>(
                    &CLSID_ProgressDialog,
                    None::<&IUnknown>,
                    CLSCTX_INPROC_SERVER,
                ) {
                    let title = platform::wide("QuiTwin");
                    let _ = dialog.SetTitle(PCWSTR(title.as_ptr()));
                    let _ = dialog.StartProgressDialog(
                        None,
                        None::<&IUnknown>,
                        PROGDLG_AUTOTIME | PROGDLG_NOCANCEL | PROGDLG_NOMINIMIZE,
                        None,
                    );
                    progress.dialog = Some(dialog);
                }
            }
        }
        progress.set_line(line);
        progress
    }

    pub fn set_line(&mut self, line: impl Into<String>) {
        self.last_line = line.into();
        if let Some(dialog) = &self.dialog {
            let line = platform::wide(&self.last_line);
            unsafe {
                let _ = dialog.SetLine(1, PCWSTR(line.as_ptr()), false, None);
            }
        }
    }

    pub fn set_download(&mut self, downloaded: u64, total: Option<u64>) {
        if let (Some(dialog), Some(total)) = (&self.dialog, total) {
            unsafe {
                let _ = dialog.SetProgress64(downloaded, total);
            }
        }
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        if let Some(dialog) = self.dialog.take() {
            unsafe {
                let _ = dialog.StopProgressDialog();
            }
        }
        if self.com_initialized {
            unsafe {
                CoUninitialize();
            }
        }
    }
}
