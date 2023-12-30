use crate::{constant, CameraFocus};
use bevy::prelude::*;

#[derive(Component)]
pub enum ChangeViewButton {
    Mercury,
    Venus,
    Earth,
    Moon,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Airplane,
    Global,
}

impl ChangeViewButton {
    pub fn name(&self) -> &str {
        match *self {
            ChangeViewButton::Mercury => constant::mercury::NAME,
            ChangeViewButton::Venus => constant::venus::NAME,
            ChangeViewButton::Earth => constant::earth::NAME,
            ChangeViewButton::Moon => constant::moon::NAME,
            ChangeViewButton::Mars => constant::mars::NAME,
            ChangeViewButton::Jupiter => constant::jupiter::NAME,
            ChangeViewButton::Saturn => constant::saturn::NAME,
            ChangeViewButton::Uranus => constant::uranus::NAME,
            ChangeViewButton::Neptune => constant::neptune::NAME,
            ChangeViewButton::Airplane => constant::airplane::NAME,
            ChangeViewButton::Global => "Global",
        }
    }
}

pub fn setup_button(mut commands: Commands) {
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
            let btns = vec![
                ChangeViewButton::Mercury,
                ChangeViewButton::Venus,
                ChangeViewButton::Earth,
                ChangeViewButton::Moon,
                ChangeViewButton::Mars,
                ChangeViewButton::Jupiter,
                ChangeViewButton::Saturn,
                ChangeViewButton::Uranus,
                ChangeViewButton::Neptune,
                ChangeViewButton::Airplane,
            ];
            for btn in btns {
                add_button(parent, btn);
            }
        });
}

fn add_button(cmd: &mut ChildBuilder, btn: ChangeViewButton) {
    let name = String::from(btn.name());
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
            &ChangeViewButton,
        ),
        Changed<Interaction>,
    >,
    mut camera_focus: ResMut<CameraFocus>,
) {
    for (interaction, mut color, mut border_color, btn) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                // *color = Color::rgb(0.35, 0.75, 0.35).into();
                border_color.0 = Color::RED;
                camera_focus.focus = btn.name().into();
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
        camera_focus.focus = ChangeViewButton::Global.name().into();
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
                camera_focus.focus = ChangeViewButton::Global.name().into();
                // println!(
                //     "Scroll (pixel units): vertical: {}, horizontal: {}",
                //     ev.y, ev.x
                // );
            }
        }
    }
}
