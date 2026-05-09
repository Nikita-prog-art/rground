use bevy::prelude::*;

use crate::game::player::components::Player;

use super::model::Inventory;

pub fn select_hotbar_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inventories: Query<&mut Inventory, With<Player>>,
) {
    let Some(index) = selected_digit(&keyboard) else {
        return;
    };

    for mut inventory in &mut inventories {
        if index < inventory.hotbar_size {
            inventory.selected = index;
        }
    }
}

fn selected_digit(keyboard: &ButtonInput<KeyCode>) -> Option<usize> {
    let keys = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];

    keys.iter().position(|key| keyboard.just_pressed(*key))
}
