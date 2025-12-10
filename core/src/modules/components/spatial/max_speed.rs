use serde::{Serialize, Deserialize};

use crate::defines::MinMax;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct MaxSpeed (pub MinMax);
