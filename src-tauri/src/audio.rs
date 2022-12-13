use std::path::PathBuf;

use hound::WavSpec;

#[derive(Debug, Default)]
pub struct AudioEditor {
    spec: Option<WavSpec>,
    samples: Vec<i32>,
}
impl AudioEditor {
    pub fn decode(&mut self, path: PathBuf) -> Result<(), String> {
        let mut reader = match hound::WavReader::open(path) {
            Ok(reader) => reader,
            Err(err) => {
                return Err(format!("ファイルを読み込めませんでした。\n{}", err));
            },
        };
        self.spec = Some(reader.spec());
        self.samples = match reader.samples().collect::<Result<Vec<i32>, _>>() {
            Ok(samples) => samples,
            Err(err) => {
                return Err(format!("デコードに失敗しました。\n{}", err));
            },
        };
        Ok(())
    }
}