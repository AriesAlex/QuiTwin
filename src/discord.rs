use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use semver::Version;
use winreg::{RegKey, enums::HKEY_CURRENT_USER};

use crate::platform;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Channel {
    Stable,
    Ptb,
    Canary,
}

impl Channel {
    pub fn from_root_name(value: &str) -> Option<Self> {
        [Self::Stable, Self::Ptb, Self::Canary]
            .into_iter()
            .find(|channel| channel.root_name().eq_ignore_ascii_case(value))
    }

    pub fn root_name(self) -> &'static str {
        match self {
            Self::Stable => "Discord",
            Self::Ptb => "DiscordPTB",
            Self::Canary => "DiscordCanary",
        }
    }

    pub fn executable_name(self) -> &'static str {
        match self {
            Self::Stable => "Discord.exe",
            Self::Ptb => "DiscordPTB.exe",
            Self::Canary => "DiscordCanary.exe",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Stable => "Discord",
            Self::Ptb => "Discord PTB",
            Self::Canary => "Discord Canary",
        }
    }
}

pub fn install_at_root(root: &Path) -> Option<Install> {
    let name = root.file_name()?.to_string_lossy();
    let channel = Channel::from_root_name(&name)?;
    if root.is_dir() && (root.join("installer.db").is_file() || root.join("packages").is_dir()) {
        Some(Install {
            root: root.to_path_buf(),
            channel,
        })
    } else {
        None
    }
}

#[derive(Clone, Debug)]
pub struct Install {
    pub root: PathBuf,
    pub channel: Channel,
}

#[derive(Clone, Debug)]
pub struct Host {
    pub install: Install,
    pub version: Version,
    pub directory: PathBuf,
    pub executable: PathBuf,
    pub resources: PathBuf,
}

pub fn discover() -> Result<Vec<Install>> {
    let mut installs = Vec::new();
    for channel in [Channel::Stable, Channel::Ptb, Channel::Canary] {
        let default = platform::local_app_data()?.join(channel.root_name());
        push_if_valid(&mut installs, default, channel);
        if let Some(registry) = registry_install_location(channel) {
            push_if_valid(&mut installs, registry, channel);
        }
    }
    installs.dedup_by(|left, right| platform::same_path(&left.root, &right.root));
    Ok(installs)
}

fn registry_install_location(channel: Channel) -> Option<PathBuf> {
    let suffix = match channel {
        Channel::Stable => "Discord",
        Channel::Ptb => "DiscordPTB",
        Channel::Canary => "DiscordCanary",
    };
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(format!(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{suffix}"
        ))
        .ok()?;
    key.get_value::<String, _>("InstallLocation")
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn push_if_valid(installs: &mut Vec<Install>, root: PathBuf, channel: Channel) {
    if root.is_dir()
        && (root.join("Update.exe").is_file()
            || fs::read_dir(&root)
                .ok()
                .into_iter()
                .flatten()
                .flatten()
                .any(|entry| entry.file_name().to_string_lossy().starts_with("app-")))
    {
        installs.push(Install { root, channel });
    }
}

pub fn choose(mut installs: Vec<Install>) -> Option<Install> {
    installs.sort_by_key(|install| match install.channel {
        Channel::Stable => 0,
        Channel::Ptb => 1,
        Channel::Canary => 2,
    });
    installs.into_iter().next()
}

pub fn latest_host(install: &Install) -> Result<Host> {
    let mut hosts = Vec::new();
    for entry in
        fs::read_dir(&install.root).with_context(|| format!("read {}", install.root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(version) = name.strip_prefix("app-") else {
            continue;
        };
        let Ok(version) = Version::parse(version) else {
            continue;
        };
        let directory = entry.path();
        let executable = directory.join(install.channel.executable_name());
        let resources = directory.join("resources");
        if !executable.is_file() || !directory.join("updater.node").is_file() || !resources.is_dir()
        {
            continue;
        }
        if !resources.join("app.asar").exists() && !resources.join("_app.asar").is_file() {
            continue;
        }
        hosts.push(Host {
            install: install.clone(),
            version,
            directory,
            executable,
            resources,
        });
    }

    hosts
        .into_iter()
        .max_by(|left, right| left.version.cmp(&right.version))
        .context("Discord has no complete app-* host directory")
}

pub fn normalize_resources(host: &Host) -> Result<()> {
    let app = host.resources.join("app.asar");
    let original = host.resources.join("_app.asar");
    let displaced = host.resources.join("app.asar.quitwin-loader");

    if displaced.exists() {
        if app.is_file() && is_stock_asar(&app)? {
            remove_any(&displaced)?;
        } else if original.is_file() && !app.exists() {
            platform::publish_file(&original, &app)?;
            remove_any(&displaced)?;
        }
    }

    match (app.exists(), original.is_file()) {
        (false, true) => {
            platform::publish_file(&original, &app).context("restore missing stock app.asar")?;
        }
        (true, true) if app.is_dir() || !is_stock_asar(&app)? => {
            if app.is_dir() {
                if displaced.exists() {
                    remove_any(&displaced)?;
                }
                fs::rename(&app, &displaced)
                    .context("move Equicord/Vencord folder loader aside")?;
            }
            platform::publish_file(&original, &app).context("restore stock app.asar")?;
            remove_any(&displaced)?;
        }
        (true, true) => {
            fs::remove_file(&original).context("remove stale _app.asar")?;
        }
        (true, false) if app.is_file() && is_stock_asar(&app)? => {}
        (true, false) => bail!(
            "{} is a loader but _app.asar is missing; Discord must be repaired",
            app.display()
        ),
        (false, false) => bail!("Discord resources contain no app.asar"),
    }

    platform::ensure(app.is_file(), "stock app.asar was not restored")
}

fn is_stock_asar(path: &Path) -> Result<bool> {
    Ok(path.metadata()?.len() >= 512 * 1024)
}

fn remove_any(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn parse_process_start_args(args: &[OsString]) -> Vec<OsString> {
    let mut output = Vec::new();
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].to_string_lossy();
        if (arg.eq_ignore_ascii_case("--process-start-args") || arg.eq_ignore_ascii_case("-a"))
            && let Some(value) = args.get(index + 1)
        {
            output.extend(
                platform::split_command_line(value).unwrap_or_else(|_| vec![value.clone()]),
            );
            index += 2;
            continue;
        }
        if let Some((name, value)) = arg.split_once('=')
            && (name.eq_ignore_ascii_case("--process-start-args")
                || name.eq_ignore_ascii_case("-a"))
        {
            output.extend(
                platform::split_command_line(OsStr::new(value))
                    .unwrap_or_else(|_| vec![OsString::from(value)]),
            );
        }
        index += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use std::{fs::File, str::FromStr};

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn process_start_args_are_extracted() {
        let args = [
            OsString::from("--processStart"),
            OsString::from("Discord.exe"),
            OsString::from("--process-start-args"),
            OsString::from("--start-minimized \"second value\""),
        ];
        assert_eq!(
            parse_process_start_args(&args),
            vec![
                OsString::from("--start-minimized"),
                OsString::from("second value")
            ]
        );
    }

    #[test]
    fn process_start_args_support_equals_syntax() {
        let args = [OsString::from(
            "--process-start-args=--start-minimized \"second value\"",
        )];
        assert_eq!(
            parse_process_start_args(&args),
            vec![
                OsString::from("--start-minimized"),
                OsString::from("second value")
            ]
        );
    }

    #[test]
    fn restores_file_loader_atomically() {
        let fixture = ResourceFixture::new();
        fs::write(fixture.resources.join("app.asar"), b"loader").unwrap();
        fixture.write_original();

        normalize_resources(&fixture.host()).unwrap();

        assert_eq!(
            fs::metadata(fixture.resources.join("app.asar"))
                .unwrap()
                .len(),
            600_000
        );
        assert!(!fixture.resources.join("_app.asar").exists());
    }

    #[test]
    fn restores_folder_loader_and_recovers_missing_app() {
        let fixture = ResourceFixture::new();
        fs::create_dir(fixture.resources.join("app.asar")).unwrap();
        fs::write(fixture.resources.join("app.asar/index.js"), b"loader").unwrap();
        fixture.write_original();

        normalize_resources(&fixture.host()).unwrap();

        assert!(fixture.resources.join("app.asar").is_file());
        assert!(!fixture.resources.join("app.asar.quitwin-loader").exists());
    }

    #[test]
    fn keeps_new_stock_asar_and_discards_stale_original() {
        let fixture = ResourceFixture::new();
        fixture.write_stock();
        fixture.write_original();

        normalize_resources(&fixture.host()).unwrap();

        assert!(fixture.resources.join("app.asar").is_file());
        assert!(!fixture.resources.join("_app.asar").exists());
    }

    struct ResourceFixture {
        _temporary: TempDir,
        root: PathBuf,
        directory: PathBuf,
        resources: PathBuf,
    }

    impl ResourceFixture {
        fn new() -> Self {
            let temporary = TempDir::new().unwrap();
            let root = temporary.path().join("Discord");
            let directory = root.join("app-1.2.3");
            let resources = directory.join("resources");
            fs::create_dir_all(&resources).unwrap();
            Self {
                _temporary: temporary,
                root,
                directory,
                resources,
            }
        }

        fn write_stock(&self) {
            let file = File::create(self.resources.join("app.asar")).unwrap();
            file.set_len(600_000).unwrap();
        }

        fn write_original(&self) {
            let file = File::create(self.resources.join("_app.asar")).unwrap();
            file.set_len(600_000).unwrap();
        }

        fn host(&self) -> Host {
            Host {
                install: Install {
                    root: self.root.clone(),
                    channel: Channel::Stable,
                },
                version: Version::from_str("1.2.3").unwrap(),
                directory: self.directory.clone(),
                executable: self.directory.join("Discord.exe"),
                resources: self.resources.clone(),
            }
        }
    }
}
