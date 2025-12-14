use crate::defines::MapSize;
use serde::Deserialize;
use serde_yaml;
use std::error::Error;

#[derive(Deserialize)]
struct SetupYaml {
    spatial: SpatialYaml,
    trash_probability_threshold: f32,
    waste_probability_threshold: f32,
}

#[derive(Deserialize)]
struct SpatialYaml {
    threshold: f32,
    map_size: MapSize,
}

#[derive(Clone, Copy)]
pub struct Spatial {
    pub threshold: f32, // Threshold to consider reached, e.g., 1.0 units
    pub map_size: MapSize
}

pub struct Setup {
    pub spatial: Spatial,
    pub trash_probability_threshold: f32,
    pub waste_probability_threshold: f32,
}

pub fn load_setup_static(yaml: &str) -> Result<Setup, Box<dyn Error>> {
    let yaml_data: SetupYaml = serde_yaml::from_str(yaml)?;
    let spatial = Spatial {
        threshold: yaml_data.spatial.threshold,
        map_size: yaml_data.spatial.map_size,
    };
    Ok(Setup {
        spatial,
        trash_probability_threshold: yaml_data.trash_probability_threshold,
        waste_probability_threshold: yaml_data.waste_probability_threshold,
    })
}
