use std::sync::*;
use std::f32::consts::*;

use crate::blocks::constant::ConstantBlock;
use crate::blocks::{BlockType, SignalBlock, SignalSource};

#[derive(Default)]
pub struct StereoBlock {
    left: SignalSource,
    right: SignalSource,
    shift: SignalSource,
}


impl StereoBlock {
    pub fn update_left(&mut self, left: SignalSource) {
        self.left = left;
    }

    pub fn update_right(&mut self, right: SignalSource) {
        self.right = right;
    }

    pub fn update_shift(&mut self, shift: SignalSource) {
        self.shift = shift;
    }
}


impl SignalBlock for StereoBlock {
    fn step(&mut self) {
        self.left.step();
        self.right.step();
        self.shift.step();
    }

    fn get_mono(&self) -> f32 {
        self.left.get_mono() + self.right.get_mono() / 2.0
    }

    fn get_left(&self) -> f32 {
        let shift = (self.shift.get_mono() + 1.0) / 2.0;
        self.left.get_left() * (1.0 - shift)
    }

    fn get_right(&self) -> f32 {
        let shift = (self.shift.get_mono() + 1.0) / 2.0;
        self.left.get_right() * shift
    }

    fn block_type(&self) -> super::BlockType {
        BlockType::Stereo
    }
}
