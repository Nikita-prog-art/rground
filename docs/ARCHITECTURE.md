# Architecture Notes

## Agent scale

AI agents are ECS entities with `AgentBrain`, `Inventory`, `Health`, `Facing`, and transform components. They do not call models directly from gameplay systems. Instead:

1. `schedule_agent_thinking_system` builds a compact `AgentObservation`.
2. The observation is serialized and mirrored into `LlmBridge.outbound`.
3. A real API worker can replace the local planner by pushing `LlmResponse` into `LlmBridge.inbound`.
4. Only validated `AgentToolCall` values are executed against the world.

This keeps the expensive part outside the ECS hot path and makes it possible to throttle model calls by tick, priority, distance from the player, or budget.

## World scale

The current renderer spawns one tile sprite per generated tile for clarity. Gameplay state is still stored in `TileMap`, not per-tile components. For very large maps, the next step is replacing `spawn_world_tiles` with a chunk/viewport tile renderer while keeping the same `TileMap` API.

## Content

Items and recipes are Rust data today because the project is new and compile-time safety is useful. They should move to `assets/data/items.ron` and `assets/data/recipes.ron` once the schema stabilizes.
