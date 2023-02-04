#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod audio;

use std::sync::{Arc, Mutex};

use audio::AudioEditor;
use tauri::Manager;

use crate::commands::*;

pub struct EditorState(Mutex<Option<AudioEditor>>);

pub struct ProcessCount{
    split_audio: Arc<Mutex<i32>>,
}

fn main() {
    tauri::Builder::default()
        .manage(EditorState(Mutex::new(None)))
        .manage(ProcessCount {
            split_audio: Arc::new(Mutex::new(0)),
        })
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_file,
            extract_amplitude_samples,
            split_audio,
            extract_significant_range,
            encode_partial,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
