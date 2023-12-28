//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

pub mod button;
pub mod constant;
use std::{f32::consts::PI, ops::Mul};

use bevy::{
    core::Zeroable,
    ecs::query,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use button::{button_handler, mouse_button_input, scroll_events, touchpad_gestures};
#[derive(Component)]
struct Earth;

#[derive(Component)]
struct Moon {
    radius: f32,
    distance_to_earth: f32,
}

#[derive(Resource)]
pub struct CameraFocus {
    focus: CameraFocusType,
}
enum CameraFocusType {
    Earth,
    Moon,
    Global,
}

fn main() {
    App::new()
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .insert_resource(CameraFocus {
            focus: CameraFocusType::Global,
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, button::setup_button))
        .add_systems(
            Update,
            (
                move_moon,
                move_earth,
                axis,
                button_handler,
                camera_control,
                mouse_button_input,
                touchpad_gestures,
                scroll_events,
            ),
        )
        .run();
}

fn move_moon(time: Res<Time>, mut query: Query<(&mut Transform, &Moon)>) {
    for (mut transform, moon) in &mut query {
        // let movement = transf.forward() + transf.left();
        transform.rotate_y(
            constant::TIME_SCALE * constant::Moon::ROTATION_VELCITY * time.delta_seconds(),
        );

        let elapsed_seconds = time.elapsed_seconds();
        let angle = elapsed_seconds * constant::TIME_SCALE * constant::Moon::ORBITAL_VELCITY;
        transform.translation = Vec3 {
            x: moon.distance_to_earth * angle.cos(),
            y: 0.0,
            z: moon.distance_to_earth * angle.sin(),
        };
    }
}

fn move_earth(time: Res<Time>, mut query: Query<&mut Transform, With<Earth>>) {
    let mut transf = query.single_mut();
    transf
        .rotate_y(constant::TIME_SCALE * constant::Earth::ROTATION_VELCITY * time.delta_seconds());

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
    let es = asset_server.load("earth.glb#Scene0");
    println!("{:?}", es);
    commands.spawn((
        SceneBundle {
            scene: es,
            transform: Transform::from_xyz(0., 0., 0.),
            // .with_scale(Vec3::splat(constant::SPACE_SCALE)),
            ..default()
        },
        Earth,
    ));
    // .with_children(|parent| {
    //     // child cube
    //     parent.spawn((
    //         SceneBundle {
    //             scene: asset_server.load("moon.glb#Scene0"),
    //             transform: Transform::from_xyz(
    //                 constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
    //                 0.,
    //                 0.,
    //             ),
    //             // .with_scale(Vec3::splat(1. / constant::SPACE_SCALE)),
    //             ..default()
    //         },
    //         Moon {
    //             radius: constant::Moon::RADIUS * constant::SPACE_SCALE,
    //             distance_to_earth: constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
    //         },
    //     ));
    // });

    // Moon
    commands.spawn((
        // PbrBundle {
        //     mesh: meshes.add(
        //         shape::UVSphere {
        //             radius: constant::Moon::RADIUS * constant::SPACE_SCALE,
        //             ..default()
        //         }
        //         .into(),
        //     ),
        //     material: materials.add(Color::SILVER.into()),
        //     transform: Transform::from_xyz(
        //         constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
        //         0.0,
        //         0.0,
        //     ),
        //     ..default()
        // },
        SceneBundle {
            scene: asset_server.load("moon.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
                0.,
                0.,
            ),
            // .with_scale(Vec3::splat(0.1)),
            ..default()
        },
        Moon {
            radius: constant::Moon::RADIUS * constant::SPACE_SCALE,
            distance_to_earth: constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
        },
    ));

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

fn axis(mut gizmos: Gizmos, time: Res<Time>) {
    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::Moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
            Color::RED,
        )
        .segments(128);
    gizmos.ray(
        Vec3::new(0., 0., 0.),
        Vec3::new(-100., 0., 0.),
        Color::GREEN,
    );
    gizmos.ray(Vec3::new(0., 0., 0.), Vec3::new(100., 0., 0.), Color::GREEN);

    gizmos.ray(
        Vec3::new(0., 0., 0.),
        Vec3::new(0., -100., 0.),
        Color::GREEN,
    );
    gizmos.ray(Vec3::new(0., 0., 0.), Vec3::new(0., 100., 0.), Color::GREEN);
}

fn camera_control(
    camera_focus: Res<CameraFocus>,
    mut set: ParamSet<(
        Query<(&mut PanOrbitCamera, &mut Transform)>,
        Query<(&Transform, &Moon)>,
        Query<(&Transform, &Earth)>,
    )>,
) {
    let mut target = Vec3::ZERO;
    match camera_focus.focus {
        CameraFocusType::Earth => {
            let query = set.p2();
            let earth = query.single();
            target += earth.0.translation;
        }
        CameraFocusType::Moon => {
            let query = set.p1();
            let moon = query.single();
            target += moon.0.translation;
        }
        CameraFocusType::Global => return,
    }
    let mut camera = set.p0();
    let mut camera = camera.single_mut();
    *camera.1 =
        Transform::from_translation(target + (Vec3::splat(10.0))).looking_at(target, Vec3::Y);
}
