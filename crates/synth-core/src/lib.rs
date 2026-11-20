//! The shared synthesis engine.

use std::f32::consts::TAU;

/// Fills `output` with sine-wave samples, starting at phase zero on each call.
///
/// Frequency is in hertz and sample rate is in samples per second. The caller
/// supplies a positive sample rate and may pass an empty output slice.
pub fn render_sine(frequency_hz: f32, sample_rate_hz: f32, output: &mut [f32]) {
    let phase_step = frequency_hz / sample_rate_hz;

    for (sample_index, sample) in output.iter_mut().enumerate() {
        let phase_step_cycles = (sample_index as f32) * phase_step;
        let angle_radians = phase_step_cycles * TAU;
        *sample = angle_radians.sin();
    }
}
