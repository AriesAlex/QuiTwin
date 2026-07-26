#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod discord;
mod install;
mod logging;
mod network;
mod platform;
mod shadow;
mod shortcuts;
mod ui;

use std::env;

use anyhow::Result;

fn main() {
    if let Err(error) = run() {
        logging::write(&format!("fatal error: {error:#}"));
        ui::show_error(&format!(
            "{error:#}\n\nDiagnostic log:\n{}",
            logging::path().display()
        ));
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = env::args_os().skip(1).collect::<Vec<_>>();
    let executable = env::current_exe()?;
    logging::init(initial_log_path(&executable));
    logging::write(&format!(
        "QuiTwin {} started from {}",
        env!("CARGO_PKG_VERSION"),
        executable.display()
    ));

    if args
        .first()
        .is_some_and(|arg| arg == "--quitwin-uninstall-helper")
    {
        return install::run_uninstall_helper(&executable, &args[1..]);
    }

    if install::is_installed_launcher(&executable) {
        install::run_installed(&executable, &args)
    } else {
        install::run_portable(&executable, &args)
    }
}

fn initial_log_path(executable: &std::path::Path) -> std::path::PathBuf {
    if executable
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case("Update.exe"))
    {
        executable
            .parent()
            .map(|root| root.join(".quitwin").join("QuiTwin.log"))
            .unwrap_or_else(fallback_log_path)
    } else {
        fallback_log_path()
    }
}

fn fallback_log_path() -> std::path::PathBuf {
    env::temp_dir().join("QuiTwin.log")
}
