use hound::{WavReader, WavSpec};
use itertools::Itertools;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::thread;

const THREAD: u32 = 4;

#[derive(Debug, Clone, Default)]
pub struct AudioEditor {
    pub file_path: PathBuf,

    spec: Option<WavSpec>,
    samples: Vec<i32>,
}
impl AudioEditor {
    pub fn decode(&mut self) -> Result<(), String> {
        let reader = WavReader::open(&self.file_path)
            .map_err(|err| format!("ファイルが開けませんでした。\n{}", err))?;

        //クリア
        self.samples = vec![];

        //デコード
        let mut spec = reader.spec();
        let dur = reader.duration();
        let path = Arc::new(self.file_path.clone());

        let handles = (0..THREAD).map(|i| {
            let path = path.clone();
            thread::spawn(move || -> Result<_, String> {
                let mut reader = WavReader::open(&*path)
                    .map_err(|err| format!("ファイルが開けませんでした。\n{}", err))?;
                
                let seek_to = dur / THREAD * i;
                reader.seek(seek_to).unwrap();
                
                let take_len = (dur / THREAD + if i == THREAD - 1 {dur % THREAD} else {0}) * spec.channels as u32;
                let samples = reader.into_samples()
                    .take(take_len as usize)
                    .step_by(spec.channels as usize)
                    .collect::<Result<Vec<i32>, _>>()
                    .map_err(|err| format!("デコードに失敗しました。\n{}", err))?;
                Ok(samples)
            })
        }).collect::<Vec<_>>();
        
        for handle in handles {
            let v = handle.join().unwrap()?;
            self.samples.extend(v);
        }
        spec.channels = 1;
        self.spec = Some(spec);
        
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

    pub fn extract_amplitude_samples(&self, start: usize, end: i32, n: u32) -> Vec<i32> {
        let end = if end == -1 { self.samples.len() } else { end as usize };
        
        let chunk_size = ((end - start) as f32 / n as f32).ceil() as usize;
        let extracted: Vec<i32> = self.samples[start..end]
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().fold(0, |acc, x| acc.max(x.abs())))
            .collect();

        extracted
    }

    pub fn split_audio(&self, process_count: Arc<Mutex<i32>>, threshold: i32, talk_dur_sec: f32, silence_dur_sec: f32, extend_sec: f32) -> Option<Vec<Vec<usize>>> {
        let sample_rate = self.spec.unwrap().sample_rate;
        let talk_dur = (sample_rate as f32 * talk_dur_sec) as usize;
        let silence_dur = (sample_rate as f32 * silence_dur_sec) as usize;
        let extend = (sample_rate as f32 * extend_sec) as usize;
        let orig = *process_count.lock().unwrap();

        let mut ranges: Vec<Vec<usize>> = vec![];
        let mut segment_range: [usize; 2] = [0, 0];
        let mut last_key = false;
        for (key, mut group) in &self.samples.iter().enumerate().group_by(|(_, sample)| sample.abs() >= threshold) {
            if *process_count.lock().unwrap() != orig {
                return None;
            }
            let index = group.next().unwrap().0;
            let len = group.count() + 1;
            if key && !last_key {
                segment_range[0] = index;
                last_key = true;
            } else if !key && len >= silence_dur && index != 0 {
                segment_range[1] = index - 1;
                last_key = false;

                if segment_range[1] - segment_range[0] >= talk_dur {
                    let start = segment_range[0].saturating_sub(extend);
                    let end = segment_range[1].saturating_add(extend);
                    ranges.push(vec![start, end]);
                    segment_range = [0, 0];
                }
            }
        }
        if segment_range[1].saturating_sub(segment_range[0]) >= talk_dur {
            let start = segment_range[0].saturating_sub(extend);
            let end = segment_range[1].saturating_add(extend);
            ranges.push(vec![start, end]);
        }

        Some(ranges)
    }
}