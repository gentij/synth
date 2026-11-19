use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use truce::prelude::*;
use truce_egui::{EguiEditor, widgets::param_knob};

// The standalone runs one probe. The DAW entry point never enables this control.
static SHOW_AUDITION: AtomicBool = AtomicBool::new(false);
static AUDITION_PLAYING: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "standalone")]
pub fn enable_standalone_audition() {
    SHOW_AUDITION.store(true, Ordering::Relaxed);
}

#[derive(Params)]
pub struct ProbeParams {
    #[param(id = 1, name = "Gain", range = "linear(0, 1)", default = 0.5)]
    pub gain: FloatParam,
}

/// A fixed test signal lets us check the host before building synth-core.
#[derive(Default)]
pub struct ProbeState {
    held_note: Option<u8>,
    frame: usize,
    period: usize,
}

pub struct PlatformProbe;

impl PluginLogic for PlatformProbe {
    type Params = ProbeParams;
    type DspState = ProbeState;

    fn bus_layouts() -> Vec<BusLayout> {
        BusLayout::stereo_and_mono_output()
    }

    fn reset(state: &mut ProbeState, _: &ProbeParams, config: &AudioConfig) {
        *state = ProbeState {
            period: (config.sample_rate / 220.0).round().max(2.0) as usize,
            ..Default::default()
        };
    }

    fn process(
        state: &mut ProbeState,
        params: &ProbeParams,
        buffer: &mut AudioBuffer,
        events: &EventList,
        _: &mut ProcessContext,
    ) -> ProcessStatus {
        let audition_playing = AUDITION_PLAYING.load(Ordering::Relaxed);
        let mut event_index = 0;
        for frame in 0..buffer.num_samples() {
            while let Some(event) = events.get(event_index) {
                if event.sample_offset as usize > frame {
                    break;
                }
                match event.body {
                    EventBody::NoteOn { note, velocity, .. } if velocity > 0 => {
                        state.held_note = Some(note);
                        state.frame = 0;
                    }
                    EventBody::NoteOff { note, .. } | EventBody::NoteOn { note, .. }
                        if state.held_note == Some(note) =>
                    {
                        state.held_note = None
                    }
                    EventBody::ControlChange { cc: 120 | 123, .. } => state.held_note = None,
                    _ => {}
                }
                event_index += 1;
            }
            let mut sample = 0.0;
            if audition_playing || state.held_note.is_some() {
                let polarity = if state.frame < state.period / 2 {
                    1.0
                } else {
                    -1.0
                };
                sample = 0.08 * polarity * params.gain.read();
                state.frame += 1;
                if state.frame >= state.period {
                    state.frame = 0;
                }
            }
            for channel in 0..buffer.num_output_channels() {
                buffer.output(channel)[frame] = sample;
            }
        }
        ProcessStatus::Normal
    }

    fn editor(params: Arc<ProbeParams>) -> Box<dyn Editor> {
        let standalone = SHOW_AUDITION.load(Ordering::Relaxed);
        let height = if standalone { 190 } else { 150 };
        EguiEditor::new(params, (240, height), move |ui, context| {
            ui.heading("Platform probe");
            if standalone {
                let playing = AUDITION_PLAYING.load(Ordering::Relaxed);
                let label = if playing {
                    "Stop test tone"
                } else {
                    "Play test tone"
                };
                if ui.button(label).clicked() {
                    AUDITION_PLAYING.store(!playing, Ordering::Relaxed);
                }
            } else {
                ui.label("Hold a MIDI note to hear the test tone.");
            }
            param_knob(ui, context, ProbeParamsParamId::Gain, "Gain");
        })
        .into_editor()
    }
}

truce::plugin! {
    logic: PlatformProbe,
    params: ProbeParams,
}
