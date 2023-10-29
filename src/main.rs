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
const WINDOW_HEIGHT: f32 = 650.0;

const CORDS_HEIGHT: f32 = 50.0;

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
        .insert_resource(Mode::Addition)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                update_cell_count,
                update_cords_text,
                update_world_cords
                    .before(update_cell_count)
                    .before(update_cords_text),
                update_mode,
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

    let background_size = Vec2::new(WINDOW_WIDTH, CORDS_HEIGHT);

    // Calculate the position for the gray background sprite
    let background_position = Vec3::new(WINDOW_WIDTH / 2.0, CORDS_HEIGHT / 2.0, 0.0);

    // Spawn the gray background sprite
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),   // Set the color to gray
                custom_size: Some(background_size), // Set the custom size
                ..default()
            },
            transform: Transform::from_translation(background_position),
            ..default()
        })
        .with_children(|parent| {
            // Spawn the text in the middle of the gray background
            parent
                .spawn(Text2dBundle {
                    text: Text::from_section(
                        "(0, 0)", // Text to be displayed
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE, // Set text color to white for contrast
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 0.0, 1.0), // Position the text on top of the background
                    ..default()
                })
                .insert(CordsText);
        });

    assert!(BRICK_SIZE.x > 0.0);
    assert!(BRICK_SIZE.y > 0.0);

    let cells_width = WINDOW_WIDTH;
    let cells_height = WINDOW_HEIGHT - CORDS_HEIGHT;

    let n_columns = (cells_width / BRICK_SIZE.x).floor() as usize;
    let n_rows = (cells_height / BRICK_SIZE.y).floor() as usize;

    let total_brick_width = n_columns as f32 * BRICK_SIZE.x;
    let total_brick_height = n_rows as f32 * BRICK_SIZE.y;

    let start_x = (cells_width - total_brick_width) / 2.0;
    let start_y = (cells_height - total_brick_height) / 2.0 + CORDS_HEIGHT;

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
                .with_children(|parent| {
                    parent.spawn(Text2dBundle {
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
                .insert(Cell {
                    count: 1,
                    row,
                    column,
                })
                .insert(Name::new(format!("Cell {}-{}", row, column)));
        }
    }
}

fn update_cell_count(
    mut bricks: Query<(&mut Sprite, &mut Cell, &Children, &GlobalTransform)>,
    mut texts: Query<&mut Text>,
    mouse: Res<Input<MouseButton>>,
    mycoords: Res<MyWorldCoords>,
    mode: Res<Mode>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // Convert the stored cursor world position into a Vec3 for comparison
        let cursor_position = mycoords.0.extend(0.0);

        // Iterate over the bricks to find the one that contains the cursor
        for (_sprite, mut cell, children, transform) in bricks.iter_mut() {
            // Check if the cursor is within the bounds of this cell's sprite
            if is_cursor_in_cell(cursor_position, transform) {
                // Update the cell's count and text

                match *mode {
                    Mode::Addition => cell.count += 1,
                    Mode::Subtraction => cell.count -= 1,
                }

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

fn update_mode(keys: Res<Input<KeyCode>>, mut mode: ResMut<Mode>) {
    if keys.just_pressed(KeyCode::Space) {
        mode.toggle();
    }
}

fn update_cords_text(
    mut cord_text: Query<&mut Text, With<CordsText>>,
    mut bricks: Query<(&Cell, &GlobalTransform)>,
    mycoords: Res<MyWorldCoords>,
    mode: Res<Mode>,
) {
    // Convert the stored cursor world position into a Vec3 for comparison
    let cursor_position = mycoords.0.extend(0.0);

    // Assume the cursor is not over any cell initially
    let mut cell_cords = None;

    // Iterate over the bricks to find if the cursor is over any cell
    for (cell, transform) in bricks.iter_mut() {
        if is_cursor_in_cell(cursor_position, transform) {
            // If the cursor is in a cell, store the cell's grid coordinates
            cell_cords = Some((cell.row, cell.column));
            break; // No need to check other cells
        }
    }

    // Iterate over the text entities to update the text
    let mut text = cord_text.single_mut();
    // Update the text to show cell coordinates
    if let Some((row, column)) = cell_cords {
        text.sections[0].value = format!("({},{})", row, column);
    }

    match *mode {
        Mode::Addition => text.sections[0].style.color = Color::WHITE,
        Mode::Subtraction => text.sections[0].style.color = Color::RED,
    }
}

#[derive(Component)]
struct Cell {
    count: i16,
    row: usize,
    column: usize,
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct CordsText;

#[derive(Resource)]
enum Mode {
    Addition,
    Subtraction,
}

impl Mode {
    fn toggle(&mut self) {
        match self {
            Mode::Addition => *self = Mode::Subtraction,
            Mode::Subtraction => *self = Mode::Addition,
        }
    }
}
