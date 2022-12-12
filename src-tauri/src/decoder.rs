use std::path::PathBuf;

use hound::WavSpec;

pub struct DecodedData {
    pub spec: WavSpec,
    pub samples: Vec<i32>,
}

pub async fn decoder(file_path: PathBuf) -> Result<DecodedData, String> {
    let mut reader = match hound::WavReader::open(file_path) {
        Ok(reader) => reader,
        Err(err) => {
            return Err(format!("ファイルを読み込めませんでした。\n{}", err));
        },
    };
    let spec = reader.spec();
    let samples = match reader.samples().collect::<Result<Vec<i32>, _>>() {
        Ok(samples) => samples,
        Err(err) => {
            return Err(format!("デコードに失敗しました。\n{}", err));
        },
    };

    return Ok(DecodedData {
        spec,
        samples,
    });
}