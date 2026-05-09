pub mod llm;
pub mod scheduler;
pub mod tools;

use bevy::prelude::*;

use self::scheduler::{
    AgentScheduler, apply_inbound_llm_responses_system, execute_agent_tools_system,
    schedule_agent_thinking_system,
};

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AgentScheduler>()
            .init_resource::<llm::LlmBridge>()
            .add_systems(
                Update,
                (
                    schedule_agent_thinking_system,
                    apply_inbound_llm_responses_system,
                    execute_agent_tools_system,
                ),
            );
    }
}
