use bevy::prelude::*;

use crate::game::{
    actors::components::{ActorKind, Health},
    crafting::recipes::{RecipeBook, try_craft},
    inventory::model::Inventory,
    items::registry::ItemRegistry,
    player::{
        components::Facing,
        systems::{harvest_tile, set_tile_visual},
    },
    world::{
        map::{TileMap, TileVisual, TileVisuals},
        tiles::TileKind,
    },
};

use super::{
    llm::{AgentBrain, AgentObservation, LlmBridge, LlmRequest, NearbyCounts},
    tools::{AgentToolCall, ToolResult},
};

#[derive(Resource)]
pub struct AgentScheduler {
    timer: Timer,
    max_decisions_per_tick: usize,
    pub decisions_total: u64,
}

impl Default for AgentScheduler {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.18, TimerMode::Repeating),
            max_decisions_per_tick: 48,
            decisions_total: 0,
        }
    }
}

pub fn schedule_agent_thinking_system(
    time: Res<Time>,
    mut scheduler: ResMut<AgentScheduler>,
    mut bridge: ResMut<LlmBridge>,
    tile_map: Res<TileMap>,
    registry: Res<ItemRegistry>,
    actors: Query<(&Transform, &ActorKind)>,
    mut agents: Query<(&Transform, &Facing, &Health, &Inventory, &mut AgentBrain)>,
) {
    scheduler.timer.tick(time.delta());
    if !scheduler.timer.just_finished() {
        return;
    }

    let nearby = count_nearby_actors(&actors);
    let mut processed = 0usize;

    for (transform, facing, health, inventory, mut brain) in &mut agents {
        if processed >= scheduler.max_decisions_per_tick {
            break;
        }
        if brain.pending_tool.is_some() {
            continue;
        }

        let tile_pos = tile_map.world_to_tile(transform.translation);
        let front = tile_map
            .get(tile_pos + facing.0)
            .map_or("Void".to_string(), |tile| tile.name().to_string());

        let inventory_summary = inventory
            .slots
            .iter()
            .flatten()
            .take(10)
            .map(|stack| (registry.name(stack.item).to_string(), stack.quantity))
            .collect::<Vec<_>>();

        let observation = AgentObservation {
            agent_id: brain.id,
            position_tile: (tile_pos.x, tile_pos.y),
            facing: (facing.0.x, facing.0.y),
            tile_front: front,
            health: (health.current, health.max),
            inventory: inventory_summary,
            nearby_counts: nearby.clone(),
            goal_hint: "survive, gather logs/stone, craft basic tools, avoid mobs".to_string(),
        };

        brain.last_observation_json =
            serde_json::to_string(&observation).unwrap_or_else(|_| "{}".to_string());

        bridge.outbound.push_back(LlmRequest {
            agent_id: brain.id,
            model: brain.model.clone(),
            observation: observation.clone(),
            available_tools: vec!["move", "harvest_front", "place", "craft", "wait", "say"],
        });

        brain.pending_tool = Some(local_planner_tool(&tile_map, tile_pos, facing.0, inventory));
        brain.decisions += 1;
        scheduler.decisions_total += 1;
        processed += 1;
    }
}

pub fn execute_agent_tools_system(
    mut tile_map: ResMut<TileMap>,
    visuals: Res<TileVisuals>,
    registry: Res<ItemRegistry>,
    recipes: Res<RecipeBook>,
    mut tile_sprites: Query<(&mut Sprite, &mut Transform), With<TileVisual>>,
    mut agents: Query<
        (&mut Transform, &mut Facing, &mut Inventory, &mut AgentBrain),
        Without<TileVisual>,
    >,
) {
    for (mut transform, mut facing, mut inventory, mut brain) in &mut agents {
        let Some(tool) = brain.pending_tool.take() else {
            continue;
        };

        let result = match tool {
            AgentToolCall::Move { dx, dy } => {
                let step = IVec2::new(dx.signum(), dy.signum());
                if step != IVec2::ZERO {
                    facing.0 = step;
                }
                let target = tile_map.world_to_tile(transform.translation) + step;
                if tile_map.is_walkable(target) {
                    let mut next = tile_map.tile_to_world(target);
                    next.z = transform.translation.z;
                    transform.translation = next;
                    ToolResult {
                        ok: true,
                        message: "moved".to_string(),
                    }
                } else {
                    ToolResult {
                        ok: false,
                        message: "blocked".to_string(),
                    }
                }
            }
            AgentToolCall::HarvestFront => {
                let target = tile_map.world_to_tile(transform.translation) + facing.0;
                let ok = harvest_tile(
                    target,
                    &mut tile_map,
                    &visuals,
                    &registry,
                    &mut tile_sprites,
                    &mut inventory,
                );
                ToolResult {
                    ok,
                    message: if ok { "harvested" } else { "nothing harvested" }.to_string(),
                }
            }
            AgentToolCall::Place { item } => {
                let Some(item_id) = registry.resolve_id(&item) else {
                    brain.last_result = Some(ToolResult {
                        ok: false,
                        message: "unknown item".to_string(),
                    });
                    continue;
                };
                let Some(tile) = TileKind::from_place_item(item_id) else {
                    brain.last_result = Some(ToolResult {
                        ok: false,
                        message: "not placeable".to_string(),
                    });
                    continue;
                };
                let target = tile_map.world_to_tile(transform.translation) + facing.0;
                let ok = tile_map.get(target).is_some_and(TileKind::is_walkable)
                    && inventory.remove(item_id, 1);
                if ok {
                    set_tile_visual(target, tile, &mut tile_map, &visuals, &mut tile_sprites);
                }
                ToolResult {
                    ok,
                    message: if ok { "placed" } else { "could not place" }.to_string(),
                }
            }
            AgentToolCall::Craft { recipe_id } => {
                let Some(recipe) = recipes.recipes.iter().find(|recipe| recipe.id == recipe_id)
                else {
                    brain.last_result = Some(ToolResult {
                        ok: false,
                        message: "unknown recipe".to_string(),
                    });
                    continue;
                };
                let tile_pos = tile_map.world_to_tile(transform.translation);
                let ok = tile_map.station_available_near(tile_pos, recipe.station)
                    && try_craft(&mut inventory, recipe, &registry).is_ok();
                ToolResult {
                    ok,
                    message: if ok { "crafted" } else { "craft failed" }.to_string(),
                }
            }
            AgentToolCall::Wait => ToolResult {
                ok: true,
                message: "waited".to_string(),
            },
            AgentToolCall::Say { message } => ToolResult { ok: true, message },
        };

        brain.last_result = Some(result);
    }
}

pub fn apply_inbound_llm_responses_system(
    mut bridge: ResMut<LlmBridge>,
    mut agents: Query<&mut AgentBrain>,
) {
    while let Some(response) = bridge.inbound.pop_front() {
        for mut brain in &mut agents {
            if brain.id == response.agent_id && brain.pending_tool.is_none() {
                brain.pending_tool = Some(response.tool_call.clone());
                break;
            }
        }
    }
}

fn local_planner_tool(
    tile_map: &TileMap,
    tile_pos: IVec2,
    facing: IVec2,
    inventory: &Inventory,
) -> AgentToolCall {
    if inventory.has("oak_log", 1) && !inventory.has("oak_planks", 4) {
        return AgentToolCall::Craft {
            recipe_id: "oak_planks".to_string(),
        };
    }
    if inventory.has("oak_planks", 2) && !inventory.has("stick", 4) {
        return AgentToolCall::Craft {
            recipe_id: "sticks_oak".to_string(),
        };
    }

    let front = tile_pos + facing;
    if tile_map
        .get(front)
        .and_then(TileKind::harvest_drop)
        .is_some()
    {
        return AgentToolCall::HarvestFront;
    }

    let directions = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
    let choice = ((tile_pos.x * 31 + tile_pos.y * 17).unsigned_abs() as usize) % directions.len();
    let mut direction = directions[choice];

    if !tile_map.is_walkable(tile_pos + direction) {
        direction = directions
            .iter()
            .copied()
            .find(|candidate| tile_map.is_walkable(tile_pos + *candidate))
            .unwrap_or(IVec2::ZERO);
    }

    AgentToolCall::Move {
        dx: direction.x,
        dy: direction.y,
    }
}

fn count_nearby_actors(actors: &Query<(&Transform, &ActorKind)>) -> NearbyCounts {
    let mut counts = NearbyCounts::default();
    for (_, kind) in actors.iter() {
        match kind {
            ActorKind::LlmAgent => counts.agents += 1,
            ActorKind::Zombie => counts.zombies += 1,
            ActorKind::Skeleton => counts.skeletons += 1,
            ActorKind::Villager => counts.villagers += 1,
            ActorKind::Player => {}
        }
    }
    counts
}
