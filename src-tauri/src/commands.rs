use std::{path::PathBuf, env};

use tauri::{api::dialog::blocking::FileDialogBuilder, State};

use crate::{EditorState, ProcessCount, audio::AudioEditor};

#[tauri::command]
pub async fn open_file(editor_state: State<'_, EditorState>) -> Result<(), String> {
    let file_path = FileDialogBuilder::new().add_filter("WAV Audio File (VLC)", &["wav"]).pick_file().ok_or("")?;
    let audio_editor = AudioEditor::init(file_path)?;
    *editor_state.0.lock().unwrap() = Some(audio_editor);
    Ok(())
}

#[tauri::command]
pub async fn extract_amplitude_samples(
    editor_state: State<'_, EditorState>,
    start: usize,
    end: i32,
    n: f32,
) -> Result<Vec<i32>, ()> {
    let audio_editor = editor_state.0.lock().unwrap().clone().unwrap();
    let result = audio_editor.extract_amplitude_samples(start, end, n);
    Ok(result)
}

#[tauri::command]
pub async fn split_audio(
    editor_state: State<'_, EditorState>,
    process_count: State<'_, ProcessCount>,
    threshold: i32,
    talk_dur_sec: f32,
    silence_dur_sec: f32,
    extend_sec: f32
) -> Result<Vec<Vec<usize>>, ()> {
    let count = process_count.split_audio.clone();
    *count.lock().unwrap() += 1;

    let audio_editor = editor_state.0.lock().unwrap().clone().unwrap();
    let result = audio_editor.split_audio(count.clone(), threshold, talk_dur_sec, silence_dur_sec, extend_sec);
    result.ok_or(())
}

#[tauri::command]
pub async fn extract_significant_range() -> Result<Vec<Vec<usize>>, String> {
    // TODO: 処理を実装
    Ok(vec![vec![0, 10000]; 4])
}

#[tauri::command]
pub async fn encode_partial(
    editor_state: State<'_, EditorState>,
    start: usize,
    end: usize
) -> Result<PathBuf, String> {
    let audio_editor = editor_state.0.lock().unwrap().clone().unwrap();
    let path = env::temp_dir()
        .join(format!("{}-{}-{}.wav", audio_editor.file_path.file_stem().unwrap().to_str().unwrap(), start, end));
    if !path.is_file() {
        audio_editor.encode(&path, start, end);
    }
    Ok(path)
}