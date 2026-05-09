use bevy::prelude::*;

use crate::game::{
    actors::components::{ActorKind, ActorName, Health},
    agents::{llm::AgentBrain, scheduler::AgentScheduler},
    crafting::recipes::{RecipeBook, SelectedRecipe, can_craft, try_craft},
    inventory::model::Inventory,
    items::registry::ItemRegistry,
    player::components::Player,
    world::map::TileMap,
};

const PANEL_BG: Color = Color::srgba(0.055, 0.060, 0.070, 0.88);
const PANEL_BORDER: Color = Color::srgb(0.28, 0.31, 0.34);
const SLOT_BG: Color = Color::srgba(0.11, 0.12, 0.13, 0.94);
const SLOT_ACTIVE: Color = Color::srgb(0.22, 0.48, 0.58);
const TEXT: Color = Color::srgb(0.88, 0.90, 0.88);
const MUTED: Color = Color::srgb(0.58, 0.63, 0.62);

#[derive(Component)]
pub struct HudText;

#[derive(Component)]
pub struct AgentStatsText;

#[derive(Component)]
pub struct InventoryPanel;

#[derive(Component)]
pub struct InventoryPanelText;

#[derive(Component)]
pub struct RecipePanelText;

#[derive(Component)]
pub struct HotbarSlot {
    index: usize,
}

#[derive(Component)]
pub struct HotbarSlotText {
    index: usize,
}

#[derive(Component)]
pub enum RecipeButtonAction {
    Previous,
    Next,
    Craft,
}

#[derive(Resource)]
pub struct InventoryPanelState {
    pub open: bool,
}

impl Default for InventoryPanelState {
    fn default() -> Self {
        Self { open: true }
    }
}

pub fn setup_game_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(14.0),
                top: Val::Px(14.0),
                width: Val::Px(360.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("RGround"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT),
            ));
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(MUTED),
                HudText,
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(14.0),
                top: Val::Px(14.0),
                width: Val::Px(310.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Simulation"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(TEXT),
            ));
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(MUTED),
                AgentStatsText,
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(14.0),
                bottom: Val::Px(14.0),
                height: Val::Px(66.0),
                padding: UiRect::all(Val::Px(6.0)),
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
        ))
        .with_children(|parent| {
            for index in 0..9 {
                parent
                    .spawn((
                        Node {
                            width: Val::Px(54.0),
                            height: Val::Px(54.0),
                            padding: UiRect::all(Val::Px(4.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(SLOT_BG),
                        BorderColor::all(PANEL_BORDER),
                        HotbarSlot { index },
                    ))
                    .with_children(|slot| {
                        slot.spawn((
                            Text::new(""),
                            TextFont {
                                font_size: 11.0,
                                ..default()
                            },
                            TextColor(TEXT),
                            HotbarSlotText { index },
                        ));
                    });
            }
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(14.0),
                top: Val::Px(122.0),
                width: Val::Px(590.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                display: Display::Flex,
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor::all(PANEL_BORDER),
            InventoryPanel,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Inventory"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(TEXT),
            ));
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(MUTED),
                InventoryPanelText,
            ));
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(TEXT),
                RecipePanelText,
            ));
            parent
                .spawn((Node {
                    height: Val::Px(38.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    ..default()
                },))
                .with_children(|buttons| {
                    recipe_button(buttons, "<", RecipeButtonAction::Previous);
                    recipe_button(buttons, "Craft", RecipeButtonAction::Craft);
                    recipe_button(buttons, ">", RecipeButtonAction::Next);
                });
        });
}

pub fn toggle_inventory_panel_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<InventoryPanelState>,
    mut panels: Query<&mut Node, With<InventoryPanel>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }
    state.open = !state.open;
    for mut node in &mut panels {
        node.display = if state.open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn update_player_hud_system(
    tile_map: Res<TileMap>,
    registry: Res<ItemRegistry>,
    player: Query<(&Transform, &Health, &Inventory), With<Player>>,
    mut text_query: Query<&mut Text, With<HudText>>,
) {
    let Ok((transform, health, inventory)) = player.single() else {
        return;
    };
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    let tile = tile_map.world_to_tile(transform.translation);
    let tile_name = tile_map.get(tile).map_or("Void", |tile| tile.name());
    let selected = inventory
        .selected_stack()
        .map_or("empty".to_string(), |stack| {
            let kind = registry
                .get(stack.item)
                .map_or("Item".to_string(), |def| format!("{:?}", def.kind));
            format!(
                "{} | {} x{}",
                kind,
                registry.name(stack.item),
                stack.quantity
            )
        });

    text.0 = format!(
        "HP {:.0}/{:.0}\nTile {}, {} | {}\nSelected: {}",
        health.current, health.max, tile.x, tile.y, tile_name, selected
    );
}

pub fn update_hotbar_system(
    registry: Res<ItemRegistry>,
    player: Query<&Inventory, With<Player>>,
    mut slots: Query<(&HotbarSlot, &mut BackgroundColor)>,
    mut labels: Query<(&HotbarSlotText, &mut Text)>,
) {
    let Ok(inventory) = player.single() else {
        return;
    };

    for (slot, mut background) in &mut slots {
        background.0 = if slot.index == inventory.selected {
            SLOT_ACTIVE
        } else {
            SLOT_BG
        };
    }

    for (label, mut text) in &mut labels {
        text.0 = inventory
            .slots
            .get(label.index)
            .and_then(Option::as_ref)
            .map_or(String::new(), |stack| {
                format!(
                    "{}\n{}",
                    short_name(registry.name(stack.item)),
                    stack.quantity
                )
            });
    }
}

pub fn update_inventory_panel_system(
    registry: Res<ItemRegistry>,
    recipe_book: Res<RecipeBook>,
    selected: Res<SelectedRecipe>,
    tile_map: Res<TileMap>,
    player: Query<(&Transform, &Inventory), With<Player>>,
    mut inventory_text: Query<&mut Text, With<InventoryPanelText>>,
    mut recipe_text: Query<&mut Text, (With<RecipePanelText>, Without<InventoryPanelText>)>,
) {
    let Ok((transform, inventory)) = player.single() else {
        return;
    };

    if let Ok(mut text) = inventory_text.single_mut() {
        text.0 = inventory.summary(&registry, 36);
    }

    let Ok(mut text) = recipe_text.single_mut() else {
        return;
    };
    let Some(recipe) = recipe_book.recipes.get(selected.index) else {
        text.0.clear();
        return;
    };

    let ingredients = recipe
        .ingredients
        .iter()
        .map(|stack| format!("{} x{}", registry.name(stack.item), stack.quantity))
        .collect::<Vec<_>>()
        .join(", ");
    let tile_pos = tile_map.world_to_tile(transform.translation);
    let station_ready = tile_map.station_available_near(tile_pos, recipe.station);
    let craftable = if station_ready && can_craft(inventory, recipe, &registry) {
        "ready"
    } else if !station_ready {
        "station missing"
    } else {
        "missing"
    };

    text.0 = format!(
        "Recipe {}/{}: {} -> {} x{}\nStation: {:?} | {:?} | {}\n{}\n{}",
        selected.index + 1,
        recipe_book.recipes.len(),
        recipe.name,
        registry.name(recipe.output.item),
        recipe.output.quantity,
        recipe.station,
        recipe.category,
        craftable,
        ingredients,
        selected.last_status
    );
}

pub fn update_agent_stats_system(
    scheduler: Res<AgentScheduler>,
    registry: Res<ItemRegistry>,
    actors: Query<(&ActorKind, Option<&ActorName>)>,
    agents: Query<&AgentBrain>,
    mut text_query: Query<&mut Text, With<AgentStatsText>>,
) {
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    let mut llm_agents = 0usize;
    let mut zombies = 0usize;
    let mut skeletons = 0usize;
    let mut villagers = 0usize;
    let mut first_agent_name = "none".to_string();
    for (kind, name) in &actors {
        match kind {
            ActorKind::LlmAgent => {
                if llm_agents == 0 {
                    first_agent_name = name.map_or("Agent".to_string(), |name| name.0.clone());
                }
                llm_agents += 1;
            }
            ActorKind::Zombie => zombies += 1,
            ActorKind::Skeleton => skeletons += 1,
            ActorKind::Villager => villagers += 1,
            ActorKind::Player => {}
        }
    }

    let pending = agents
        .iter()
        .filter(|brain| brain.pending_tool.is_some())
        .count();

    text.0 = format!(
        "LLM agents: {}\nSample: {}\nPending tools: {}\nZombies: {} | Skeletons: {}\nVillagers: {}\nItems: {}\nDecisions: {}",
        llm_agents,
        first_agent_name,
        pending,
        zombies,
        skeletons,
        villagers,
        registry.all_ids().len(),
        scheduler.decisions_total
    );
}

pub fn button_interaction_system(
    mut selected: ResMut<SelectedRecipe>,
    recipe_book: Res<RecipeBook>,
    registry: Res<ItemRegistry>,
    tile_map: Res<TileMap>,
    mut interactions: Query<
        (&Interaction, &RecipeButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut player: Query<(&Transform, &mut Inventory), With<Player>>,
) {
    if recipe_book.recipes.is_empty() {
        return;
    }

    for (interaction, action, mut background) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                background.0 = SLOT_ACTIVE;
                match action {
                    RecipeButtonAction::Previous => {
                        selected.index = selected
                            .index
                            .checked_sub(1)
                            .unwrap_or(recipe_book.recipes.len() - 1);
                        selected.last_status.clear();
                    }
                    RecipeButtonAction::Next => {
                        selected.index = (selected.index + 1) % recipe_book.recipes.len();
                        selected.last_status.clear();
                    }
                    RecipeButtonAction::Craft => {
                        let Some(recipe) = recipe_book.recipes.get(selected.index) else {
                            continue;
                        };
                        let Ok((transform, mut inventory)) = player.single_mut() else {
                            continue;
                        };
                        let tile_pos = tile_map.world_to_tile(transform.translation);
                        if !tile_map.station_available_near(tile_pos, recipe.station) {
                            selected.last_status = format!("Need {:?}", recipe.station);
                            continue;
                        }
                        selected.last_status = match try_craft(&mut inventory, recipe, &registry) {
                            Ok(()) => format!("Crafted {}", registry.name(recipe.output.item)),
                            Err(error) => format!("{error:?}"),
                        };
                    }
                }
            }
            Interaction::Hovered => {
                background.0 = Color::srgb(0.18, 0.22, 0.25);
            }
            Interaction::None => {
                background.0 = SLOT_BG;
            }
        }
    }
}

fn recipe_button(
    parent: &mut ChildSpawnerCommands,
    label: &'static str,
    action: RecipeButtonAction,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(if label == "Craft" { 104.0 } else { 42.0 }),
                height: Val::Px(34.0),
                border: UiRect::all(Val::Px(1.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderColor::all(PANEL_BORDER),
            BackgroundColor(SLOT_BG),
            action,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT),
            ));
        });
}

fn short_name(name: &str) -> String {
    let mut chars = name.chars().filter(|c| c.is_ascii_alphanumeric());
    let mut out = String::new();
    for _ in 0..4 {
        let Some(ch) = chars.next() else {
            break;
        };
        out.push(ch.to_ascii_uppercase());
    }
    out
}
