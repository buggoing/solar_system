//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

pub mod constant;
use std::f32::consts::PI;

use bevy::{
    core::Zeroable,
    ecs::query,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
#[derive(Component)]
struct Earth;

#[derive(Component)]
struct Moon {
    radius: f32,
    distance_to_earth: f32,
}

fn main() {
    App::new()
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_moon, move_earth))
        .run();
}

fn move_moon(time: Res<Time>, mut query: Query<(&mut Transform, &Moon)>) {
    // for (mut transform, moon) in &mut query {
    //     // let movement = transf.forward() + transf.left();
    //     // transform.rotate_y(time.elapsed_seconds() / 2.0);
    //     let elapsed_seconds = time.elapsed_seconds();
    //     let angle = elapsed_seconds * constant::TIME_SCALE * constant::Moon::VELCITY;
    //     transform.translation = Vec3 {
    //         x: moon.distance_to_earth * angle.cos(),
    //         y: 0.0,
    //         z: moon.distance_to_earth * angle.sin(),
    //     };
    // }
}

fn move_earth(time: Res<Time>, mut query: Query<&mut Transform, With<Earth>>) {
    let mut transf = query.single_mut();
    transf.rotate_y(time.delta_seconds());

    // let elapsed_seconds = time.elapsed_seconds();
    // let angle = elapsed_seconds * constant::Moon::VELCITY;
    // transf.translation = Vec3 {
    //     x: 25.0 * angle.cos(),
    //     y: 0.0,
    //     z: 25.0 * angle.sin(),
    // };
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 9000.0,
    //         range: 100.,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(8.0, 16.0, 8.0),
    //     ..default()
    // });

    // Earth
    let es = asset_server.load("earth1.glb#Scene0");
    println!("{:?}", es);
    commands
        .spawn((
            SceneBundle {
                scene: es,
                transform: Transform::from_xyz(0., 0., 0.),
                // .with_scale(Vec3::splat(0.1)),
                ..default()
            },
            Earth,
        ))
        .with_children(|parent| {
            // child cube
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load("moon.glb#Scene0"),
                    transform: Transform::from_xyz(
                        constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
                        0.,
                        0.,
                    ),
                    // .with_scale(Vec3::splat(1. / constant::SPACE_SCALE)),
                    ..default()
                },
                Moon {
                    radius: constant::Moon::RADIUS * constant::SPACE_SCALE,
                    distance_to_earth: constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
                },
            ));
        });

    // Moon
    // commands.spawn((
    //     // PbrBundle {
    //     //     mesh: meshes.add(
    //     //         shape::UVSphere {
    //     //             radius: constant::Moon::RADIUS * constant::SPACE_SCALE,
    //     //             ..default()
    //     //         }
    //     //         .into(),
    //     //     ),
    //     //     material: materials.add(Color::SILVER.into()),
    //     //     transform: Transform::from_xyz(
    //     //         constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
    //     //         0.0,
    //     //         0.0,
    //     //     ),
    //     //     ..default()
    //     // },
    //     SceneBundle {
    //         scene: asset_server.load("moon.glb#Scene0"),
    //         transform: Transform::from_xyz(
    //             constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
    //             0.,
    //             0.,
    //         ),
    //         // .with_scale(Vec3::splat(0.1)),
    //         ..default()
    //     },
    //     Moon {
    //         radius: constant::Moon::RADIUS * constant::SPACE_SCALE,
    //         distance_to_earth: constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
    //     },
    // ));

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(
    //         shape::Cylinder {
    //             height: 100.,
    //             radius: 0.1,
    //             ..default()
    //         }
    //         .into(),
    //     ),
    //     material: materials.add(Color::RED.into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(
    //         shape::Cylinder {
    //             height: 100.,
    //             radius: 0.1,
    //             ..default()
    //         }
    //         .into(),
    //     ),
    //     material: materials.add(Color::RED.into()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0)
    //         .with_rotation(Quat::from_rotation_z(-PI / 2.)),
    //     ..default()
    // });

    commands.spawn((
        Camera3dBundle {
            // projection: Projection::Perspective(PerspectiveProjection {
            //     far: 10000.0, // change the maximum render distance
            //     ..default()
            // }),
            transform: Transform::from_xyz(0.0, 100., 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
