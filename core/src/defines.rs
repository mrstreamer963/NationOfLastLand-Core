use serde::{Serialize, Deserialize, Deserializer};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MapSize {
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct MinMax {
    pub min: f32,
    pub max: f32,
}

impl<'de> Deserialize<'de> for MinMax {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: [f32; 2] = Deserialize::deserialize(deserializer)?;
        Ok(MinMax { min: arr[0], max: arr[1] })
    }
}
