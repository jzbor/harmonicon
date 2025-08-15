use std::sync::*;
use std::f32::consts::*;

use crate::blocks::constant::ConstantBlock;
use crate::blocks::{BlockType, SignalBlock, SignalSource};

pub struct AmplifierBlock {
    source: SignalSource,
    multiplicator: SignalSource,
}


impl AmplifierBlock {
    pub fn update_source(&mut self, source: SignalSource) {
        self.source = source
    }

    pub fn update_multiplicator(&mut self, mult: SignalSource) {
        self.multiplicator = mult
    }
}

impl SignalBlock for AmplifierBlock {
    fn step(&mut self) {
        self.source.step();
        self.multiplicator.step();
    }

    fn get_mono(&self) -> f32 {
        let src = self.source.inner().lock().unwrap().get_mono();
        let mult = self.multiplicator.inner().lock().unwrap().get_mono();
        src * mult
    }

    fn get_left(&self) -> f32 {
        let src_left = self.source.inner().lock().unwrap().get_left();
        let mult = self.multiplicator.inner().lock().unwrap().get_mono();
        src_left * mult
    }

    fn get_right(&self) -> f32 {
        let src_right = self.source.inner().lock().unwrap().get_right();
        let mult = self.multiplicator.inner().lock().unwrap().get_mono();
        src_right * mult
    }

    fn block_type(&self) -> super::BlockType {
        BlockType::Amplifier
    }
}

impl Default for AmplifierBlock {
    fn default() -> Self {
        AmplifierBlock {
            source: SignalSource::Anonymous(Arc::new(Mutex::new(ConstantBlock::new(0.0)))),
            multiplicator: SignalSource::Anonymous(Arc::new(Mutex::new(ConstantBlock::new(1.0)))),
        }
    }
}
