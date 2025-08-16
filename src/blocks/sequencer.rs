use crate::blocks::constant::ConstantBlock;
use crate::blocks::{BlockType, SignalBlock, SignalSource};
use crate::note::Note;

pub struct SequencerBlock {
    sequence: Vec<Note>,
    bpm: SignalSource,
    spacing: SignalSource,
    progress: f32,
}


impl SequencerBlock {
    pub fn update_sequence(&mut self, seq: Vec<Note>) {
        self.sequence = seq
    }

    pub fn update_bpm(&mut self, bpm: SignalSource) {
        self.bpm = bpm
    }

    pub fn update_spacing(&mut self, spacing: SignalSource) {
        self.spacing = spacing
    }
}


impl SignalBlock for SequencerBlock {
    fn step(&mut self) {
        self.progress += self.bpm.get_mono() / (crate::SAMPLE_RATE as f32 * 60.0);
        if self.progress > self.sequence.len() as f32 {
            self.progress -= self.sequence.len() as f32
        }
    }

    fn get_mono(&self) -> f32 {
        if self.spacing.get_mono() >= 0.05 && (self.progress.round()  - self.progress).abs() < self.spacing.get_mono() / 2.0 {
            0.0
        } else {
            self.sequence[self.progress as usize % self.sequence.len()].frequency()
        }
    }

    fn block_type(&self) -> super::BlockType {
        BlockType::Sequencer
    }

    fn sync_from(&mut self, other: &dyn SignalBlock) {
        self.progress = other.sync_value()
    }

    fn sync_value(&self) -> f32 {
        self.progress
    }
}

impl Default for SequencerBlock {
    fn default() -> Self {
        SequencerBlock {
            sequence: Vec::new(),
            bpm: SignalSource::new_anonymous(ConstantBlock::new(120.0)),
            progress: 0.0,
            spacing: SignalSource::new_anonymous(ConstantBlock::new(0.0)),
        }
    }
}
