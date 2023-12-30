use crate::constant;
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    math::vec3,
    prelude::*,
};

#[derive(Component)]
pub struct Airplane {
    volecity: f32,
    direction: Vec3,
}

impl Airplane {
    fn new(volecity: f32) -> Self {
        Airplane {
            volecity: volecity,
            direction: Vec3::new(1.0, 0., 0.),
        }
    }
}

pub fn set_plane(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Airplane.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::earth::DISTANCE_TO_SUN / 100000.0 + 100.,
                0.,
                0.,
            ),
            ..default()
        },
        Airplane::new(5.),
    ));
}

pub fn airplane_control(time: Res<Time>, mut query: Query<(&mut Transform, &Airplane)>) {
    for (mut transform, plane) in &mut query {
        transform.translation += plane.direction * plane.volecity * time.delta_seconds();
    }
}

pub fn airplane_direction(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<(&mut Transform, &mut Airplane)>,
) {
    for ev in key_evr.read() {
        match ev.state {
            ButtonState::Released => {
                let mut plane = query.single_mut();

                match ev.key_code {
                    Some(KeyCode::Left) | Some(KeyCode::A) => {
                        plane.1.direction = vec3(-1.0, 0.0, 0.0);
                    }
                    Some(KeyCode::Right) | Some(KeyCode::D) => {
                        plane.1.direction = vec3(1.0, 0.0, 0.0);
                    }
                    Some(KeyCode::Up) | Some(KeyCode::W) => {
                        plane.1.direction = vec3(0.0, 0.0, -1.0)
                    }
                    Some(KeyCode::Down) | Some(KeyCode::S) => {
                        plane.1.direction = vec3(0.0, 0.0, 1.0)
                    }
                    Some(KeyCode::Q) => {
                        plane.1.direction = vec3(0.0, 1.0, 0.0);
                    }
                    Some(KeyCode::E) => {
                        plane.1.direction = vec3(1.0, -1.0, 0.0);
                    }
                    _ => {
                        println!("unknown keycode: {:?}", ev.key_code);
                    }
                }
            }
            ButtonState::Pressed => {}
        }
    }
}
