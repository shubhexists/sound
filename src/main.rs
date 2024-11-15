use console::Term;
use rdev::{listen, Event, EventType};
use rodio::{Decoder, OutputStream, Source};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let _ = Term::buffered_stdout();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file: BufReader<File> = BufReader::new(
        File::open(std::env::current_dir().unwrap().join("mech-keyboard-02-102918.mp3")).unwrap(),
    );

    let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
    let channels: u16 = source.channels();
    let sample_rate: u32 = source.sample_rate();
    let samples: Vec<f32> = source.convert_samples().collect();

    listen(move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            match key {
                _ => {
                    let samples: Vec<f32> = samples.clone();
                    let sound_source: rodio::buffer::SamplesBuffer<f32> =
                        rodio::buffer::SamplesBuffer::new(
                            channels,
                            sample_rate,
                            samples,
                        );

                    let _ = stream_handle
                        .play_raw(sound_source.convert_samples())
                        .map_err(|e: rodio::PlayError| eprintln!("Playback error: {}", e));
                }
            }
        }
    })
    .expect("Failed to start global key listener");

    std::thread::park();
}
