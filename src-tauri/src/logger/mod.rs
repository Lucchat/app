pub mod macros;
pub mod syslog;

use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use tauri::Manager;

lazy_static::lazy_static! {
    static ref LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
}

pub fn init_logger(app_handle: &tauri::AppHandle) {
    let dir = app_handle
        .path()
        .app_data_dir()
        .expect("Impossible de récupérer app_data_dir")
        .join("lucchat");

    std::fs::create_dir_all(&dir).expect("Impossible de créer le dossier lucchat");

    let file_path = dir.join("lucchat.log");

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&file_path)
        .expect("Impossible d'ouvrir ou de créer lucchat.log");

    *LOG_FILE.lock().unwrap() = Some(file);
}

pub fn write_log(level: &str, msg: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let line = format!("[{}] [{}] {}", now, level, msg);

    if let Some(file) = LOG_FILE.lock().unwrap().as_mut() {
        let _ = file.write_all(line.as_bytes());
    }

    println!("{}", line);
}
