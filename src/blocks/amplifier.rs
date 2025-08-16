use std::sync::*;

use crate::blocks::constant::ConstantBlock;
use crate::blocks::{BlockType, SignalBlock, SignalBlockChildren, SignalSource};

pub struct AmplifierBlock {
    sources: Vec<(SignalSource, SignalSource)>,
}


impl AmplifierBlock {
    pub fn update_source(&mut self, n: usize, first: bool, source: SignalSource) {
        while self.sources.len() <= n {
            self.sources.push((SignalSource::default(), SignalSource::new_anonymous(ConstantBlock::new(1.0))));
        }

        if first {
            self.sources[n].0 = source;
        } else {
            self.sources[n].1 = source;
        }
    }
}

impl SignalBlock for AmplifierBlock {
    fn step(&mut self) {
        for (s1, s2) in &mut self.sources {
            s1.step();
            s2.step();
        }
    }

    fn get_mono(&self) -> f32 {
        self.sources.iter()
            .map(|(s1, s2)| s1.get_mono() * s2.get_mono())
            .sum()
    }

    fn get_left(&self) -> f32 {
        self.sources.iter()
            .map(|(s1, s2)| s1.get_left() * s2.get_left())
            .sum()
    }

    fn get_right(&self) -> f32 {
        self.sources.iter()
            .map(|(s1, s2)| s1.get_right() * s2.get_right())
            .sum()
    }

    fn block_type(&self) -> super::BlockType {
        BlockType::Amplifier
    }

    fn sync_from(&mut self, other: &dyn SignalBlock) {
        self.sync_children_from(other);
    }

    fn children(&self) -> SignalBlockChildren {
        let mut children = SignalBlockChildren::new();
        for (s1, s2) in &self.sources {
            children.push(s1.inner());
            children.push(s2.inner());
        }
        children
    }
}

impl Default for AmplifierBlock {
    fn default() -> Self {
        AmplifierBlock {
            sources: Vec::new()
        }
    }
}
