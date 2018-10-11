extern crate roog;

#[macro_use]
extern crate vst;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin};

fn midi_pitch_to_freq(pitch: u8) -> f64 {
  const A4_PITCH: i8 = 69;
  const A4_FREQ: f64 = 440.0;

  // Midi notes can be 0-127
  ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

struct RoogVST {
  sample_rate: f64,
  time: f64,
  synth: roog::MonoSynth,
}

impl RoogVST {
  fn process_midi_event(&mut self, data: [u8; 3]) {
    match data[0] {
      128 => self.note_off(data[1]),
      144 => self.note_on(data[1]),
      _ => (),
    }
  }

  fn note_on(&mut self, note: u8) {
    let hertz = midi_pitch_to_freq(note);
    self.synth.note_on(hertz)
  }

  fn note_off(&mut self, note: u8) {
    let hertz = midi_pitch_to_freq(note);
    self.synth.note_off(hertz)
  }

  fn time_per_sample(&self) -> f64 {
    1.0 / self.sample_rate
  }
}

impl Default for RoogVST {
  fn default() -> RoogVST {
    RoogVST {
      sample_rate: 44100.0,
      time: 0.0,
      synth: roog::MonoSynth::new(),
    }
  }
}

impl Plugin for RoogVST {
  fn get_info(&self) -> Info {
    Info {
      name: "roog".to_string(),
      vendor: "Walther".to_string(),
      unique_id: 65535,
      category: Category::Synth,
      inputs: 2,
      outputs: 2,
      parameters: 8,
      initial_delay: 0,
      ..Info::default()
    }
  }

  fn get_parameter(&self, index: i32) -> f32 {
    return self.synth.get_parameter(index) as f32;
  }

  fn set_parameter(&mut self, index: i32, val: f32) {
    self.synth.set_parameter(index, val as f64);
  }

  fn get_parameter_text(&self, index: i32) -> String {
    return format!("{:.6}", self.synth.get_parameter(index));
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => "Saw",
      1 => "Sin",
      2 => "Square",
      3 => "Triangle",

      4 => "Attack",
      5 => "Decay",
      6 => "Sustain",
      7 => "Release",
      _ => "",
    }.to_string()
  }

  fn process_events(&mut self, events: &Events) {
    for event in events.events() {
      match event {
        Event::Midi(ev) => self.process_midi_event(ev.data),
        _ => (),
      }
    }
  }

  fn set_sample_rate(&mut self, rate: f32) {
    self.sample_rate = f64::from(rate);
    self.synth.set_sample_rate(f64::from(rate));
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    let samples = buffer.samples();
    let per_sample = self.time_per_sample();

    for (input_buffer, output_buffer) in buffer.zip() {
      let mut time = self.time;

      for (_, output_sample) in input_buffer.iter().zip(output_buffer) {
        let mut mix = self.synth.get_sample(time);

        *output_sample = mix as f32;
        time += per_sample;
      }
    }

    self.time += samples as f64 * per_sample;
  }

  fn can_do(&self, can_do: CanDo) -> Supported {
    match can_do {
      CanDo::ReceiveMidiEvent => Supported::Yes,
      _ => Supported::Maybe,
    }
  }
}

plugin_main!(RoogVST);
