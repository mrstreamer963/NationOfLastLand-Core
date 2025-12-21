use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Interaction {
    pub name: String,
    #[serde(default)]
    pub range: Option<f32>,
    #[serde(flatten)]
    pub effects: HashMap<String, f64>,
}

pub type InteractionDescriptions = HashMap<String, Interaction>;

pub fn deserialize_interactions<'de, D>(deserializer: D) -> Result<Option<InteractionDescriptions>, D::Error>
where
    D: Deserializer<'de>,
{
    let interactions: Vec<Interaction> = Vec::deserialize(deserializer)?;
    if interactions.is_empty() {
        Ok(None)
    } else {
        let mut map = HashMap::new();
        for interaction in interactions {
            map.insert(interaction.name.clone(), interaction);
        }
        Ok(Some(map))
    }
}
