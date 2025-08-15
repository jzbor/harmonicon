use crate::blocks::{BlockType, SignalBlock};

pub struct ConstantBlock {
    val: f32,
}

impl ConstantBlock {
    pub fn new(val: f32) -> Self {
        ConstantBlock { val }
    }
}


impl SignalBlock for ConstantBlock {
    fn step(&mut self) {}

    fn get(&self) -> f32 {
        self.val
    }

    fn sync_from(&mut self, other: &dyn SignalBlock) {}

    fn block_type(&self) -> super::BlockType {
        BlockType::Constant
    }
}

impl Default for ConstantBlock {
    fn default() -> Self {
        ConstantBlock { val: 0.0 }
    }
}
