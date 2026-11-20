//! Run from the workspace root with `cargo run -p synth-core --example render_wav`.
//! The example should write `target/sine.wav`.

use synth_core::render_sine;

fn main() -> hound::Result<()> {
    let sample_rate_hz = 48_000_u32;
    let frequency_hz = 440.0;
    let mut samples = vec![0.0; sample_rate_hz as usize];
    render_sine(frequency_hz, sample_rate_hz as f32, &mut samples);

    // Describe a mono, 16-bit integer WAV with the same sample rate.
    // Create a writer for target/sine.wav, then scale each sample to
    // 20% of full scale and convert it to i16 before writing it.
    // Finalize the file after all samples have been written.

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate_hz,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("target/sine.wav", spec).unwrap();
    let amplitude = i16::MAX as f32;

    for sample in samples {
        writer.write_sample((sample * 0.2 * amplitude) as i16)?;
    }

    writer.finalize()
}
