use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    time::Duration,
};

use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{platform, ui::Progress};

const USER_AGENT: &str = concat!(
    "QuiTwin/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/AriesAlex/QuiTwin)"
);

#[derive(Debug, Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn client() -> Result<Client> {
    Ok(Client::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(Duration::from_secs(20))
        .timeout(Duration::from_secs(15 * 60))
        .build()?)
}

pub fn download_equicord(destination: &Path, progress: &mut Progress) -> Result<()> {
    const API: &str = "https://api.github.com/repos/Equicord/Equicord/releases/latest";
    const FALLBACK: &str =
        "https://github.com/Equicord/Equicord/releases/latest/download/desktop.asar";

    progress.set_line("Finding the latest Equicord build…");
    let client = client()?;
    let url = client
        .get(API)
        .send()
        .and_then(|response| response.error_for_status())
        .and_then(|response| response.json::<Release>())
        .ok()
        .and_then(|release| {
            release
                .assets
                .into_iter()
                .find(|asset| asset.name == "desktop.asar")
                .map(|asset| asset.browser_download_url)
        })
        .unwrap_or_else(|| FALLBACK.to_owned());

    progress.set_line("Downloading Equicord…");
    download(
        &client,
        &url,
        destination,
        1_000_000,
        validate_asar,
        progress,
    )?;
    Ok(())
}

pub fn download_discord_setup(destination: &Path, progress: &mut Progress) -> Result<()> {
    const URL: &str = "https://discord.com/api/downloads/distributions/app/installers/latest?channel=stable&platform=win&arch=x64";
    progress.set_line("Downloading Discord for Windows…");
    download(
        &client()?,
        URL,
        destination,
        10_000_000,
        |path| {
            validate_pe(path)?;
            platform::verify_authenticode(path).context("verify Discord installer signature")
        },
        progress,
    )?;
    Ok(())
}

fn download<F>(
    client: &Client,
    url: &str,
    destination: &Path,
    minimum_size: u64,
    validate: F,
    progress: &mut Progress,
) -> Result<()>
where
    F: FnOnce(&Path) -> Result<()>,
{
    let temporary = destination.with_extension("download");
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut response = client
        .get(url)
        .send()
        .with_context(|| format!("download {url}"))?
        .error_for_status()
        .with_context(|| format!("download {url}"))?;
    let total = response.content_length();
    let mut output =
        File::create(&temporary).with_context(|| format!("create {}", temporary.display()))?;
    let mut buffer = vec![0_u8; 128 * 1024];
    let mut downloaded = 0_u64;
    loop {
        let count = response.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        output.write_all(&buffer[..count])?;
        downloaded += count as u64;
        progress.set_download(downloaded, total);
    }
    output.sync_all()?;
    drop(output);
    if let Some(total) = total
        && total != downloaded
    {
        bail!("incomplete download: expected {total} bytes, received {downloaded}");
    }
    if downloaded < minimum_size {
        bail!("download is unexpectedly small ({downloaded} bytes)");
    }
    if let Err(error) = validate(&temporary) {
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }

    platform::publish_file(&temporary, destination)?;
    Ok(())
}

fn validate_asar(path: &Path) -> Result<()> {
    let mut header = [0_u8; 24];
    File::open(path)?.read_exact(&mut header)?;
    if header[0..4] != [4, 0, 0, 0] || &header[16..24] != b"{\"files\"" {
        bail!("downloaded Equicord archive is not a valid ASAR");
    }
    Ok(())
}

fn validate_pe(path: &Path) -> Result<()> {
    let mut magic = [0_u8; 2];
    File::open(path)?.read_exact(&mut magic)?;
    if magic != *b"MZ" {
        bail!("downloaded Discord installer is not a Windows executable");
    }
    Ok(())
}
