use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 650.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const GRID_START: Vec2 = Vec2::new(450., 400.);
const GRID_CELL_SIZE: f32 = 50.;

#[derive(Component, Debug)]
enum Clickable {
    Square { location: Vec2, size: f32 },
    Circle { location: Vec2, radius: f32 },
}

#[derive(Component)]
struct ScoreGranter {
    grants: u32,
}

#[derive(Component)]
struct ColorChangingCell {
    state: CellColor,
}

enum CellColor {
    Orange,
    Blue,
}

#[derive(Resource)]
struct Points(u32);

#[derive(Resource, Default)]
struct MouseCords(Vec2);

impl Clickable {
    fn is_inside_bounds(&self, location: Vec2) -> bool {
        match self {
            Clickable::Square {
                location:
                    Vec2 {
                        x: start_x,
                        y: start_y,
                    },
                size,
            } => {
                location.x >= *start_x
                    && location.x <= *start_x + *size
                    && location.y >= *start_y
                    && location.y <= *start_y + *size
            }
            Clickable::Circle {
                location:
                    Vec2 {
                        x: start_x,
                        y: start_y,
                    },
                radius,
            } => {
                let distance = (location.x - *start_x).powi(2) + (location.y - *start_y).powi(2);
                distance <= radius.powi(2)
            }
        }
    }
}

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
        .insert_resource(Points(50))
        .init_resource::<MouseCords>()
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, update_mouse_cords)
        .add_systems(
            Update,
            (bevy::window::close_on_esc, handle_clicks, handle_reset),
        )
        .add_systems(PostUpdate, update_score)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn camera
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

    // spawn grid
    for x in 0..4 {
        for y in 0..4 {
            let starting_cords =
                GRID_START + Vec2::new(x as f32 * GRID_CELL_SIZE, y as f32 * GRID_CELL_SIZE);

            let in_outer_ring = x == 0 || x == 3 || y == 0 || y == 3;

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: if in_outer_ring {
                            Color::BLUE
                        } else {
                            Color::ORANGE
                        },
                        custom_size: Some(Vec2::new(GRID_CELL_SIZE, GRID_CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(starting_cords.extend(0.0)),
                    ..default()
                })
                .insert((
                    Clickable::Square {
                        location: starting_cords,
                        size: GRID_CELL_SIZE,
                    },
                    ColorChangingCell {
                        state: CellColor::Orange,
                    },
                    ScoreGranter { grants: 5 },
                ));
        }
    }

    // spawn circle

    let circle_location = Vec2::new(200., 200.);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            transform: Transform::from_translation(circle_location.extend(0.0)),
            ..default()
        })
        .insert((
            Clickable::Circle {
                location: circle_location,
                radius: 50.,
            },
            ScoreGranter { grants: 10 },
        ));

    // spawn points text
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(
                "Points: 0",
                TextStyle {
                    font_size: 42.0,
                    color: Color::BLACK,
                    ..default()
                },
            )],
            ..default()
        },
        // ensure the text is drawn on top of the box
        transform: Transform::from_translation(Vec3::new(200., 100., 1.)),
        ..default()
    });
}

fn update_mouse_cords(
    mut mouse_cords: ResMut<MouseCords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(cursor_position) = q_window.single().cursor_position() {
        // flip vertical axis, so that y is up
        let cursor_position = Vec2::new(cursor_position.x, WINDOW_HEIGHT - cursor_position.y);
        mouse_cords.0 = cursor_position;
    }
}

// checks for click, checks each clickable if it was clicked,
// if so then updates the score and changes the color of the cell
fn handle_clicks(
    mut q_cells: Query<(
        &Clickable,
        Option<&mut ColorChangingCell>,
        Option<&mut Sprite>,
        &ScoreGranter,
    )>,
    mut points: ResMut<Points>,
    cursor_position: Res<MouseCords>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (clickable, opt_cell, opt_sprite, score_granter) in q_cells.iter_mut() {
            if clickable.is_inside_bounds(cursor_position.0) {
                match opt_cell {
                    // is color changing cell (square)
                    Some(mut cell) => match cell.state {
                        CellColor::Orange => {
                            cell.state = CellColor::Blue;
                            opt_sprite.expect("square always has sprite").color = Color::BLUE;
                            points.0 += score_granter.grants;
                        }
                        CellColor::Blue => {
                            cell.state = CellColor::Orange;
                            opt_sprite.expect("square always has sprite").color = Color::ORANGE;
                        }
                    },
                    // is not color changing cell (circle)
                    None => {
                        points.0 += score_granter.grants;
                    }
                }
            }
        }
    }
}

// checks if 'r' was pressed, if so then resets the score back to 50
fn handle_reset(mut points: ResMut<Points>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::R) {
        points.0 = 50;
    }
}

// updates score text to match the current score
fn update_score(mut q_text: Query<&mut Text>, points: Res<Points>) {
    let mut text = q_text.single_mut();

    text.sections[0].value = format!("Points: {}", points.0);
}
