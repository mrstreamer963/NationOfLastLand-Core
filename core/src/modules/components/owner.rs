use hecs::Entity;
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Owner(pub Entity);
