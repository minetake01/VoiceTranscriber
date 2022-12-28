#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod audio;

use std::sync::{Arc, Mutex};

use audio::AudioEditor;
use tauri::{
    Menu,
    CustomMenuItem,
    Submenu,
    Manager,
};

use crate::commands::*;

pub struct EditorState(Mutex<AudioEditor>);
pub struct SplitRangeCount(Arc<Mutex<i32>>);

fn main() {
    let open_file = CustomMenuItem::new("open_file".to_owned(), "Open File...");
    let open_project = CustomMenuItem::new("open_project".to_owned(), "Open Project...");
    let file = Submenu::new(
        "File",
        Menu::new()
            .add_item(open_file)
            .add_item(open_project)
    );
    let menu = Menu::new()
        .add_submenu(file);

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
        ])
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "open_file" => {
                    event.window().emit("file_select", ()).unwrap();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
