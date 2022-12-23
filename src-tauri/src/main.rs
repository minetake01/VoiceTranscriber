#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod audio;

use std::sync::Mutex;

use audio::AudioEditor;
use tauri::{
    Menu,
    CustomMenuItem,
    Submenu,
    Manager,
};

use crate::commands::*;

pub struct EditorState(Mutex<AudioEditor>);

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
                    event.window().emit("file_selected", select_file()).unwrap();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
