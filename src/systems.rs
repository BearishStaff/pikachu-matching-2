use bevy::{
    color::palettes::css::{BLUE, GREEN, ORANGE, RED, YELLOW},
    prelude::*,
};

use crate::components::Tile;
use crate::resources::Board;

const ROWS: usize = 7;
const COLS: usize = 10;
const TILE_SIZE: f32 = 64.0; // each tile 64x64 pixels

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut board = Board::new(ROWS, COLS);

    let kinds = [0, 1, 2, 3, 4]; // Pokémon type IDs

    for row in 0..ROWS {
        for col in 0..COLS {
            let kind = kinds[(row * COLS + col) % kinds.len()];
            let x = col as f32 * TILE_SIZE - (COLS as f32 * TILE_SIZE) / 2.0;
            let y = row as f32 * TILE_SIZE - (ROWS as f32 * TILE_SIZE) / 2.0;

            let entity = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: match kind {
                                0 => Color::Srgba(YELLOW),
                            1 => Color::Srgba(GREEN),
                            2 => Color::Srgba(BLUE),
                            3 => Color::Srgba(RED),
                            _ => Color::Srgba(ORANGE),
                            },
                            custom_size: Some(Vec2::splat(TILE_SIZE - 4.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, y, 0.0),
                        ..default()
                    },
                    Tile { row, col, kind },
                ))
                .id();
            board.cells[row][col] = Some(entity);
        }
    }
    commands.insert_resource(board);
    
}

#[derive(Resource, Default)]
pub struct SelectedTiles(Vec<Entity>);

pub fn select_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut selected: ResMut<SelectedTiles>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    q_tiles: Query<(Entity, &Tile, &Transform)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, cam_tf) = camera_q.single();
        let window = windows.single();

        if let Some(screen_pos) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(cam_tf, screen_pos) {
                let world_pos = ray.origin.truncate();

                for (entity, tile, transform) in q_tiles.iter() {
                    let tile_pos = transform.translation.truncate();
                    let half = TILE_SIZE / 2.0;
                    if (world_pos.x > tile_pos.x - half
                        && world_pos.x < tile_pos.x + half
                        && world_pos.y > tile_pos.y - half
                        && world_pos.y < tile_pos.y + half)
                    {
                        selected.0.push(entity);
                        if selected.0.len() == 1 {
                            commands.spawn(AudioBundle {
                                source: asset_server.load("sounds/select.ogg"),
                                settings: PlaybackSettings::ONCE,
                            });
                        }
                        println!(
                            "Selected tile at ({}, {}) type {}",
                            tile.row, tile.col, tile.kind
                        );
                    }
                }
            }
        }
    }
}

pub fn process_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut selected: ResMut<SelectedTiles>,
    q_tiles: Query<(Entity, &Tile)>,
    mut board: ResMut<Board>,
) {
    if selected.0.len() == 2 {
        let e1 = selected.0[0];
        let e2 = selected.0[1];

        let t1 = q_tiles.get(e1).unwrap().1;
        let t2 = q_tiles.get(e2).unwrap().1;

        if t1.kind == t2.kind && can_connect(&board, t1, t2) {
            println!("Tiles can connect!");
             commands.spawn(AudioBundle {
                            source: asset_server.load("sounds/matched.ogg"),
                            settings: PlaybackSettings::ONCE,
                        });
            commands.entity(e1).despawn();
            commands.entity(e2).despawn();
            board.cells[t1.row][t1.col] = None;
            board.cells[t2.row][t2.col] = None;

        } else {
            println!("Tiles cannot connect");
        }

        selected.0.clear();
    }
}

fn can_connect(board: &Board, t1: &Tile, t2: &Tile) -> bool {
    can_connect_straight(board, t1, t2)
        || can_connect_one_turn(board, t1, t2)
        || can_connect_two_turn(board, t1, t2)
}

fn can_connect_straight(board: &Board, t1: &Tile, t2: &Tile) -> bool {
    if t1.row == t2.row {
        // Same row
        let (c1, c2) = (t1.col.min(t2.col), t1.col.max(t2.col));
        for c in (c1 + 1)..c2 {
            if !board.is_empty(t1.row, c) {
                return false;
            }
        }
        return true;
    }

    if t1.col == t2.col {
        // Same column
        let (r1, r2) = (t1.row.min(t2.row), t1.row.max(t2.row));
        for r in (r1 + 1)..r2 {
            if !board.is_empty(r, t1.col) {
                return false;
            }
        }
        return true;
    }

    false
}

fn can_connect_one_turn(board: &Board, t1: &Tile, t2: &Tile) -> bool {
    let corner1 = Tile {
        row: t1.row,
        col: t2.col,
        kind: t1.kind,
    };
    let corner2 = Tile {
        row: t2.row,
        col: t1.col,
        kind: t1.kind,
    };

    if board.is_empty(corner1.row, corner1.col)
        && can_connect_straight(board, t1, &corner1)
        && can_connect_straight(board, &corner1, t2)
    {
        return true;
    }

    if board.is_empty(corner2.row, corner2.col)
        && can_connect_straight(board, t1, &corner2)
        && can_connect_straight(board, &corner2, t2)
    {
        return true;
    }

    false
}

fn can_connect_two_turn(board: &Board, t1: &Tile, t2: &Tile) -> bool {
    // Scan all rows
    for r in 0..ROWS {
        let corner = Tile {
            row: r,
            col: t1.col,
            kind: t1.kind,
        };
        if board.is_empty(corner.row, corner.col)
            && can_connect_straight(board, t1, &corner)
            && can_connect_one_turn(board, &corner, t2)
        {
            return true;
        }
    }

    // Scan all cols
    for c in 0..COLS {
        let corner = Tile {
            row: t1.row,
            col: c,
            kind: t1.kind,
        };
        if board.is_empty(corner.row, corner.col)
            && can_connect_straight(board, t1, &corner)
            && can_connect_one_turn(board, &corner, t2)
        {
            return true;
        }
    }

    false
}
