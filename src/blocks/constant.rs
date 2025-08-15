use crate::blocks::SignalBlock;

pub struct ConstantBlock {
    val: f32,
}

impl ConstantBlock {
    pub fn new(val: f32) -> Self {
        ConstantBlock { val }
    }
}


impl SignalBlock for ConstantBlock {
    fn step(&mut self) {
        ()
    }

    fn get(&self) -> f32 {
        self.val
    }
}

impl Default for ConstantBlock {
    fn default() -> Self {
        ConstantBlock { val: 0.0 }
    }
}
