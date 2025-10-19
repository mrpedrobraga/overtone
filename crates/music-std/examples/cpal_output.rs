use {
    cpal::{
        Sample, SampleRate,
        traits::{DeviceTrait as _, HostTrait, StreamTrait},
    },
    overtone_music_std::formats::pcm::AudioPcm,
    std::time::Duration,
};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Couldn't get device.");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("Couldn't get configs.");
    let supported_config = supported_configs_range
        .next()
        .expect("No supported config?")
        .with_sample_rate(SampleRate(41000));

    let AudioPcm {
        sample_rate: _,
        content,
    } = AudioPcm::example();
    let mut sample_i = 0;

    let stream = device
        .build_output_stream(
            &supported_config.into(),
            move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    sample_i += 1;
                    *sample = Sample::from_sample(
                        content
                            .get(sample_i)
                            .copied()
                            .unwrap_or(Sample::EQUILIBRIUM),
                    )
                }
            },
            |err| {},
            None,
        )
        .unwrap();

    stream.play().unwrap();
    std::thread::sleep(Duration::from_secs(1));
}
