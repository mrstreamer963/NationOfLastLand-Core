use crate::defines::MapSize;

pub struct Spatial {
    pub threshold: f32, // Threshold to consider reached, e.g., 1.0 units
    pub map_size: MapSize
}

pub struct Setup {
    pub spatial: Spatial
}

pub fn new() -> Setup {
    let s = Spatial {
        threshold: 0.1,
        map_size: MapSize { width: 10, height: 10 }
    };
    Setup { spatial: s}
}
