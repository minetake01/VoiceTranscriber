use std::path::PathBuf;

use hound::WavSpec;
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct AudioEditor {
    spec: Option<WavSpec>,
    samples: Vec<i32>,
}
impl AudioEditor {
    pub fn decode(&mut self, path: PathBuf) -> Result<(), String> {
        let reader = match hound::WavReader::open(path) {
            Ok(reader) => reader,
            Err(err) => {
                return Err(format!("ファイルを読み込めませんでした。\n{}", err));
            },
        };

        let mut spec = reader.spec();
        let raw_channels = spec.channels;

        //デコード
        let samples = match reader.into_samples().collect::<Result<Vec<i32>, _>>() {
            Ok(samples) => samples,
            Err(err) => {
                return Err(format!("デコードに失敗しました。\n{}", err));
            },
        };

        //1チャンネルに変換(それぞれのチャンネルを比較して一番小さい値を採用)
        self.samples = samples.chunks(raw_channels as usize)
            .map(|chunk| chunk.iter().min().unwrap().to_owned())
            .collect::<Vec<i32>>();
        spec.channels = 1;
        self.spec = Some(spec);

        Ok(())
    }

    //デバッグ用エンコーダー
    pub fn encode (&self) {
        let mut writer = hound::WavWriter::create("output.wav", self.spec.unwrap()).unwrap();
        self.samples.iter().for_each(|sample| {
            writer.write_sample(*sample).unwrap();
        });
        writer.finalize().unwrap();
    }
}