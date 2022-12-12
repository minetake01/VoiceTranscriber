use std::fs::File;

use symphonia::core::{io::MediaSourceStream, probe::Hint, formats::FormatOptions, meta::MetadataOptions, codecs::DecoderOptions, audio::SampleBuffer};

pub fn decoder(file: File) -> Result<Option<SampleBuffer<f32>>, &'static str> {
    let file = Box::new(file);
                    
    let mss = MediaSourceStream::new(file, Default::default());
    let hint = Hint::new();

    let Ok(probed) = symphonia::default::get_probe().format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default()) else {
        return Err("コーデックの取得に失敗しました。");
    };
    let mut format = probed.format;

    let Some(track) = format.default_track() else {
        return Err("トラックがありません。");
    };

    let Ok(mut decoder) = symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default()) else {
        return Err("デコーダーの作成に失敗しました。");
    };

    let track_id = track.id;

    let mut sample_buf: Option<SampleBuffer<f32>> = None;

    let decoded = loop {
        //次のパケットを取得
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(err) => {
                //ストリームが終点に達したら終了
                if err.to_string() == "end of stream".to_owned() {
                    break Ok(sample_buf);
                }
                break Err("パケットの取得に失敗しました。");
            }
        };

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        //パケットが選択したトラックに属さない場合はスキップ
        if packet.track_id() != track_id {
            continue;
        }

        let Ok(audio_buf) = decoder.decode(&packet) else {
            break Err("デコードに失敗しました。");
        };

        if let Some(buf) = &mut sample_buf {
            buf.copy_interleaved_ref(audio_buf);
        } else {
            let spec = *audio_buf.spec();
            let duration = audio_buf.capacity() as u64;
            sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
        }
    };

    decoded
}