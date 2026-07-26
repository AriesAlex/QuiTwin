use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::{discord::Host, logging};

const SCHEMA: u32 = 3;

#[derive(Debug, Deserialize, Serialize)]
struct Marker {
    schema: u32,
    version: String,
    stock_asar_sha256: String,
    host_tree_fingerprint: String,
    equicord_path: String,
}

pub fn prepare(host: &Host, equicord_asar: &Path) -> Result<PathBuf> {
    let runtime = host.install.root.join(".quitwin").join("runtime");
    fs::create_dir_all(&runtime)?;
    cleanup_building(&runtime);

    let expected = Marker {
        schema: SCHEMA,
        version: host.version.to_string(),
        stock_asar_sha256: sha256(&host.resources.join("app.asar"))?,
        host_tree_fingerprint: host_tree_fingerprint(host)?,
        equicord_path: equicord_asar.to_string_lossy().into_owned(),
    };
    let equicord_path_hash = Sha256::digest(expected.equicord_path.as_bytes());
    let cache_key = format!(
        "{}{}{}",
        &expected.stock_asar_sha256[..8],
        &expected.host_tree_fingerprint[..8],
        &hex::encode(equicord_path_hash)[..8]
    );
    let final_directory = runtime.join(format!("app-{}-{cache_key}", host.version));
    if valid_cache(&final_directory, &expected) {
        return Ok(final_directory);
    }
    if final_directory.exists() {
        fs::remove_dir_all(&final_directory)
            .with_context(|| format!("remove invalid cache {}", final_directory.display()))?;
    }

    let building = runtime.join(format!(
        "app-{}-{cache_key}.building-{}",
        host.version,
        std::process::id()
    ));
    if building.exists() {
        fs::remove_dir_all(&building)?;
    }
    fs::create_dir_all(&building)?;
    let linked = match build(host, equicord_asar, &building, &expected) {
        Ok(linked) => linked,
        Err(error) => {
            let _ = fs::remove_dir_all(&building);
            return Err(error);
        }
    };
    fs::rename(&building, &final_directory).with_context(|| {
        format!(
            "publish shadow runtime {} -> {}",
            building.display(),
            final_directory.display()
        )
    })?;
    logging::write(&format!(
        "published shadow runtime with {linked} hardlinked files"
    ));
    cleanup_old(&runtime, &final_directory);
    Ok(final_directory)
}

fn build(host: &Host, equicord_asar: &Path, destination: &Path, marker: &Marker) -> Result<u64> {
    let mut linked = 0;
    for entry in WalkDir::new(&host.directory).follow_links(false) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(&host.directory)?;
        if relative == Path::new("resources").join("app.asar")
            || relative == Path::new("resources").join("_app.asar")
        {
            continue;
        }
        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::hard_link(entry.path(), &target).with_context(|| {
                format!(
                    "hardlink {} into update-safe runtime",
                    entry.path().display()
                )
            })?;
            linked += 1;
        }
    }

    let resources = destination.join("resources");
    fs::create_dir_all(resources.join("app"))?;
    fs::hard_link(host.resources.join("app.asar"), resources.join("_app.asar"))
        .context("hardlink stock app.asar into shadow runtime")?;
    fs::write(
        resources.join("app").join("package.json"),
        r#"{"name":"quitwin-loader","main":"index.js"}"#,
    )?;
    fs::write(
        resources.join("app").join("index.js"),
        loader_script(equicord_asar)?,
    )?;

    let marker_path = destination.join(".quitwin.json");
    let mut marker_file = File::create(&marker_path)?;
    serde_json::to_writer_pretty(&mut marker_file, marker)?;
    marker_file.write_all(b"\n")?;
    marker_file.sync_all()?;

    if !destination
        .join(host.install.channel.executable_name())
        .is_file()
    {
        bail!("shadow Discord executable is missing");
    }
    Ok(linked + 1)
}

fn loader_script(equicord_asar: &Path) -> Result<String> {
    let equicord = serde_json::to_string(&equicord_asar.to_string_lossy())?;
    Ok(format!(
        r#""use strict";
const fs = require("node:fs");
const path = require("node:path");
const {{ app }} = require("electron");

const realExe = process.env.QUITWIN_REAL_EXE;
const realResources = process.env.QUITWIN_REAL_RESOURCES;
const stateDirectory = process.env.QUITWIN_STATE_DIRECTORY;
if (!realExe || !realResources || !stateDirectory) throw new Error("QuiTwin launch context is missing");

const originalGetPath = app.getPath.bind(app);
app.getPath = name => name === "exe" ? realExe : originalGetPath(name);
Object.defineProperty(process, "execPath", {{ configurable: true, value: realExe }});
Object.defineProperty(process, "resourcesPath", {{ configurable: true, value: realResources }});

delete process.env.QUITWIN_REAL_EXE;
delete process.env.QUITWIN_REAL_RESOURCES;
delete process.env.QUITWIN_STATE_DIRECTORY;
require({equicord});

const proof = {{
  schema: 1,
  startedAt: new Date().toISOString(),
  realExe: process.execPath,
  realResources: process.resourcesPath,
  equicord: {equicord}
}};
const proofPath = path.join(stateDirectory, "last-launch.json");
const temporaryProofPath = `${{proofPath}}.${{process.pid}}.tmp`;
fs.writeFileSync(temporaryProofPath, JSON.stringify(proof, null, 2) + "\n");
fs.renameSync(temporaryProofPath, proofPath);
"#
    ))
}

fn valid_cache(directory: &Path, expected: &Marker) -> bool {
    let marker = fs::read(directory.join(".quitwin.json"))
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Marker>(&bytes).ok());
    marker.is_some_and(|actual| {
        actual.schema == expected.schema
            && actual.version == expected.version
            && actual.stock_asar_sha256 == expected.stock_asar_sha256
            && actual.host_tree_fingerprint == expected.host_tree_fingerprint
            && actual.equicord_path == expected.equicord_path
            && directory.join("resources/app/index.js").is_file()
            && directory.join("resources/_app.asar").is_file()
    })
}

fn host_tree_fingerprint(host: &Host) -> Result<String> {
    let mut files = WalkDir::new(&host.directory)
        .follow_links(false)
        .into_iter()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    files.sort_by(|left, right| left.path().cmp(right.path()));

    let mut hasher = Sha256::new();
    for entry in files {
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry.path().strip_prefix(&host.directory)?;
        if relative == Path::new("resources").join("app.asar")
            || relative == Path::new("resources").join("_app.asar")
        {
            continue;
        }
        let metadata = entry.metadata()?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        hasher.update(relative.to_string_lossy().replace('\\', "/").as_bytes());
        hasher.update([0]);
        hasher.update(metadata.len().to_le_bytes());
        hasher.update(modified.to_le_bytes());
    }
    Ok(hex::encode(hasher.finalize()))
}

fn sha256(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(hex::encode(hasher.finalize()))
}

fn cleanup_building(runtime: &Path) {
    let Ok(entries) = fs::read_dir(runtime) else {
        return;
    };
    for entry in entries.flatten() {
        if entry.file_name().to_string_lossy().contains(".building-") {
            let _ = fs::remove_dir_all(entry.path());
        }
    }
}

fn cleanup_old(runtime: &Path, keep: &Path) {
    let Ok(entries) = fs::read_dir(runtime) else {
        return;
    };
    for entry in entries.flatten() {
        if entry.path() != keep && entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            let _ = fs::remove_dir_all(entry.path());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use semver::Version;
    use tempfile::TempDir;

    use super::*;
    use crate::discord::{Channel, Install};

    #[test]
    fn shadow_uses_hardlinks_and_rotates_when_host_changes() {
        let temporary = TempDir::new().unwrap();
        let root = temporary.path().join("Discord");
        let host_directory = root.join("app-1.2.3");
        let resources = host_directory.join("resources");
        let module = host_directory.join("modules").join("module.node");
        fs::create_dir_all(module.parent().unwrap()).unwrap();
        fs::create_dir_all(&resources).unwrap();
        fs::write(host_directory.join("Discord.exe"), b"exe").unwrap();
        fs::write(host_directory.join("updater.node"), b"updater").unwrap();
        fs::write(&module, b"before").unwrap();
        fs::write(resources.join("app.asar"), vec![0_u8; 600_000]).unwrap();
        let equicord = temporary.path().join("equicord.asar");
        fs::write(&equicord, b"equicord").unwrap();

        let host = Host {
            install: Install {
                root,
                channel: Channel::Stable,
            },
            version: Version::from_str("1.2.3").unwrap(),
            directory: host_directory,
            executable: temporary.path().join("unused"),
            resources,
        };

        let first = prepare(&host, &equicord).unwrap();
        let shadow_module = first.join("modules").join("module.node");
        fs::write(&module, b"after!").unwrap();
        assert_eq!(fs::read(&shadow_module).unwrap(), b"after!");

        let second = prepare(&host, &equicord).unwrap();
        assert_ne!(first, second);
        assert_eq!(
            fs::read(second.join("modules/module.node")).unwrap(),
            b"after!"
        );
    }
}
