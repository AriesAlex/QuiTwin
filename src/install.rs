use std::{
    ffi::{OsStr, OsString},
    fs,
    io::ErrorKind,
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

use anyhow::{Context, Result, bail};
use windows::Win32::System::Threading::CREATE_NEW_PROCESS_GROUP;
use winreg::{
    RegKey,
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
};

use crate::{
    discord::{self, Channel, Install},
    logging, network, platform, shadow, shortcuts,
    ui::{self, Progress},
};

const INSTALLED_NAME: &str = "Update.exe";
const INSTALL_FLAG: &str = "--quitwin-install";

pub fn is_installed_launcher(executable: &Path) -> bool {
    executable
        .file_name()
        .is_some_and(|name| name.eq_ignore_ascii_case(INSTALLED_NAME))
        && executable
            .parent()
            .and_then(discord::install_at_root)
            .is_some()
}

pub fn run_portable(executable: &Path, args: &[OsString]) -> Result<()> {
    let _mutex = platform::InstanceMutex::acquire("Local\\QuiTwinInstaller")?;
    let mut progress = Progress::new("Finding Discord…");
    let install = match discord::choose(discord::discover()?) {
        Some(install) => install,
        None => install_discord(&mut progress)?,
    };

    let state_directory = install.root.join(".quitwin");
    fs::create_dir_all(&state_directory)?;
    logging::init(state_directory.join("QuiTwin.log"));
    logging::write(&format!(
        "installing into {} ({:?})",
        install.root.display(),
        install.channel
    ));
    platform::kill_process(install.channel.executable_name());
    progress.set_line("Installing the update-safe launcher…");
    let installed = install_self(executable, &install)?;
    let mut forwarded = vec![OsString::from(INSTALL_FLAG)];
    forwarded.extend_from_slice(args);
    drop(progress);
    platform::run_hidden(&installed, &forwarded, false)?;
    Ok(())
}

pub fn run_installed(executable: &Path, args: &[OsString]) -> Result<()> {
    let root = executable.parent().context("Update.exe has no parent")?;
    let install = discord::discover()?
        .into_iter()
        .find(|install| platform::same_path(&install.root, root))
        .context("QuiTwin is not inside a recognized Discord installation")?;

    if has_arg(args, "--uninstall") {
        logging::write("starting QuiTwin uninstall helper");
        return start_uninstall_helper(
            executable,
            &install,
            has_arg(args, "-s") || has_arg(args, "--silent"),
        );
    }
    if option_value(args, "--createShortcut").is_some() {
        let host = discord::latest_host(&install)?;
        shortcuts::create(&install, &host, has_arg(args, "--updateOnly"))?;
        return Ok(());
    }
    if option_value(args, "--removeShortcut").is_some() {
        shortcuts::remove(&install)?;
        return Ok(());
    }
    if has_option(args, "--update")
        || has_option(args, "--updateSelf")
        || has_option(args, "--checkForUpdate")
    {
        logging::write("ignored obsolete Squirrel update command; Discord uses updater.node");
        return Ok(());
    }

    if has_arg(args, INSTALL_FLAG) || args.is_empty() {
        let mut progress = Progress::new("Installing QuiTwin…");
        install_runtime(&install, &mut progress)?;
        drop(progress);
        launch(&install, &[], false)?;
        if has_arg(args, INSTALL_FLAG) {
            ui::show_success(
                "QuiTwin and Equicord are installed.\n\nDiscord updates can no longer remove the mod.",
            );
        }
        return Ok(());
    }

    if option_value(args, "--processStart").is_some()
        || option_value(args, "--processStartAndWait").is_some()
    {
        return launch(
            &install,
            &discord::parse_process_start_args(args),
            option_value(args, "--processStartAndWait").is_some(),
        );
    }

    bail!("unsupported Discord launcher arguments: {args:?}")
}

fn install_discord(progress: &mut Progress) -> Result<Install> {
    let temporary =
        std::env::temp_dir().join(format!("QuiTwin-DiscordSetup-{}.exe", std::process::id()));
    network::download_discord_setup(&temporary, progress)?;
    progress.set_line("Installing Discord…");
    let status = Command::new(&temporary)
        .arg("--silent")
        .creation_flags(platform::CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP.0)
        .status()
        .context("run DiscordSetup.exe")?;
    let _ = fs::remove_file(&temporary);
    if !status.success() {
        bail!("DiscordSetup.exe exited with {status}");
    }
    discord::choose(discord::discover()?)
        .context("Discord installer completed but no installation was found")
}

fn install_self(source: &Path, install: &Install) -> Result<PathBuf> {
    let destination = install.root.join(INSTALLED_NAME);
    let staging = install.root.join("Update.quitwin.new.exe");
    fs::create_dir_all(install.root.join(".quitwin"))?;

    fs::copy(source, &staging).context("copy QuiTwin into Discord")?;
    let staged = fs::OpenOptions::new().write(true).open(&staging)?;
    staged.sync_all()?;
    drop(staged);
    platform::publish_file(&staging, &destination)?;
    remove_stale_squirrel_backup(install)?;
    Ok(destination)
}

fn remove_stale_squirrel_backup(install: &Install) -> Result<()> {
    for name in ["Update.squirrel.exe", "Update.squirrel.new.exe"] {
        let path = install.root.join(name);
        if path.exists() {
            fs::remove_file(&path).with_context(|| format!("remove {}", path.display()))?;
        }
    }
    Ok(())
}

fn install_runtime(install: &Install, progress: &mut Progress) -> Result<()> {
    let _mutex = platform::InstanceMutex::acquire("Local\\QuiTwinRuntime")?;
    platform::kill_process(install.channel.executable_name());
    let host = discord::latest_host(install)?;
    discord::normalize_resources(&host)?;

    let equicord_dir = platform::roaming_app_data()?.join("Equicord");
    let equicord = equicord_dir.join("equicord.asar");
    fs::create_dir_all(&equicord_dir)?;
    network::download_equicord(&equicord, progress)?;
    progress.set_line("Building the update-safe Discord runtime…");
    shadow::prepare(&host, &equicord)?;
    logging::write(&format!("runtime ready for Discord {}", host.version));
    Ok(())
}

fn launch(install: &Install, args: &[OsString], wait: bool) -> Result<()> {
    let _mutex = platform::InstanceMutex::acquire("Local\\QuiTwinRuntime")?;
    let host = discord::latest_host(install)?;
    discord::normalize_resources(&host)?;
    let equicord = platform::roaming_app_data()?
        .join("Equicord")
        .join("equicord.asar");
    if !equicord.is_file() {
        let mut progress = Progress::new("Repairing Equicord…");
        fs::create_dir_all(
            equicord
                .parent()
                .context("Equicord archive has no parent directory")?,
        )?;
        network::download_equicord(&equicord, &mut progress)?;
    }
    let runtime = shadow::prepare(&host, &equicord)?;
    let executable = runtime.join(install.channel.executable_name());
    logging::write(&format!(
        "launching Discord {} from {}",
        host.version,
        runtime.display()
    ));

    let mut command = Command::new(&executable);
    command
        .args(args)
        .current_dir(&runtime)
        .env("QUITWIN_REAL_EXE", &host.executable)
        .env("QUITWIN_REAL_RESOURCES", &host.resources)
        .env("QUITWIN_STATE_DIRECTORY", install.root.join(".quitwin"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if wait {
        command.status()?;
    } else {
        command.spawn()?;
    }
    Ok(())
}

fn start_uninstall_helper(executable: &Path, install: &Install, silent: bool) -> Result<()> {
    let helper = std::env::temp_dir().join(format!("QuiTwin-uninstall-{}.exe", std::process::id()));
    fs::copy(executable, &helper).context("create uninstall helper")?;
    let file = fs::OpenOptions::new().write(true).open(&helper)?;
    file.sync_all()?;
    drop(file);

    let mut command = Command::new(&helper);
    command
        .arg("--quitwin-uninstall-helper")
        .arg(&install.root)
        .arg(install.channel.root_name())
        .arg(std::process::id().to_string())
        .arg(if silent { "--silent" } else { "--interactive" })
        .creation_flags(platform::CREATE_NO_WINDOW)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

pub fn run_uninstall_helper(executable: &Path, args: &[OsString]) -> Result<()> {
    let [root, channel, parent_id, mode] = args else {
        bail!("invalid uninstall helper arguments");
    };
    let root = PathBuf::from(root);
    let channel = Channel::from_root_name(&channel.to_string_lossy())
        .context("invalid Discord channel for uninstall")?;
    let parent_id = parent_id
        .to_string_lossy()
        .parse::<u32>()
        .context("invalid uninstall parent process")?;
    let recognized = discord::discover()?
        .into_iter()
        .any(|install| install.channel == channel && platform::same_path(&install.root, &root));
    platform::ensure(
        root.is_absolute()
            && root
                .file_name()
                .is_some_and(|name| name.eq_ignore_ascii_case(channel.root_name()))
            && root.join(".quitwin").is_dir()
            && recognized,
        "refusing to uninstall an unrecognized directory",
    )?;

    let _mutex = platform::InstanceMutex::acquire("Local\\QuiTwinInstaller")?;
    platform::kill_process(channel.executable_name());
    platform::wait_for_process(parent_id);
    remove_install_directory(&root)?;
    remove_uninstall_registry(channel)?;
    shortcuts::remove(&Install { root, channel })?;
    platform::delete_after_reboot(executable);

    if mode != OsStr::new("--silent") {
        ui::show_success("Discord and QuiTwin were uninstalled.");
    }
    Ok(())
}

fn remove_install_directory(root: &Path) -> Result<()> {
    for attempt in 0..=100 {
        match fs::remove_dir_all(root) {
            Ok(()) => return Ok(()),
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
            Err(_) if attempt < 100 => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(error) => {
                return Err(error).with_context(|| format!("remove {}", root.display()));
            }
        }
    }
    unreachable!()
}

fn remove_uninstall_registry(channel: Channel) -> Result<()> {
    let current_user = RegKey::predef(HKEY_CURRENT_USER);
    let uninstall = current_user
        .open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_WRITE,
        )
        .context("open uninstall registry")?;
    match uninstall.delete_subkey_all(channel.root_name()) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).context("remove Discord uninstall registry entry"),
    }
}

fn has_arg(args: &[OsString], expected: &str) -> bool {
    args.iter()
        .any(|arg| arg.to_string_lossy().eq_ignore_ascii_case(expected))
}

fn has_option(args: &[OsString], expected: &str) -> bool {
    option_value(args, expected).is_some() || has_arg(args, expected)
}

fn option_value(args: &[OsString], expected: &str) -> Option<OsString> {
    for (index, argument) in args.iter().enumerate() {
        let argument = argument.to_string_lossy();
        if argument.eq_ignore_ascii_case(expected) {
            return args.get(index + 1).cloned();
        }
        if let Some((name, value)) = argument.split_once('=')
            && name.eq_ignore_ascii_case(expected)
        {
            return Some(OsString::from(value));
        }
    }
    None
}
