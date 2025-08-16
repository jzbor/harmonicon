use std::sync::*;
use std::f32::consts::*;

use crate::blocks::constant::ConstantBlock;
use crate::blocks::{BlockType, SignalBlock, SignalBlockChildren, SignalSource};

pub struct OscillatorBlock {
    freq_source: SignalSource,
    phase: f32,
    wave: Waveform,
}

#[derive(Copy, Clone, Debug)]
pub enum Waveform {
    Sinus,
    Sawtooth,
    Square,
    Triangle,
}

impl OscillatorBlock {
    pub fn update_frequency(&mut self, freq_source: SignalSource) {
        self.freq_source = freq_source;
    }

    pub fn update_waveform(&mut self, wave: Waveform) {
        self.wave = wave;
    }
}


impl SignalBlock for OscillatorBlock {
    fn step(&mut self) {
        self.freq_source.step();

        let freq = self.freq_source.inner().lock().unwrap().get_mono();
        self.phase += freq / (crate::SAMPLE_RATE as f32);

        // limit phase between 0 and 2*PI to avoid inaccuracies
        while self.phase > 1.0 {
            self.phase -= 1.0;
        }
        while self.phase < 0.0 {
            self.phase += 1.0;
        }
    }

    fn get_mono(&self) -> f32 {
        let fract = self.phase.fract().abs();
        use Waveform::*;
        match self.wave {
            Sinus => f32::sin(self.phase * 2.0 * PI),
            Sawtooth => 1.0 - fract,
            Square => if fract < 0.5 { 1.0 } else { 0.0 }
            Triangle => if fract < 0.5 { 2.0 * fract } else { 1.0 - 2.0 * fract }
        }
    }

    fn sync_from(&mut self, other: &dyn SignalBlock) {
        if other.block_type() == self.block_type() {
            self.phase = other.sync_value();
        }

        self.sync_children_from(other);
    }

    fn block_type(&self) -> super::BlockType {
        BlockType::Oscillator
    }

    fn sync_value(&self) -> f32 {
        self.phase
    }

    fn children(&self) -> SignalBlockChildren {
        let mut children = SignalBlockChildren::new();
        children.push(self.freq_source.inner());
        children
    }
}


impl Default for OscillatorBlock {
    fn default() -> Self {
        OscillatorBlock {
            freq_source: SignalSource::Anonymous(Arc::new(Mutex::new(ConstantBlock::new(440.0)))),
            phase: 0.0,
            wave: Waveform::Sinus,
        }
    }
}
