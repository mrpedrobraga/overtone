unsafe extern "C" {
    pub fn start(mode: f32) -> *mut f32;
}

#[test]
fn test_start_function() {
    let length = 41000;
    let audio_clip = unsafe { start(0.0) };
    let audio_clip = unsafe { std::slice::from_raw_parts(audio_clip, length as usize) };

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 41000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("./test.wav", spec).expect("Failed to write.");

    for sample in audio_clip.iter().copied() {
        let sample16: i16 = (sample * (i16::MAX as f32)) as i16;
        writer.write_sample(sample16).unwrap();
    }

    writer
        .finalize()
        .expect("Error finalising to write the wav file.");
}
