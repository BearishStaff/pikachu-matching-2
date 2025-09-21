use bevy::{
    color::palettes::css::{BLUE, GREEN, ORANGE, RED, YELLOW},
    prelude::*,
};

use crate::components::Tile;

const ROWS: usize = 7;
const COLS: usize = 10;
const TILE_SIZE: f32 = 64.0; // each tile 64x64 pixels

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let kinds = [0, 1, 2, 3, 4]; // Pokémon types (just IDs for now)

    for row in 0..ROWS {
        for col in 0..COLS {
            let kind = kinds[(row * COLS + col) % kinds.len()]; // demo assignment
            let x = col as f32 * TILE_SIZE - (COLS as f32 * TILE_SIZE) / 2.0;
            let y = row as f32 * TILE_SIZE - (ROWS as f32 * TILE_SIZE) / 2.0;

            commands.spawn((
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
            ));
        }
    }
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
) {
    if selected.0.len() == 2 {
        let e1 = selected.0[0];
        let e2 = selected.0[1];

        let t1 = q_tiles.get(e1).unwrap().1;
        let t2 = q_tiles.get(e2).unwrap().1;

        if t1.kind == t2.kind && !(t1.row == t2.row && t1.col == t2.col) {
            // TODO: run connection check here
            println!(
                "Tiles match! ({}, {}) <-> ({}, {})",
                t1.row, t1.col, t2.row, t2.col
            );
            commands.spawn(AudioBundle {
                            source: asset_server.load("sounds/matched.ogg"),
                            settings: PlaybackSettings::ONCE,
                        });
            commands.entity(e1).despawn();
            commands.entity(e2).despawn();
        } else {
            if t1.row == t2.row && t1.col == t2.col {
                println!("Unselect tile");
            } else {
                println!("Tiles don't match");
            }
        }

        selected.0.clear();
    }
}
