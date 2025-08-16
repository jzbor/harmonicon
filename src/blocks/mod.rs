use std::collections::VecDeque;
use std::iter;
use std::str::FromStr;
use std::sync::{Arc, Mutex, Weak};

use crate::blocks::constant::ConstantBlock;

pub mod constant;
pub mod oscillator;
pub mod amplifier;
pub mod stereo;
pub mod sequencer;

pub trait SignalBlock : Send {
    fn step(&mut self);
    fn get_mono(&self) -> f32;
    fn block_type(&self) -> BlockType;

    fn sync_from(&mut self, _other: &dyn SignalBlock);

    fn sync_value(&self) -> f32 {
        0.0
    }

    fn get_left(&self) -> f32 {
        self.get_mono()
    }

    fn get_right(&self) -> f32 {
        self.get_mono()
    }

    fn children(&self) -> SignalBlockChildren {
        SignalBlockChildren(VecDeque::new())
    }

    fn sync_children_from(&self, other: &dyn SignalBlock) {
        for (child, other_child) in iter::zip(self.children(), other.children()) {
            let mut child = child.lock().unwrap();
            let other_child = other_child.lock().unwrap();
            if child.block_type() == other_child.block_type() {
                child.sync_from(&*other_child);
            }
        }
    }
}

pub enum SignalSource {
    Anonymous(Arc<Mutex<dyn SignalBlock>>),
    Named(Weak<Mutex<dyn SignalBlock>>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlockType {
    Constant,
    Oscillator,
    Amplifier,
    Stereo,
    Sequencer,
}

pub struct SignalBlockChildren(VecDeque<Arc<Mutex<dyn SignalBlock>>>);


impl SignalSource {
    pub fn new_anonymous(sb: impl SignalBlock + 'static) -> Self {
        SignalSource::Anonymous(Arc::new(Mutex::new(sb)))
    }

    pub fn inner(&self) -> Arc<Mutex<dyn SignalBlock>> {
        use SignalSource::*;
        match self {
            Anonymous(sb) => sb.clone(),
            Named(weak) => weak.upgrade().unwrap(),
        }
    }

    pub fn step(&self) {
        use SignalSource::*;
        match self {
            Anonymous(sb) => sb.lock().unwrap().step(),
            Named(_) => (),
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

impl SignalBlockChildren {
    fn new() -> Self {
        SignalBlockChildren(VecDeque::new())
    }

    fn push(&mut self, child: Arc<Mutex<dyn SignalBlock>>) {
        self.0.push_back(child);
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
            "sequencer" | "seq" => Ok(Sequencer),
            _ => Err(()),
        }
    }
}

impl Default for SignalSource {
    fn default() -> Self {
        SignalSource::Anonymous(Arc::new(Mutex::new(ConstantBlock::default())))
    }
}

impl Iterator for SignalBlockChildren {
    type Item = Arc<Mutex<dyn SignalBlock>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}
