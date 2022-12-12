#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod decoder;

use std::fs::File;

use decoder::decoder;
use tauri::{
    Menu,
    CustomMenuItem,
    Submenu,
    api::dialog::{
        blocking::{
            FileDialogBuilder,
            MessageDialogBuilder
        },
        MessageDialogKind
    },
};

fn main() {
    let open_file = CustomMenuItem::new("open_file".to_owned(), "Open File...");
    let submenu = Submenu::new("File", Menu::new().add_item(open_file));
    let menu = Menu::new()
        .add_submenu(submenu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "open_file" => {
                    //ファイル選択ダイアログ表示
                    let Some(file_path) = FileDialogBuilder::new().pick_file() else { return; };

                    //ウィンドウにファイルパスを通知
                    let window = event.window();
                    window.emit("open_file", {
                        &file_path
                    }).unwrap();

                    //デコード
                    let Ok(file) = File::open(file_path) else {
                        MessageDialogBuilder::new("エラー", "ファイルを開けません。").kind(MessageDialogKind::Error).show();
                        return;
                    };
                    
                    let decoded = match decoder(file) {
                        Ok(decoded) => decoded,
                        Err(err) => {
                            MessageDialogBuilder::new("デコーダーエラー", err).kind(MessageDialogKind::Error).show();
                            return;
                        },
                    };
                    let Some(decoded) = decoded else { return; };
                    let samples = decoded.samples();

                    dbg!(samples);
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
