use std::collections::VecDeque;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::tools::{AgentToolCall, ToolResult};

#[derive(Component, Clone, Debug)]
pub struct AgentBrain {
    pub id: u32,
    pub model: String,
    pub pending_tool: Option<AgentToolCall>,
    pub last_observation_json: String,
    pub last_result: Option<ToolResult>,
    pub decisions: u64,
}

impl AgentBrain {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            model: "local-planner-until-api-client-is-attached".to_string(),
            pending_tool: None,
            last_observation_json: "{}".to_string(),
            last_result: None,
            decisions: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentObservation {
    pub agent_id: u32,
    pub position_tile: (i32, i32),
    pub facing: (i32, i32),
    pub tile_front: String,
    pub health: (f32, f32),
    pub inventory: Vec<(String, u32)>,
    pub nearby_counts: NearbyCounts,
    pub goal_hint: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NearbyCounts {
    pub agents: usize,
    pub zombies: usize,
    pub skeletons: usize,
    pub villagers: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct LlmRequest {
    pub agent_id: u32,
    pub model: String,
    pub observation: AgentObservation,
    pub available_tools: Vec<&'static str>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LlmResponse {
    pub agent_id: u32,
    pub tool_call: AgentToolCall,
}

#[derive(Resource, Default)]
pub struct LlmBridge {
    pub outbound: VecDeque<LlmRequest>,
    pub inbound: VecDeque<LlmResponse>,
}
