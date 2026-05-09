# RGround

RGround is a Rust + Bevy 2D top-down survival sandbox prototype inspired by block crafting, action-adventure exploration, and village/farm simulation.

The current build is a playable vertical slice:

- deterministic tile world with harvestable trees, stone, ores, water, sand, placed stations, and blocks
- player movement, harvesting, placing, inventory, hotbar, recipe selection, and crafting
- Minecraft-like item registry and broad starter recipe book
- classical mobs: zombies, skeletons, villagers
- scalable LLM-agent architecture with serializable observations and tool-call actions
- Bevy UI HUD, hotbar, inventory/crafting panel, recipe controls, and simulation stats

No official Minecraft, Zelda, or Stardew assets are used.

## Run

```bash
cargo run
```

## Commit Step

Run checks, commit the project paths, and push the current branch:

```bash
./tests/commit-step.sh -m "Update game prototype"
```

## Controls

- `WASD` move
- `Space` harvest the tile in front of the player
- `F` place the selected hotbar block or station
- `1..9` select hotbar slot
- `E` open/close inventory and crafting
- `[` / `]` select recipe
- `C` craft selected recipe

## Architecture

- `src/game/world` owns the tile map, terrain generation, tile metadata, and tile visuals
- `src/game/items` owns item definitions and stack metadata
- `src/game/inventory` owns slot storage and hotbar selection
- `src/game/crafting` owns recipe definitions and craft execution
- `src/game/player` owns player input and direct world interaction
- `src/game/actors` owns shared actor components, mob spawning, and classical AI
- `src/game/agents` owns LLM-agent observations, tool-call protocol, scheduling, and local planner fallback
- `src/game/ui` owns HUD, hotbar, crafting panel, and buttons

The LLM path is intentionally split into protocol and execution. External API code can fill `LlmBridge.inbound` with tool calls without giving the model direct ECS access.
