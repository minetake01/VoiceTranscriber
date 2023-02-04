use hound::{WavReader, WavSpec};
use itertools::Itertools;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::thread;

const THREAD: u32 = 4;

#[derive(Debug, Clone)]
pub struct AudioEditor {
    pub file_path: PathBuf,
    spec: WavSpec,
    samples: Vec<i32>,
}
impl AudioEditor {
    pub fn init(file_path: PathBuf) -> Result<Self, String> {
        let reader = WavReader::open(&file_path)
            .map_err(|err| format!("ファイルが開けませんでした。\n{}", err))?;

        //デコード
        let spec = reader.spec();
        let dur = reader.duration();
        let path = Arc::new(file_path.clone());

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

        let mut audio_editor = Self {
            file_path,
            spec: WavSpec { channels: 1, ..spec },
            samples: vec![],
        };
        
        for handle in handles {
            let v = handle.join().unwrap()?;
            audio_editor.samples.extend(v);
        }
        
        Ok(audio_editor)
    }

    pub fn encode(&self, path: &PathBuf, start: usize, end: usize) {
        let mut writer = hound::WavWriter::create(path, self.spec).unwrap();
        self.samples[start..end].iter().for_each(|sample| {
            writer.write_sample(*sample).unwrap();
        });
        writer.finalize().unwrap();
    }

    pub fn extract_amplitude_samples(&self, start: usize, end: i32, n: f32) -> Vec<i32> {
        let end = if end == -1 { self.samples.len() } else { end as usize };
        
        let chunk_size = ((end - start) as f32 / n).ceil() as usize;
        let extracted: Vec<i32> = self.samples[start..end]
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().fold(0, |acc, x| acc.max(x.abs())))
            .collect();

        extracted
    }

    pub fn split_audio(&self, process_count: Arc<Mutex<i32>>, threshold: i32, talk_dur_sec: f32, silence_dur_sec: f32, extend_sec: f32) -> Option<Vec<Vec<usize>>> {
        let sample_rate = self.spec.sample_rate;
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