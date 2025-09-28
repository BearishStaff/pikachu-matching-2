use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub row: usize,
    pub col: usize,
    pub kind: u32, // Pokémon type ID
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct ConnectionLine;


#[derive(Component)]
pub struct LineTimer(pub Timer);

#[derive(Component)]
pub struct PendingRemove;