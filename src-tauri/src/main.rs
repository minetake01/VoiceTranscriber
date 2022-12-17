#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod audio;

use std::sync::Mutex;

use audio::AudioEditor;
use once_cell::sync::Lazy;
use tauri::{
    Menu,
    CustomMenuItem,
    Submenu,
    api::dialog::{
        blocking::{FileDialogBuilder, MessageDialogBuilder}, MessageDialogKind,
    },
};

use crate::commands::samples_extraction;

static AUDIO_EDITOR: Lazy<Mutex<AudioEditor>> = Lazy::new(|| Mutex::new(AudioEditor::default()));

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
        .invoke_handler(tauri::generate_handler![
            samples_extraction
        ])
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "open_file" => {
                    //ファイル選択ダイアログ表示
                    let Some(path) = FileDialogBuilder::new().add_filter("WAV Audio File (VLC)", &["wav"]).pick_file() else { return; };

                    let mut audio_editor = AUDIO_EDITOR.lock().unwrap();

                    if let Err(err) = audio_editor.decode(path) {
                        MessageDialogBuilder::new("デコーダーエラー", format!("{}", err)).kind(MessageDialogKind::Error).show();
                        return;
                    }
                    
                    event.window().emit("decoded", audio_editor.spec.unwrap().sample_rate).unwrap();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
