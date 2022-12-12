#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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
                    window.emit("open_file", &file_path).unwrap();

                    //デコード
                    let mut reader = match hound::WavReader::open(file_path) {
                        Ok(reader) => reader,
                        Err(err) => {
                            MessageDialogBuilder::new("デコーダーエラー", format!("ファイルを読み込めませんでした。\n{}", err)).kind(MessageDialogKind::Error).show();
                            return;
                        },
                    };
                    let samples: Vec<i32> = match reader.samples::<i32>().collect::<Result<Vec<i32>, _>>() {
                        Ok(samples) => samples,
                        Err(err) => {
                            MessageDialogBuilder::new("デコーダーエラー", format!("デコードに失敗しました。\n{}", err)).kind(MessageDialogKind::Error).show();
                            return;
                        },
                    };
                    
                    //ウィンドウにデコード結果を通知
                    let window = event.window();
                    window.emit("decoded", &samples).unwrap();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
