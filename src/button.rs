use bevy::{ecs::query, prelude::*};

use crate::airplane::Airplane;
use crate::{CameraFocus, CameraFocusType};
#[derive(Component)]
pub struct MoonButton;

#[derive(Component)]
pub struct EarthButton;

#[derive(Component)]
pub enum ChangeViewButton {
    Earth,
    Moon,
    Global,
    Airplane,
    Uranus,
    Neptune,
}

impl ChangeViewButton {
    fn name(&self) -> String {
        match *self {
            ChangeViewButton::Earth => "Earth".into(),
            ChangeViewButton::Moon => "Moon".into(),
            ChangeViewButton::Airplane => "Airplane".into(),
            ChangeViewButton::Uranus => "Uranus".into(),
            ChangeViewButton::Neptune => "Neptune".into(),
            _ => "Unknown".into(),
        }
    }
}

pub fn setup_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::GRAY),
                        background_color: Color::BLACK.into(),
                        ..default()
                    },
                    ChangeViewButton::Earth,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Earth",
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            add_button(parent, ChangeViewButton::Moon);
        })
        .with_children(|parent| {
            add_button(parent, ChangeViewButton::Airplane);
        })
        .with_children(|parent| {
            add_button(parent, ChangeViewButton::Uranus);
        })
        .with_children(|parent| {
            add_button(parent, ChangeViewButton::Neptune);
        });
}

fn add_button(cmd: &mut ChildBuilder, btn: ChangeViewButton) {
    let name = btn.name();
    cmd.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::GRAY),
            background_color: Color::BLACK.into(),
            ..default()
        },
        btn,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            name,
            TextStyle {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        ));
    });
}

pub fn button_handler(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &ChangeViewButton,
        ),
        Changed<Interaction>,
    >,
    mut camera_focus: ResMut<CameraFocus>,
) {
    for (interaction, mut color, mut border_color, children, btn) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                // *color = Color::rgb(0.35, 0.75, 0.35).into();
                border_color.0 = Color::RED;
                match btn {
                    ChangeViewButton::Earth => {
                        camera_focus.focus = CameraFocusType::Earth;
                    }
                    ChangeViewButton::Moon => {
                        camera_focus.focus = CameraFocusType::Moon;
                    }
                    ChangeViewButton::Global => {
                        camera_focus.focus = CameraFocusType::Global;
                    }
                    ChangeViewButton::Airplane => camera_focus.focus = CameraFocusType::Airplane,
                    ChangeViewButton::Uranus => camera_focus.focus = CameraFocusType::Uranus,
                    ChangeViewButton::Neptune => camera_focus.focus = CameraFocusType::Neptune,
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = Color::BLACK.into();
                border_color.0 = Color::GRAY;
            }
        }
    }
}

pub fn mouse_button_input(buttons: Res<Input<MouseButton>>, mut camera_focus: ResMut<CameraFocus>) {
    if buttons.pressed(MouseButton::Right) {
        camera_focus.focus = CameraFocusType::Global;
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Right, MouseButton::Middle]) {
        println!("mouse pressed");
    }
}

use bevy::input::touchpad::{TouchpadMagnify, TouchpadRotate};

// these only work on macOS
pub fn touchpad_gestures(
    mut evr_touchpad_magnify: EventReader<TouchpadMagnify>,
    mut evr_touchpad_rotate: EventReader<TouchpadRotate>,
) {
    for ev_magnify in evr_touchpad_magnify.read() {
        // Positive numbers are zooming in
        // Negative numbers are zooming out
        println!("Touchpad zoom by {}", ev_magnify.0);
    }
    for ev_rotate in evr_touchpad_rotate.read() {
        // Positive numbers are anticlockwise
        // Negative numbers are clockwise
        println!("Touchpad rotate by {}", ev_rotate.0);
    }
}

use bevy::input::mouse::MouseWheel;

pub fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera_focus: ResMut<CameraFocus>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!(
                    "Scroll (line units): vertical: {}, horizontal: {}",
                    ev.y, ev.x
                );
            }
            MouseScrollUnit::Pixel => {
                camera_focus.focus = CameraFocusType::Global;
                // println!(
                //     "Scroll (pixel units): vertical: {}, horizontal: {}",
                //     ev.y, ev.x
                // );
            }
        }
    }
}
