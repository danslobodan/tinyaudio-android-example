#![cfg(target_os = "android")]
use android_activity::{ AndroidApp, MainEvent, PollEvent };
use itertools::Itertools;
use std::sync::Arc;

use log::LevelFilter;
use log::info;

use std::{ io::Cursor, time::Duration, ffi::CString };

use rustysynth::{ SoundFont, MidiFileSequencer, Synthesizer, SynthesizerSettings, MidiFile };
use tinyaudio::{ OutputDeviceParameters, run_output_device };

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Info));

    let mut device = None;

    let mut quit = false;
    while !quit {
        app.poll_events(Some(std::time::Duration::from_millis(100)), |event| {
            match event {
                PollEvent::Main(main_event) =>
                    match main_event {
                        MainEvent::GainedFocus => {
                            info!("Gained focus!");

                            let asset_manager = app.asset_manager();

                            let mut midi_asset = asset_manager
                                .open(&CString::new("Midi_44.mid").unwrap())
                                .expect("Could not open midi");

                            let mut buffer = midi_asset.get_buffer().unwrap();
                            let midi_file = Arc::new(MidiFile::new(&mut buffer).unwrap());

                            info!("Midi file loaded.");

                            let mut soundfont_file = asset_manager
                                .open(&CString::new("A320U.sf2").unwrap())
                                .expect("Could not open soundfont");

                            let mut buffer = soundfont_file.get_buffer().unwrap();
                            let sound_font = Arc::new(SoundFont::new(&mut buffer).unwrap());

                            info!("Soundfont loaded.");

                            let params = OutputDeviceParameters {
                                channels_count: 2,
                                sample_rate: 44100,
                                channel_sample_count: 4410,
                            };

                            // Buffer for the audio output.
                            let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count];
                            let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count];

                            // Create the MIDI file sequencer.
                            let settings = SynthesizerSettings::new(params.sample_rate as i32);
                            let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
                            let mut sequencer = MidiFileSequencer::new(synthesizer);

                            sequencer.play(&midi_file, true);

                            let dev = run_output_device(params, {
                                move |data| {
                                    sequencer.render(&mut left[..], &mut right[..]);
                                    for (i, value) in left
                                        .iter()
                                        .interleave(right.iter())
                                        .enumerate() {
                                        data[i] = *value;
                                    }
                                }
                            });

                            device = Some(dev);
                        }
                        MainEvent::Destroy => {
                            quit = true;
                        }
                        _ => {}
                    }
                _ => {}
            }
        });
    }
}
