use rodio::{Sample, Source};

use std::cell::RefCell;
use std::time::Duration;
use std::sync::*;

use crate::HashMap;
use crate::blocks::SignalBlock;

pub struct HarmoniconMixer {
    pub blocks: HashMap<String, Arc<Mutex<dyn SignalBlock>>>,
}

impl HarmoniconMixer {
    pub fn new() -> Self {
        HarmoniconMixer {
            blocks: HashMap::default(),
        }
    }

    pub fn register_block<T: SignalBlock + 'static>(&mut self, name: String, block: T) -> Arc<Mutex<dyn SignalBlock>> {
        let cell = Arc::new(Mutex::new(block));
        self.blocks.insert(name, cell.clone());
        cell
    }

    pub fn get_block(&self, name: &str) -> Option<&Arc<Mutex<dyn SignalBlock>>> {
        self.blocks.get(name)
    }
}



impl Source for HarmoniconMixer {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        1
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        crate::SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for HarmoniconMixer {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        for block in self.blocks.values_mut() {
            block.lock().unwrap().step();
        }

        let val = self.blocks.get("out").unwrap().lock().unwrap().get();
        println!("{val}");

        Some(val)
    }
}
