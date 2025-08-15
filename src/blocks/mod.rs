use std::str::FromStr;
use std::sync::{Arc, Mutex, Weak};

pub mod constant;
pub mod oscillator;
pub mod amplifier;

pub trait SignalBlock : Send {
    fn step(&mut self);
    fn get(&self) -> f32;
    fn sync_from(&mut self, other: &dyn SignalBlock);
    fn block_type(&self) -> BlockType;

    fn sync_value(&self) -> f32 {
        0.0
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
}


impl FromStr for BlockType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BlockType::*;
        match s {
            "constant" | "const" => Ok(Constant),
            "oscillator" | "osc" => Ok(Oscillator),
            "amplifier" | "amp" => Ok(Amplifier),
            _ => Err(()),
        }
    }
}
