use hound::{WavReader, WavSpec};
use itertools::Itertools;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

const THREAD: u32 = 4;

#[derive(Debug, Clone, Default)]
pub struct AudioEditor {
    pub file_path: Option<PathBuf>,
    pub spec: Option<WavSpec>,
    pub samples: Vec<i32>,
}
impl AudioEditor {
    pub fn decode(&mut self) -> Result<(), String> {
        let Some(ref file_path) = self.file_path else {
            return Err(format!("ファイルパスが設定されていません。"));
        };
        let reader = match WavReader::open(file_path) {
            Ok(reader) => reader,
            Err(err) => {
                return Err(format!("ファイルを読み込めませんでした。\n{}", err));
            },
        };

        //クリア
        self.samples = vec![];

        //デコード
        let mut spec = reader.spec();
        let samples = {
            let dur = reader.duration();
            let path = Arc::new(file_path.clone());

            let handles = (0..THREAD).map(|i| {
                let path = path.clone();
                let handle: JoinHandle<Result<Vec<i32>, String>> = thread::spawn(move || {
                    let mut reader = WavReader::open(&*path)
                        .map_err(|err| format!("ファイルを読み込めませんでした。\n{}", err))?;
                    let seek_to = dur / THREAD * i;
                    reader.seek(seek_to).unwrap();
                    let take_len = (dur / THREAD + if i == THREAD - 1 {dur % THREAD} else {0}) * spec.channels as u32;
                    let samples = reader.into_samples()
                        .take(take_len as usize)
                        .skip((seek_to % spec.channels as u32) as usize)
                        .step_by(spec.channels as usize)
                        .collect::<Result<Vec<i32>, _>>()
                        .map_err(|err| format!("デコードに失敗しました。\n{}", err))?;
                    Ok(samples)
                });
                handle
            }).collect::<Vec<_>>();
            
            let mut result = Vec::new();
            for handle in handles {
                let v = handle.join().unwrap()?;
                result.extend(v);
            }
            result
        };
        self.samples = samples;
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

    pub fn samples_extraction(&self, start: usize, end: usize, n: f32) -> Vec<i32> {
        let chunk_size = ((end - start) as f32 / n).ceil() as i32;
        let extracted: Vec<i32> = self.samples[start..end].chunks(chunk_size as usize).map(|chunk| {
            chunk.iter().map(|sample| sample.abs()).sum::<i32>() / chunk_size
        })
        .collect();

        extracted
    }

    pub fn split_range(&self, count: Arc<Mutex<i32>>, threshold: i32, talk_dur_sec: f32, mute_dur_sec: f32, extend_sec: f32) -> Option<Vec<Vec<usize>>> {
        let sample_rate = self.spec.unwrap().sample_rate;
        let talk_dur = (sample_rate as f32 * talk_dur_sec) as usize;
        let mute_dur = (sample_rate as f32 * mute_dur_sec) as usize;
        let extend = (sample_rate as f32 * extend_sec) as usize;
        let orig = *count.lock().unwrap();

        let mut ranges: Vec<Vec<usize>> = vec![];
        let mut range: [usize; 2] = [0, 0];
        let mut last_key = false;
        for (key, mut group) in &self.samples.iter().enumerate().group_by(|(_, sample)| sample.abs() >= threshold) {
            if *count.lock().unwrap() != orig {
                return None;
            }
            let index = group.next().unwrap().0;
            let len = group.count() + 1;
            if key && !last_key {
                range[0] = index;
                last_key = true;
            } else if !key && len >= mute_dur && index != 0 {
                range[1] = index - 1;
                last_key = false;

                if range[1] - range[0] >= talk_dur {
                    let start = range[0].saturating_sub(extend);
                    let end = range[1].saturating_add(extend);
                    ranges.push(vec![start, end]);
                    range = [0, 0];
                }
            }
        }
        if range[1].saturating_sub(range[0]) >= talk_dur {
            let start = range[0].saturating_sub(extend);
            let end = range[1].saturating_add(extend);
            ranges.push(vec![start, end]);
        }

        Some(ranges)
    }
}