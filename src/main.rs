//! Shows text rendering with moving, rotating and scaling text.
//!
//! Note that this uses [`Text2dBundle`] to display text alongside your other entities in a 2D scene.
//!
//! For an example on how to render text as part of a user interface, independent from the world
//! viewport, you may want to look at `2d/contributors.rs` or `ui/text.rs`.

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
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
                resizable: false,
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
        .add_systems(
            Update,
            (
                update,
                bevy::window::close_on_esc,
                update_world_cords.before(update),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.init_resource::<MyWorldCoords>();

    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(
                WINDOW_WIDTH / 2.,
                WINDOW_HEIGHT / 2.,
                0.,
            )),
            ..default()
        })
        .insert(MainCamera)
        .insert(Name::new("Main Camera"));

    assert!(BRICK_SIZE.x > 0.0);
    assert!(BRICK_SIZE.y > 0.0);

    let n_columns = (WINDOW_WIDTH / BRICK_SIZE.x).floor() as usize;
    let n_rows = (WINDOW_HEIGHT / BRICK_SIZE.y).floor() as usize;

    let total_brick_width = n_columns as f32 * BRICK_SIZE.x;
    let total_brick_height = n_rows as f32 * BRICK_SIZE.y;

    let start_x = (WINDOW_WIDTH - total_brick_width) / 2.0;
    let start_y = (WINDOW_HEIGHT - total_brick_height) / 2.0;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let box_position = Vec2::new(
                start_x + column as f32 * (BRICK_SIZE.x) + BRICK_SIZE.x / 2.0,
                start_y + row as f32 * (BRICK_SIZE.y) + BRICK_SIZE.y / 2.0,
            );
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        custom_size: Some(BRICK_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(box_position.extend(0.0)),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "1",
                                TextStyle {
                                    font_size: 42.0,
                                    ..default()
                                },
                            )],
                            ..default()
                        },
                        // ensure the text is drawn on top of the box
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    });
                })
                .insert(Cell { count: 1 })
                .insert(Name::new(format!("Cell {}-{}", row, column)));
        }
    }
}

fn update(
    mut bricks: Query<(&mut Sprite, &mut Cell, &Children, &GlobalTransform)>,
    mut texts: Query<&mut Text>,
    mouse: Res<Input<MouseButton>>,
    mycoords: Res<MyWorldCoords>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // Convert the stored cursor world position into a Vec3 for comparison
        let cursor_position = mycoords.0.extend(0.0);

        // Iterate over the bricks to find the one that contains the cursor
        for (_sprite, mut cell, children, transform) in bricks.iter_mut() {
            // Check if the cursor is within the bounds of this cell's sprite
            if is_cursor_in_cell(cursor_position, transform) {
                // Update the cell's count and text
                cell.count += 1;
                if let Some(text_entity) = children.iter().next() {
                    if let Ok(mut text) = texts.get_mut(*text_entity) {
                        text.sections[0].value = cell.count.to_string();
                    }
                }
                // Break the loop after updating the clicked cell
                break;
            }
        }
    }
}

// Helper function to determine if the cursor is within the cell
fn is_cursor_in_cell(cursor_position: Vec3, cell_transform: &GlobalTransform) -> bool {
    let cell_position = cell_transform.translation();
    let half_size = BRICK_SIZE / 2.0;

    // Check bounds for x and y separately
    cursor_position.x > cell_position.x - half_size.x
        && cursor_position.x < cell_position.x + half_size.x
        && cursor_position.y > cell_position.y - half_size.y
        && cursor_position.y < cell_position.y + half_size.y
}

fn update_world_cords(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse: Res<Input<MouseButton>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

#[derive(Component)]
struct Cell {
    count: i16,
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;
