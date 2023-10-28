//! A simplified implementation of the classic game "Breakout".

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const BRICK_SIZE: Vec2 = Vec2::new(100., 100.);

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                // resizable: false,
                title: "Test".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::AltLeft)),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, bevy::window::close_on_esc))
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(
                WINDOW_WIDTH / 2.,
                WINDOW_HEIGHT / 2.,
                0.,
            )),
            ..default()
        })
        .insert(Name::new("Main Camera"));

    assert!(BRICK_SIZE.x > 0.0);
    assert!(BRICK_SIZE.y > 0.0);

    // let n_columns = (WINDOW_WIDTH / BRICK_SIZE.x).floor() as usize;
    // let n_rows = (WINDOW_HEIGHT / BRICK_SIZE.y).floor() as usize;
    let n_columns = 1;
    let n_rows = 1;

    let total_brick_width = n_columns as f32 * BRICK_SIZE.x;
    let total_brick_height = n_rows as f32 * BRICK_SIZE.y;

    let start_x = (WINDOW_WIDTH - total_brick_width) / 2.0;
    let start_y = (WINDOW_HEIGHT - total_brick_height) / 2.0;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                start_x + column as f32 * (BRICK_SIZE.x) + BRICK_SIZE.x / 2.0,
                start_y + row as f32 * (BRICK_SIZE.y) + BRICK_SIZE.y / 2.0,
            );

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Text2dBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "0".to_string(),
                                    ..default()
                                }],
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // Ensure text appears above brick
                            ..default()
                        })
                        .insert(Name::new("Brick Text"));
                })
                .insert(Cell { count: 0 })
                .insert(Name::new("Brick"));
        }
    }
}

fn update(
    mut bricks: Query<(&mut Sprite, &mut Cell, &Children)>,
    mut texts: Query<&mut Text>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for (mut square, mut cell, children) in bricks.iter_mut() {
            square.color = Color::rgb(1.0, 0.0, 0.0);
            cell.count += 1;
            if let Some(text_entity) = children.iter().next() {
                if let Ok(mut text) = texts.get_mut(*text_entity) {
                    text.sections[0].value = cell.count.to_string();
                }
            }
        }
    }
}

#[derive(Component)]
struct Cell {
    count: i16,
}
