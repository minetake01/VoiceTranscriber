use std::path::PathBuf;

use hound::WavSpec;

#[derive(Debug, Default)]
pub struct AudioEditor {
    pub spec: Option<WavSpec>,
    pub samples: Vec<i32>,
}
impl AudioEditor {
    pub fn decode(&mut self, path: PathBuf) -> Result<(), String> {
        let reader = match hound::WavReader::open(path) {
            Ok(reader) => reader,
            Err(err) => {
                return Err(format!("ファイルを読み込めませんでした。\n{}", err));
            },
        };

        let spec = reader.spec();
        self.spec = Some(spec);

        //デコード
        let samples = match reader.into_samples().step_by(spec.channels as usize).collect::<Result<Vec<i32>, _>>() {
            Ok(samples) => samples,
            Err(err) => {
                return Err(format!("デコードに失敗しました。\n{}", err));
            },
        };
        self.samples = samples;

        Ok(())
    }

    //デバッグ用エンコーダー
    pub fn encode(&self) {
        let mut writer = hound::WavWriter::create("output.wav", self.spec.unwrap()).unwrap();
        self.samples.iter().for_each(|sample| {
            writer.write_sample(*sample).unwrap();
        });
        writer.finalize().unwrap();
    }

    pub fn samples_extraction(&self, start: usize, end: usize, n: f32) -> Vec<i32> {
        let chunk_size = (self.samples.len() as f32 / n).ceil() as i32;
        let extracted: Vec<i32> = self.samples[start..end].chunks(chunk_size as usize).map(|chunk| {
            chunk.iter().map(|sample| sample.abs()).sum::<i32>() / chunk_size
        })
        .collect();

        extracted
    }
}