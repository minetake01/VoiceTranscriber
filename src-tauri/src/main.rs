#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod audio;

use std::thread;

use audio::AudioEditor;
use once_cell::sync::OnceCell;
use tauri::{
    Menu,
    CustomMenuItem,
    Submenu,
    api::dialog::{
        blocking::{FileDialogBuilder, MessageDialogBuilder}, MessageDialogKind,
    },
};

static AUDIO_EDITOR: OnceCell<AudioEditor> = OnceCell::new();

fn main() {
    let open_file = CustomMenuItem::new("open_file".to_owned(), "Open File...");
    let submenu = Submenu::new("File", Menu::new().add_item(open_file));
    let menu = Menu::new()
        .add_submenu(submenu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .menu(menu)
        .on_menu_event(move |event| {
            match event.menu_item_id() {
                "open_file" => {
                    //ファイル選択ダイアログ表示
                    let Some(file_path) = FileDialogBuilder::new().pick_file() else { return; };

                    //ウィンドウにファイルパスを通知
                    let window = event.window();
                    window.emit("open_file", &file_path).unwrap();

                    //デコード
                    thread::spawn(|| {
                        let decoded = match AudioEditor::decode(file_path) {
                            Ok(audio_editor) => audio_editor,
                            Err(err) => {
                                MessageDialogBuilder::new("デコーダーエラー", err).kind(MessageDialogKind::Error).show();
                                return;
                            }
                        };
                        AUDIO_EDITOR.set(decoded).unwrap();
                    });
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
