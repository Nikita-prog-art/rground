use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentToolCall {
    Move { dx: i32, dy: i32 },
    HarvestFront,
    Place { item: String },
    Craft { recipe_id: String },
    Wait,
    Say { message: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolResult {
    pub ok: bool,
    pub message: String,
}
