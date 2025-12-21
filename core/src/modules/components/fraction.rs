use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Fraction {
    Red,
    Blue,
    Neutral,
}

impl Fraction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "RED" => Some(Fraction::Red),
            "BLUE" => Some(Fraction::Blue),
            "NEUTRAL" => Some(Fraction::Neutral),
            _ => None,
        }
    }
}
