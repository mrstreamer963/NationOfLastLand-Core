use serde::Deserialize;
use serde_yaml;
use std::error::Error;

#[derive(Deserialize)]
pub struct SlotTagsYaml {
    pub slot_tags: Vec<String>,
}

pub fn load_slot_tags_static(yaml: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let data: SlotTagsYaml = serde_yaml::from_str(yaml)?;
    Ok(data.slot_tags)
}
