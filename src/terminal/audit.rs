//! src/terminal/audit.rs - write audit to file.

use std::io::Write;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

pub static AUDIT_PATH: Mutex<Option<std::path::PathBuf>> = Mutex::new(None);

pub fn set_path(p: impl Into<std::path::PathBuf>) {
    *AUDIT_PATH.lock().unwrap() = Some(p.into());
}

pub fn get_path() -> std::path::PathBuf {
    AUDIT_PATH
        .lock()
        .unwrap()
        .clone()
        .unwrap_or_else(|| std::path::PathBuf::from("logs/terminal-audit.log"))
}

pub fn append(entry: &serde_json::Value) {
    let _g = LOCK.lock().unwrap();
    let path = get_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let _ = writeln!(f, "{}", entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("audit.log");
        set_path(&p);
        append(&serde_json::json!({"event": "open"}));
        append(&serde_json::json!({"event": "close"}));
        let content = std::fs::read_to_string(&p).unwrap();
        assert_eq!(content.lines().count(), 2);
        assert!(content.contains("\"open\""));
    }
}
