use bevy::prelude::*;

#[derive(Resource)]
pub struct Board {
    pub cells: Vec<Vec<Option<Entity>>>, // None = empty, Some = tile entity
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            cells: vec![vec![None; cols]; rows],
        }
    }

    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.cells[row][col].is_none()
    }
}
