use synth_core::render_sine;

#[test]
fn renders_one_cycle_from_phase_zero() {
    // At 100 Hz and 48 kHz, one cycle takes 480 samples.
    let mut output = [f32::NAN; 480];
    render_sine(100.0, 48_000.0, &mut output);

    let tolerance = 0.0001;
    for (index, expected) in [(0, 0.0), (120, 1.0), (240, 0.0), (360, -1.0)] {
        let actual = output[index];
        assert!(
            (actual - expected).abs() <= tolerance,
            "sample {index}: expected {expected}, got {actual}"
        );
    }
    assert!(
        output.iter().all(|sample| sample.is_finite()),
        "every sample must be written with a finite value"
    );
}

#[test]
fn accepts_an_empty_output_buffer() {
    render_sine(100.0, 48_000.0, &mut []);
}
