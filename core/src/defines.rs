pub struct MapSize {
    pub width: i32,
    pub height: i32,
}

pub struct MaxMin {
    pub max: f32,
    pub min: f32,
}

impl From<MapSize> for MaxMin {
    fn from(value: MapSize) -> Self {
        MaxMin {
            max: (value.width.max(value.height)) as f32,
            min: (value.width.min(value.height)) as f32,
        }
    }
}
