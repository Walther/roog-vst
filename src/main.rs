extern crate cpal;
extern crate roog;

use roog::oscillator::saw;
// use oscillator::sin;
// use oscillator::square;
// use oscillator::triangle;

fn main() {
    // CPAL initialization
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device
        .default_output_format()
        .expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    // Oscillator initialization
    let hertz = 440.0;
    let mut time = 0.0; // Should be seconds

    let mut synth = || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        time = sample_clock / sample_rate; // Should be seconds

        // Example of a two-oscillator synth voice, with slight detune
        // TODO: methods for creating such a synth, with mutable params
        let oscillator1 = saw(hertz + 2.0, time);
        let oscillator2 = saw(hertz - 2.0, time);
        let mix = oscillator1 + oscillator2;
        return mix;
    };

    // CPAL example main loop. TODO: understand, write own
    event_loop.run(move |_, data| match data {
        cpal::StreamData::Output {
            buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
        } => {
            for sample in buffer.chunks_mut(format.channels as usize) {
                let value = synth();
                for out in sample.iter_mut() {
                    *out = value;
                }
            }
        }
        _ => (),
    });
}
