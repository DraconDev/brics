use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    pub id: String,
    pub payload: serde_json::Value,
}
