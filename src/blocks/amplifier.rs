use std::sync::*;
use std::f32::consts::*;

use crate::blocks::constant::ConstantBlock;
use crate::blocks::{SignalBlock, SignalSource};

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

    fn get(&self) -> f32 {
        let src = self.source.inner().lock().unwrap().get();
        let mult = self.multiplicator.inner().lock().unwrap().get();
        src * mult
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
