pub struct Spatial {
    pub threshold: f32 // Threshold to consider reached, e.g., 1.0 units
}

pub struct Setup {
    pub spatial: Spatial
}

pub fn new() -> Setup {
    let s = Spatial {
        threshold: 0.1
    };
    Setup { spatial: s}
}