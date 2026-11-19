use platform_probe::{Plugin, ProbeParamsParamId};
use std::time::Duration;
use truce_test::{assertions, driver};

#[test]
fn a_note_sounds_then_stops_on_release() {
    let result = driver!(Plugin)
        .duration(Duration::from_millis(80))
        .script(|s| {
            s.note_on(60, 0.8);
            s.wait_ms(30);
            s.note_off(60);
        })
        .run();
    assertions::assert_nonzero_between(&result, Duration::ZERO, Duration::from_millis(25));
    assertions::assert_silence_after(&result, Duration::from_millis(35));
}

#[test]
fn turning_gain_to_zero_mutes_a_held_note() {
    let result = driver!(Plugin)
        .duration(Duration::from_millis(80))
        .script(|s| {
            s.note_on(60, 0.8);
            s.wait_ms(30);
            s.set_param(ProbeParamsParamId::Gain, 0.0);
        })
        .run();
    assertions::assert_nonzero_between(&result, Duration::ZERO, Duration::from_millis(25));
    assertions::assert_silence_after(&result, Duration::from_millis(35));
}

#[test]
fn framework_recognizes_the_editor_and_recalls_state() {
    truce_test::assert_valid_info::<Plugin>();
    truce_test::assert_editor_size_consistent::<Plugin>();
    truce_test::assert_state_round_trip::<Plugin>();
}
