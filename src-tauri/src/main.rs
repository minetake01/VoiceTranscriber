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

pub struct EditorState(Mutex<AudioEditor>);
pub struct SplitRangeCount(Arc<Mutex<i32>>);

fn main() {
    tauri::Builder::default()
        .manage(EditorState(Default::default()))
        .manage(SplitRangeCount(Arc::new(Mutex::new(0))))
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            select_file,
            decode,
            samples_extraction,
            split_range,
            get_samples,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
