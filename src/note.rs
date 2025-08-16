use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub struct Note(f32);

impl Note {
    pub const SILENT: Self = Note(0.0);

    pub fn from_key(n: i32) -> Self {
        Note(440.0 * 2.0_f32.powf((n as f32 - 48.0) / 12.0))
    }

    pub fn frequency(self) -> f32 {
        self.0
    }
}

impl FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            return Ok(Self::SILENT);
        }

        let key_str: String = s.chars().take_while(|c| !c.is_numeric()).collect();
        let octave_str: String = s.chars().skip_while(|c| !c.is_numeric()).collect();

        let key_offset = match key_str.as_str() {
            "C" => 1,
            "C#" | "Db" => 2,
            "D" => 3,
            "D#" | "Eb" => 4,
            "E" => 5,
            "F" => 6,
            "F#" | "Gb" => 7,
            "G" => 8,
            "G#" | "Ab" => 9,
            "A" => 10,
            "A#" | "Bb" => 11,
            "B" => 12,
            _ => return Err(()),
        };

        let octave = if !octave_str.is_empty() {
            octave_str.parse().map_err(|_| ())?
        } else {
            4
        };

        Ok(Self::from_key(octave * 12 + key_offset - 10))
    }
}
