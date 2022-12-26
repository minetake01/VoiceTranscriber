use std::path::PathBuf;

use tauri::{api::dialog::blocking::FileDialogBuilder, State};

use crate::{EditorState, SplitRangeCount};

#[tauri::command]
pub fn select_file() -> Option<PathBuf> {
    FileDialogBuilder::new().add_filter("WAV Audio File (VLC)", &["wav"]).pick_file()
}

#[tauri::command]
pub async fn decode(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let mut audio_editor = state.0.lock().unwrap();
    audio_editor.decode(path)
}

#[tauri::command]
pub async fn samples_extraction(
    state: State<'_, EditorState>,
    start: usize,
    end: i32,
    n: f32
) -> Result<Vec<i32>, ()> {
    let audio_editor = state.0.lock().unwrap().clone();
    let arg_end: usize;
    if end == -1 {
        arg_end = audio_editor.samples.len();
    } else {
        arg_end = end as usize;
    }
    Ok(audio_editor.samples_extraction(start, arg_end, n))
}

#[tauri::command]
pub async fn split_range(
    state: State<'_, EditorState>,
    count: State<'_, SplitRangeCount>,
    threshold: i32,
    talk_dur_sec: f32,
    mute_dur_sec: f32,
    extend_sec: f32
) -> Result<Vec<Vec<usize>>, ()> {
    let audio_editor = state.0.lock().unwrap().clone();
    let count = count.0.clone();
    
    *count.lock().unwrap() += 1;

    let result = audio_editor.split_range(count.clone(), threshold, talk_dur_sec, mute_dur_sec, extend_sec);

    result.ok_or(())
}