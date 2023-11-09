use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 650.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component, Debug)]
enum Clickable {
    Square {
        start_x: f32,
        start_y: f32,
        width: f32,
    },
    Circle {
        start_x: f32,
        start_y: f32,
        radius: f32,
    },
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

#[derive(Resource, Default)]
struct Points(u32);

impl Clickable {
    fn is_inside_bounds(&self, location: Vec2) -> bool {
        match self {
            Clickable::Square {
                start_x,
                start_y,
                width,
            } => {
                location.x >= *start_x
                    && location.x <= *start_x + *width
                    && location.y >= *start_y
                    && location.y <= *start_y + *width
            }
            Clickable::Circle {
                start_x,
                start_y,
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
        .init_resource::<Points>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                handle_clicks,
                update_score,
                update_score.after(handle_clicks),
            ),
        )
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

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec2::new(0., 0.).extend(0.0)),
            ..default()
        })
        .insert((
            Clickable::Square {
                start_x: 0.,
                start_y: 0.,
                width: 100.,
            },
            ColorChangingCell {
                state: CellColor::Orange,
            },
            ScoreGranter { grants: 1 },
        ));

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

// checks for click, checks each clickable if it was clicked,
// if so then updates the score and changes the color of the cell
fn handle_clicks(
    mut q_cells: Query<(
        &Clickable,
        &mut ColorChangingCell,
        &mut Sprite,
        &ScoreGranter,
    )>,
    mut points: ResMut<Points>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if let Some(cursor_position) = q_window.single().cursor_position() {
        // flip vertical axis, so that y is up
        let cursor_position = Vec2::new(cursor_position.x, WINDOW_HEIGHT - cursor_position.y);
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for (clickable, mut cell, mut sprite, score_granter) in q_cells.iter_mut() {
                if clickable.is_inside_bounds(cursor_position) {
                    match cell.state {
                        CellColor::Orange => {
                            cell.state = CellColor::Blue;
                            sprite.color = Color::BLUE;
                            points.0 += score_granter.grants;
                        }
                        CellColor::Blue => {
                            cell.state = CellColor::Orange;
                            sprite.color = Color::ORANGE;
                        }
                    }
                }
            }
        }
    }
}

// updates score text to match the current score
fn update_score(mut q_text: Query<&mut Text>, points: Res<Points>) {
    let mut text = q_text.single_mut();

    text.sections[0].value = format!("Points: {}", points.0);
}
