use bevy::prelude::*;
use serde::Serialize;

use crate::game::{ids::ItemId, items::registry::ItemRegistry};

#[derive(Clone, Debug, Serialize)]
pub struct ItemStack {
    pub item: ItemId,
    pub quantity: u32,
}

#[derive(Component, Clone, Debug)]
pub struct Inventory {
    pub slots: Vec<Option<ItemStack>>,
    pub selected: usize,
    pub hotbar_size: usize,
}

impl Inventory {
    pub fn new(slot_count: usize, hotbar_size: usize) -> Self {
        Self {
            slots: vec![None; slot_count],
            selected: 0,
            hotbar_size: hotbar_size.min(slot_count),
        }
    }

    pub fn starter() -> Self {
        let mut inventory = Self::new(36, 9);
        inventory.force_add("oak_log", 8);
        inventory.force_add("cobblestone", 12);
        inventory.force_add("bread", 3);
        inventory.force_add("torch", 8);
        inventory
    }

    pub fn agent_starter() -> Self {
        let mut inventory = Self::new(18, 6);
        inventory.force_add("oak_log", 2);
        inventory.force_add("bread", 1);
        inventory
    }

    pub fn selected_stack(&self) -> Option<&ItemStack> {
        self.slots.get(self.selected).and_then(Option::as_ref)
    }

    pub fn count(&self, item: ItemId) -> u32 {
        self.slots
            .iter()
            .flatten()
            .filter(|stack| stack.item == item)
            .map(|stack| stack.quantity)
            .sum()
    }

    pub fn has(&self, item: ItemId, quantity: u32) -> bool {
        self.count(item) >= quantity
    }

    pub fn can_add(&self, item: ItemId, quantity: u32, registry: &ItemRegistry) -> bool {
        self.remaining_capacity(item, registry) >= quantity
    }

    pub fn add(&mut self, item: ItemId, quantity: u32, registry: &ItemRegistry) -> u32 {
        let max_stack = registry.max_stack(item);
        let mut remaining = quantity;

        for slot in self.slots.iter_mut().flatten() {
            if slot.item == item && slot.quantity < max_stack {
                let accepted = (max_stack - slot.quantity).min(remaining);
                slot.quantity += accepted;
                remaining -= accepted;
                if remaining == 0 {
                    return 0;
                }
            }
        }

        for slot in &mut self.slots {
            if slot.is_none() {
                let accepted = max_stack.min(remaining);
                *slot = Some(ItemStack {
                    item,
                    quantity: accepted,
                });
                remaining -= accepted;
                if remaining == 0 {
                    return 0;
                }
            }
        }

        remaining
    }

    pub fn remove(&mut self, item: ItemId, quantity: u32) -> bool {
        if !self.has(item, quantity) {
            return false;
        }

        let mut remaining = quantity;
        for slot in &mut self.slots {
            let Some(stack) = slot else {
                continue;
            };
            if stack.item != item {
                continue;
            }

            let removed = stack.quantity.min(remaining);
            stack.quantity -= removed;
            remaining -= removed;
            if stack.quantity == 0 {
                *slot = None;
            }
            if remaining == 0 {
                return true;
            }
        }

        true
    }

    pub fn force_add(&mut self, item: ItemId, quantity: u32) {
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(ItemStack { item, quantity });
                return;
            }
        }
    }

    pub fn summary(&self, registry: &ItemRegistry, limit: usize) -> String {
        let mut parts = Vec::new();
        for stack in self.slots.iter().flatten().take(limit) {
            parts.push(format!("{} x{}", registry.name(stack.item), stack.quantity));
        }
        if parts.is_empty() {
            "empty".to_string()
        } else {
            parts.join(", ")
        }
    }

    fn remaining_capacity(&self, item: ItemId, registry: &ItemRegistry) -> u32 {
        let max_stack = registry.max_stack(item);
        self.slots
            .iter()
            .map(|slot| match slot {
                Some(stack) if stack.item == item => max_stack.saturating_sub(stack.quantity),
                Some(_) => 0,
                None => max_stack,
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stacks_and_removes_items() {
        let registry = ItemRegistry::default();
        let mut inventory = Inventory::new(2, 2);

        assert_eq!(inventory.add("oak_log", 32, &registry), 0);
        assert_eq!(inventory.add("oak_log", 40, &registry), 0);
        assert_eq!(inventory.count("oak_log"), 72);

        assert!(inventory.remove("oak_log", 65));
        assert_eq!(inventory.count("oak_log"), 7);
        assert!(!inventory.remove("oak_log", 8));
    }
}
