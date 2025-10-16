use crate::formats::pcm::AudioPcm;

pub fn get_example_pcm_sample() -> AudioPcm {
    let mut audio_pcm = AudioPcm {
        sample_rate: 44100,
        content: vec![],
    };

    for t in (0..audio_pcm.sample_rate).map(|x| x as f32 / audio_pcm.sample_rate as f32) {
        let sample = (t * (440.0 + t * 440.0) * 2.0 * std::f32::consts::PI).sin();
        let amplitude = i16::MAX as f32;
        audio_pcm.content.push((sample * amplitude) as i16);
    }
    audio_pcm
}