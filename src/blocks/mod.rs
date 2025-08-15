use std::sync::{Arc, Mutex, Weak};

pub mod constant;
pub mod oscillator;
pub mod amplifier;

pub trait SignalBlock : Send {
    fn step(&mut self);
    fn get(&self) -> f32;
}

pub enum SignalSource {
    Anonymous(Arc<Mutex<dyn SignalBlock>>),
    Named(String, Weak<Mutex<dyn SignalBlock>>),
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
