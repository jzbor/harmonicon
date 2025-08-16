use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub struct Note(f32);

impl Note {
    pub const C: Self = Note(261.6256);
    pub const D: Self = Note(293.6648);
    pub const E: Self = Note(329.6276);
    pub const F: Self = Note(349.2282);
    pub const G: Self = Note(391.9954);
    pub const A: Self = Note(440.0000);
    pub const H: Self = Note(493.8833);
    pub const SILENT: Self = Note(0.0);

    pub fn frequency(self) -> f32 {
        self.0
    }
}

impl FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "A" => Ok(Self::A),
            "H" => Ok(Self::H),
            "-" => Ok(Self::SILENT),
            _ => Err(()),
        }
    }
}
