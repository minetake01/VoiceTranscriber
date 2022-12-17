use crate::AUDIO_EDITOR;

#[tauri::command]
pub fn samples_extraction(start: usize, end: i32, n: f32) -> Vec<i32> {
    let audio_editor = AUDIO_EDITOR.lock().unwrap();
    let arg_end: usize;
    if end == -1 {
        arg_end = audio_editor.samples.len();
    } else {
        arg_end = end as usize;
    }
    audio_editor.samples_extraction(start, arg_end, n)
}