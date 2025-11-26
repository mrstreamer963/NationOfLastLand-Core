use crate::modules::components::*;
use crate::modules::markers::*;

use crate::modules::state::State;
use hecs::{serialize::row::*, World, EntityRef};
use serde::{Serialize, ser::SerializeMap};

macro_rules! define_serialize_components {
    (
        components: $( $comp:ty ),* $(,)* ;
        markers: $( $mark:ty ),* $(,)* ;
    ) => {
        fn serialize_components<'a, S>(entity: EntityRef<'a>, mut map: S) -> Result<S::Ok, S::Error>
        where
            S: SerializeMap,
        {
            $(
                try_serialize::<$comp, _, _>(&entity, stringify!($comp), &mut map)?;
            )*
            $(
                if entity.has::<$mark>() {
                    map.serialize_entry(stringify!($mark), &true)?;
                }
            )*
            map.end()
        }
    };
}

define_serialize_components! {
    components: Pos, EntityType, Health, Velocity, Rot, MaxSpeed, TargetPos, Reputation, UnitState, TargetId, DamageType;
    markers: Alert, Vehicle;
}

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

struct Context;

impl SerializeContext for Context {
    fn serialize_entity<S>(
        &mut self,
        entity: EntityRef<'_>,
        map: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: SerializeMap,
    {
        serialize_components(entity, map)
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
