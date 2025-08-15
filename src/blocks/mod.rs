use std::str::FromStr;
use std::sync::{Arc, Mutex, Weak};

use crate::blocks::constant::ConstantBlock;

pub mod constant;
pub mod oscillator;
pub mod amplifier;
pub mod stereo;

pub trait SignalBlock : Send {
    fn step(&mut self);
    fn get_mono(&self) -> f32;
    fn block_type(&self) -> BlockType;

    fn sync_from(&mut self, other: &dyn SignalBlock) {}

    fn sync_value(&self) -> f32 {
        0.0
    }

    fn get_left(&self) -> f32 {
        self.get_mono()
    }

    fn get_right(&self) -> f32 {
        self.get_mono()
    }
}

pub enum SignalSource {
    Anonymous(Arc<Mutex<dyn SignalBlock>>),
    Named(String, Weak<Mutex<dyn SignalBlock>>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockType {
    Constant,
    Oscillator,
    Amplifier,
    Stereo,
}


impl SignalSource {
    pub fn inner(&self) -> Arc<Mutex<dyn SignalBlock>> {
        use SignalSource::*;
        match self {
            Anonymous(sb) => sb.clone(),
            Named(_, weak) => weak.upgrade().unwrap(),
        }
    }

    pub fn step(&self) {
        use SignalSource::*;
        match self {
            Anonymous(sb) => sb.lock().unwrap().step(),
            Named(_, _) => (),
        }
    }

    pub fn get_mono(&self) -> f32 {
        self.inner().lock().unwrap().get_mono()
    }

    pub fn get_left(&self) -> f32 {
        self.inner().lock().unwrap().get_left()
    }

    pub fn get_right(&self) -> f32 {
        self.inner().lock().unwrap().get_right()
    }
}


impl FromStr for BlockType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BlockType::*;
        match s {
            "constant" | "const" => Ok(Constant),
            "oscillator" | "osc" => Ok(Oscillator),
            "amplifier" | "amp" => Ok(Amplifier),
            "stereo" => Ok(Stereo),
            _ => Err(()),
        }
    }
}

impl Default for SignalSource {
    fn default() -> Self {
        SignalSource::Anonymous(Arc::new(Mutex::new(ConstantBlock::default())))
    }
}
