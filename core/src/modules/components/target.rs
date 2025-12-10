use hecs::Entity;

use crate::modules::components::Guid;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Target {
    pub e: Entity,
    pub guid: Guid
}


// при десериализации: так как Entity создаются каждый раз разные в hecs, то нужно обновить соответствие таргетов.
//  1. создается мапа pub guid_to_entity: HashMap<Guid, Entity>
//  2. создаются новые entity и в guid_to_entity запоминается соответствие Entity -> Guid
//  3. устанавливается Target(pub Entity) соответствующий уже имеющимся TargetId(pub Guid)
