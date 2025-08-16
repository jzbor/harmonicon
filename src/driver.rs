use rodio::{Sample, Source};

use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::{fs, sync::*};

use crate::blocks::constant::ConstantBlock;
use crate::error::HarmoniconError;
use crate::{parse, HashMap};
use crate::blocks::SignalBlock;

pub struct HarmoniconDriver {
    blocks: HashMap<String, Arc<Mutex<dyn SignalBlock>>>,
    update_rx: Option<Receiver<Self>>,
    pending: Option<f32>,
    output: Arc<Mutex<dyn SignalBlock>>,
}

impl HarmoniconDriver {
    pub fn new() -> Self {
        HarmoniconDriver {
            blocks: HashMap::default(),
            update_rx: None,
            pending: None,
            output: Arc::new(Mutex::new(ConstantBlock::default()))
        }
    }

    pub fn parse_from_file(file: &Path) -> crate::Result<Self> {
        let content = fs::read_to_string(file)
            .map_err(HarmoniconError::IO)?;
        let stage1 = parse::parse_stage1(&content)?;
        parse::parse_stage2(stage1)
    }

    pub fn set_update_rx(&mut self, rx: Receiver<Self>) {
        self.update_rx = Some(rx);
    }

    pub fn set_output(&mut self, output: Arc<Mutex<dyn SignalBlock>>) {
        self.output = output;
    }


    pub fn register_block<T: SignalBlock + 'static>(&mut self, name: String, block: T) -> Arc<Mutex<dyn SignalBlock>> {
        let cell = Arc::new(Mutex::new(block));
        self.blocks.insert(name, cell.clone());
        cell
    }

    pub fn alias_block(&mut self, name: &str, alias: String) -> Option<&Arc<Mutex<dyn SignalBlock>>> {
        match self.blocks.get(name).cloned() {
            Some(sb) => { self.blocks.insert(alias.clone(), sb); self.blocks.get(&alias) },
            None => None,
        }
    }

    pub fn get_block(&self, name: &str) -> Option<&Arc<Mutex<dyn SignalBlock>>> {
        self.blocks.get(name)
    }

    fn update(&mut self) {
        let new_driver = match self.update_rx.as_mut().and_then(|rx| rx.try_recv().ok()) {
            Some(d) => d,
            None => return,
        };

        let new_blocks = new_driver.blocks;

        for (name, block) in &new_blocks {
            if let Some(old_block) = self.blocks.get(name) {
                block.lock().unwrap().sync_from(&*old_block.lock().unwrap());
            }
        }

        self.blocks = new_blocks;
        self.output = new_driver.output;
    }
}



impl Source for HarmoniconDriver {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        2
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        crate::SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for HarmoniconDriver {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pending.is_some() {
            return self.pending.take()
        }

        self.update();
        for block in self.blocks.values_mut() {
            block.lock().unwrap().step();
        }

        let left = self.output.lock().unwrap().get_left();
        let right = self.output.lock().unwrap().get_right();

        self.pending = Some(right);
        Some(left)
    }
}
