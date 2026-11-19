# Wavetable synth

A wavetable synthesizer in development for standalone use and VST3 hosts. The first milestone is **Synth Platform Probe**, a small Rust instrument that tests the Truce audio framework and egui editor before work on the synthesis engine begins. Holding any MIDI note plays a quiet pulse at approximately 220 Hz. The single Gain knob is automatable.

## Play the prototype

After installing the native prerequisites below, run the standalone from the project root with computer-keyboard input enabled.

```sh
cargo run --release --locked --bin platform-probe-standalone -- --qwerty-keys
```

Focus the window, hold **A**, turn **Gain** down and up, then release the key. To use it in a DAW, build the VST3 below. The bundler writes it to `target/bundles/Synth Platform Probe.vst3`.

## Build from source

Clone the repository and build on the OS where the instrument will run. Install stable Rust with Cargo and the native prerequisites.

| System | Build setup |
| --- | --- |
| macOS | Apple Silicon, Xcode Command Line Tools and a working audio output. |
| Windows | Native x64 Rust MSVC toolchain, Visual Studio Build Tools with Desktop development with C++ and the Windows SDK. Run the commands in a native developer terminal. |
| Arch Linux | A working ALSA output, X11 or XWayland and a graphics driver with Vulkan support. Build packages include base-devel, pkgconf, alsa-lib, libx11, libxcb, xcb-util-wm, libxcursor, libxkbcommon-x11, libxrandr, fontconfig, freetype2 and Vulkan headers/loader. Keep the JACK provider appropriate to your existing audio configuration if required. |

Run these commands from the directory containing truce.toml. The version pin keeps the bundler and framework together. The first build downloads dependencies.

```sh
cargo install cargo-truce --version 6.3.0 --locked
cargo build --release --workspace --locked
cargo test --release --workspace --locked
cargo truce build --vst3 -p platform-probe --target-cpu baseline
cargo run --release --locked -p platform-probe --bin platform-probe-standalone -- --qwerty-keys
```

To list device names, replace `--qwerty-keys` with `--list-devices` or `--list-midi`. Select a device with `--output "device name"` or `--midi-input "port name"`. See the [Truce installation guide](https://truce.audio/docs/guide/install/).

## Repeat this check on each OS

1. In standalone, hold **A** or send a MIDI note. Listen, turn Gain down and up, and release. Expect one steady tone, silence at minimum gain and silence after release.
2. In REAPER's VST preferences, add the absolute path to this checkout's `target/bundles` directory and re-scan. Insert **VST3i: Synth Platform Probe** on an empty track. Add a MIDI item containing a long note and play it.
3. With track automation in **Touch**, drag Gain down and back up during playback. Switch to **Read**, rewind and play again. Check that the knob and audible level follow the recorded curve.
4. Set a recognizable gain on a second instance. Save the project, close it, reopen it and replay. Confirm that the first instance follows the automation and the second retains its own setting.
5. In the workbook's **Tasks** tab record OS, REAPER version, audio device, standalone result and VST3 result under **Review notes / evidence** for P0-03, P0-04 or P0-05. Include steps to reproduce any failure.

### Verified on Apple Silicon

On macOS 26.6.2, all three Rust tests passed and pluginval 1.0.4 passed the VST3 at strictness 5. The standalone opened the built-in stereo output at 48 kHz.

REAPER 7.86 loaded the instrument and its custom editor. During 48 kHz playback, the note started, muted under gain automation, returned at half its earlier level and stopped on release. A 44.1 kHz stereo WAV bounce confirmed silence in the muted and released sections. After saving and reopening, a second instance recalled its independent 0.625 gain setting. A separate Touch-mode pass captured 52 points from a Gain-knob gesture and restored all 52 on reopen.

The standalone listening check and native Windows and Arch host checks are tracked in the workbook. The [CI workflow](.github/workflows/check.yml) also builds and tests the source on macOS, Windows and Ubuntu.

## Start the engine

After the platform decision in **P0-06**, start **SYN-01** in [the engine crate](crates/synth-core/src/lib.rs). Render a sine into a caller-provided buffer starting at phase zero. At 100 Hz and 48 kHz, one cycle occupies 480 samples. Test samples 0, 120, 240 and 360 against 0, 1, 0 and −1 with a tolerance of 0.0001. Run `cargo test -p synth-core`.

The [task workbook](docs/v1-backlog.xlsx) covers the path from this probe to a V1 release. Each row has dependencies, suggested steps, acceptance criteria and a place for review evidence.

## Routine checks

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```
