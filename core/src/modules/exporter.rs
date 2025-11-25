use crate::modules::components::{DamageType, EntityType, Health, MaxSpeed, Pos, Reputation, Rot, TargetId, TargetPos, UnitState, Velocity};
use crate::modules::markers::{Alert, Vehicle};

use crate::modules::state::State;
use hecs::{serialize::row::*, World, EntityRef};
use serde::{Serialize, ser::SerializeMap};

struct ExportData {
    units: Vec<serde_json::Value>,
    state: State,
}

impl Serialize for ExportData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("units", &self.units)?;
        map.serialize_entry("state", &self.state)?;
        map.end()
    }
}

#[derive(Serialize)]
enum ComponentId {
    Pos,
    EntityType,
    Health,
    Velocity,
    Rot,
    MaxSpeed,
    Target,
    Reputation,
    UnitState,
    TargetId,
    DamageType,
    Alert,
    Vehicle,
}

struct Context;

impl SerializeContext for Context {
    fn serialize_entity<S>(
        &mut self,
        entity: EntityRef<'_>,
        mut map: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: SerializeMap,
    {
        try_serialize::<Pos, _, _>(&entity, &ComponentId::Pos, &mut map)?;
        try_serialize::<EntityType, _, _>(&entity, &ComponentId::EntityType, &mut map)?;
        try_serialize::<Health, _, _>(&entity, &ComponentId::Health, &mut map)?;
        try_serialize::<Velocity, _, _>(&entity, &ComponentId::Velocity, &mut map)?;
        try_serialize::<Rot, _, _>(&entity, &ComponentId::Rot, &mut map)?;
        try_serialize::<MaxSpeed, _, _>(&entity, &ComponentId::MaxSpeed, &mut map)?;
        try_serialize::<TargetPos, _, _>(&entity, &ComponentId::Target, &mut map)?;
        try_serialize::<Reputation, _, _>(&entity, &ComponentId::Reputation, &mut map)?;
        try_serialize::<UnitState, _, _>(&entity, &ComponentId::UnitState, &mut map)?;
        try_serialize::<TargetId, _, _>(&entity, &ComponentId::TargetId, &mut map)?;
        try_serialize::<DamageType, _, _>(&entity, &ComponentId::DamageType, &mut map)?;
        map.serialize_entry(&ComponentId::Alert, &entity.has::<Alert>())?;
        map.serialize_entry(&ComponentId::Vehicle, &entity.has::<Vehicle>())?;
        map.end()
    }
}

pub fn export_to_json(world: &World, state: &State) -> String {
    let mut units = Vec::new();

    for (id, (_pos, _entity_type)) in world.query::<(&Pos, &EntityType)>().iter() {
        let entity = world.entity(id).unwrap();
        let unit_val = serde_json::to_value(UnitExport {
            id: id.id(),
            entity,
        }).unwrap();
        units.push(unit_val);
    }

    let data = ExportData {
        units,
        state: state.clone(),
    };
    serde_json::to_string(&data).unwrap()
}

struct UnitExport<'a> {
    id: u32,
    entity: EntityRef<'a>,
}

impl Serialize for UnitExport<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &(self.id as u64))?;
        let mut context = Context;
        context.serialize_entity(self.entity.clone(), map)
    }
}
