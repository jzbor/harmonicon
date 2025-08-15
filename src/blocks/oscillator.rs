use std::sync::*;
use std::f32::consts::*;

use crate::blocks::constant::ConstantBlock;
use crate::blocks::{BlockType, SignalBlock, SignalSource};

pub struct OscillatorBlock {
    freq_source: SignalSource,
    phase: f32,
}


impl OscillatorBlock {
    pub fn new(freq_source: SignalSource) -> Self {
        OscillatorBlock { freq_source, phase: 0.0 }
    }

    pub fn update_frequency(&mut self, freq_source: SignalSource) {
        self.freq_source = freq_source;
    }
}


impl SignalBlock for OscillatorBlock {
    fn step(&mut self) {
        self.freq_source.step();

        let freq = self.freq_source.inner().lock().unwrap().get();
        self.phase += 2.0 * PI * freq / (crate::SAMPLE_RATE as f32);

        // limit phase between 0 and 2*PI to avoid inaccuracies
        while self.phase >= 2.0 * PI {
            self.phase -= 2.0 * PI;
        }
        while self.phase < 0.0 {
            self.phase += 2.0 * PI;
        }
    }

    fn get(&self) -> f32 {
        f32::sin(self.phase)
    }

    fn sync_from(&mut self, other: &dyn SignalBlock) {
        if other.block_type() == self.block_type() {
            self.phase = other.sync_value();
        }
    }

    fn block_type(&self) -> super::BlockType {
        BlockType::Oscillator
    }

    fn sync_value(&self) -> f32 {
        self.phase
    }
}


impl Default for OscillatorBlock {
    fn default() -> Self {
        OscillatorBlock {
            freq_source: SignalSource::Anonymous(Arc::new(Mutex::new(ConstantBlock::new(440.0)))),
            phase: 0.0
        }
    }
}
