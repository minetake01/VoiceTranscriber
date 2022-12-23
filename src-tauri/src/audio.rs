use hound::WavSpec;
use itertools::Itertools;

#[derive(Debug, Clone, Default)]
pub struct AudioEditor {
    pub spec: Option<WavSpec>,
    pub samples: Vec<i32>,
}
impl AudioEditor {
    pub fn decode(&mut self, path: String) -> Result<(), String> {
        let reader = match hound::WavReader::open(path) {
            Ok(reader) => reader,
            Err(err) => {
                return Err(format!("ファイルを読み込めませんでした。\n{}", err));
            },
        };

        //クリア
        self.samples = vec![];

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
        let chunk_size = ((end - start) as f32 / n).ceil() as i32;
        let extracted: Vec<i32> = self.samples[start..end].chunks(chunk_size as usize).map(|chunk| {
            chunk.iter().map(|sample| sample.abs()).sum::<i32>() / chunk_size
        })
        .collect();

        extracted
    }

    pub fn split_range(&self, threshold: i32, talk_dur_sec: f32, mute_dur_sec: f32, extend_sec: f32) -> Vec<Vec<usize>> {
        let sample_rate = self.spec.unwrap().sample_rate;
        let talk_dur = (sample_rate as f32 * talk_dur_sec) as usize;
        let mute_dur = (sample_rate as f32 * mute_dur_sec) as usize;
        let extend = (sample_rate as f32 * extend_sec) as usize;

        let mut ranges: Vec<Vec<usize>> = vec![];
        let mut range: [usize; 2] = [0, 0];
        let mut last_key = false;
        for (key, mut group) in &self.samples.iter().enumerate().group_by(|(_, sample)| sample.abs() >= threshold) {
            let index = group.next().unwrap().0;
            let len = group.count() + 1;
            if key && !last_key {
                range[0] = index;
                last_key = true;
            } else if !key && len >= mute_dur && index != 0 {
                range[1] = index - 1;
                last_key = false;

                if range[1] - range[0] >= talk_dur {
                    ranges.push(vec![range[0].saturating_sub(extend), range[1].saturating_add(extend)]);
                    range = [0, 0];
                }
            }
        }
        if range[1].saturating_sub(range[0]) >= talk_dur {
            ranges.push(vec![range[0] - extend, range[1] + extend]);
        }

        ranges
    }
}