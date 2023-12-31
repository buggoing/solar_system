use std::f32::consts::PI;

use crate::constant::{self, SPACE_SCALE};
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    math::vec3,
    prelude::*,
};

#[derive(Component)]
pub struct Airplane {
    velocity: f32,
    direction: Vec2,
}

impl Airplane {
    fn new(volecity: f32) -> Self {
        Airplane {
            velocity: volecity,
            direction: Vec2::new(0.0, PI / 2.),
        }
    }
}

pub fn set_plane(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Airplane.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::earth::DISTANCE_TO_SUN * SPACE_SCALE + 100.,
                0.,
                0.,
            ),
            ..default()
        },
        Airplane::new(5.),
    ));
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane {
    //         size: constant::earth::DISTANCE_TO_SUN * SPACE_SCALE + 100.,
    //         ..Default::default()
    //     })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
}

pub fn control_airplane(time: Res<Time>, mut query: Query<(&mut Transform, &Airplane)>) {
    for (mut transform, plane) in &mut query {
        let delta = plane.velocity * time.delta_seconds();
        transform.translation.y += delta * plane.direction.y.cos();
        transform.translation.x += delta * plane.direction.y.sin() * plane.direction.x.cos();
        transform.translation.z += delta * plane.direction.y.sin() * plane.direction.x.sin();
    }
}

pub fn airplane_direction(
    mut key_evr: EventReader<KeyboardInput>,
    mut query: Query<(&mut Transform, &mut Airplane)>,
) {
    for ev in key_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                let mut plane = query.single_mut();
                let delta_angle = PI / 4.0;
                match ev.key_code {
                    Some(KeyCode::Left) => {
                        plane.1.direction.x -= delta_angle;
                        plane.0.rotate_y(delta_angle);
                    }
                    Some(KeyCode::Right) | Some(KeyCode::D) => {
                        plane.1.direction.x += delta_angle;
                        plane.0.rotate_y(-delta_angle);
                    }
                    Some(KeyCode::Up) | Some(KeyCode::W) => {
                        plane.1.direction.y -= delta_angle;
                        if plane.1.direction.x.sin() >= 0.0 {
                            plane.0.rotate_z(delta_angle);
                        } else {
                            plane.0.rotate_z(-delta_angle);
                        }
                    }
                    Some(KeyCode::Down) | Some(KeyCode::S) => {
                        plane.1.direction.y += delta_angle;
                        if plane.1.direction.x.sin() >= 0.0 {
                            plane.0.rotate_z(-delta_angle);
                        } else {
                            plane.0.rotate_z(delta_angle);
                        }
                    }
                    _ => {}
                };
            }
            ButtonState::Released => {}
        }
    }
}

#[derive(Component, Default)]
pub struct Bullet {
    velocity: f32,
    direction: Vec2,
    distance: f32,
}
const MAX_BULLET_DISTANCE: f32 = 100.;

impl Bullet {
    fn new(velocity: f32, direction: Vec2) -> Self {
        Bullet {
            velocity,
            direction,
            ..default()
        }
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Airplane, &Transform)>,
    keys: ResMut<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::A) {
        let plane = query.single();
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("Bullet.glb#Scene0"),
                transform: plane.1.with_scale(Vec3::splat(0.1)),
                ..default()
            },
            Bullet::new(plane.0.velocity + 10., plane.0.direction),
        ));
        // keys.reset(KeyCode::A);
    }
}

pub fn control_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Bullet, Entity)>,
) {
    for (mut transform, mut bullet, entity) in &mut query {
        let delta = bullet.velocity * time.delta_seconds();
        transform.translation.y += delta * bullet.direction.y.cos();
        transform.translation.x += delta * bullet.direction.y.sin() * bullet.direction.x.cos();
        transform.translation.z += delta * bullet.direction.y.sin() * bullet.direction.x.sin();
        bullet.distance += delta;
        if bullet.distance >= MAX_BULLET_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}
