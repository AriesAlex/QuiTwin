use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

static LOG_PATH: OnceLock<Mutex<PathBuf>> = OnceLock::new();

pub fn init(path: PathBuf) {
    let storage = LOG_PATH.get_or_init(|| Mutex::new(path.clone()));
    if let Ok(mut current) = storage.lock() {
        *current = path;
    }
}

pub fn path() -> PathBuf {
    LOG_PATH
        .get()
        .and_then(|path| path.lock().ok().map(|path| path.clone()))
        .unwrap_or_else(|| std::env::temp_dir().join("QuiTwin.log"))
}

pub fn write(message: &str) {
    let path = path();
    let _ = append(&path, message);
}

fn append(path: &Path, message: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let milliseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "[{milliseconds}] {message}")
}
