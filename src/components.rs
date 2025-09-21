use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub row: usize,
    pub col: usize,
    pub kind: u32, // Pokémon type ID
}