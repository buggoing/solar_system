//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

pub mod airplane;
pub mod button;
pub mod constant;

use std::{f32::consts::PI, ops::Mul};

use airplane::{airplane_control, airplane_direction, set_plane, Airplane};
use bevy::{
    core::Zeroable,
    ecs::query,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::WindowMode,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use button::{button_handler, mouse_button_input, scroll_events, touchpad_gestures};
use constant::{earth::DISTANCE_TO_SUN, moon::DISTANCE_TO_EARTH, SPACE_SCALE, TIME_SCALE};

#[derive(Component)]
struct Mercury {
    distance_to_sun: f32,
}

#[derive(Component)]
struct Venus {
    distance_to_sun: f32,
}

#[derive(Component)]
struct Earth {
    distance_to_sun: f32,
}

#[derive(Component)]
struct Moon {
    radius: f32,
    distance_to_earth: f32,
}

#[derive(Component)]
struct Mars {
    distance_to_sun: f32,
}

#[derive(Component)]
struct Jupiter {
    distance_to_sun: f32,
}

#[derive(Component)]
struct Saturn {
    distance_to_sun: f32,
}

#[derive(Component)]
struct Uranus {
    distance_to_sun: f32,
}

#[derive(Component)]
struct Neptune {
    distance_to_sun: f32,
}

#[derive(Resource)]
pub struct CameraFocus {
    focus: CameraFocusType,
}
enum CameraFocusType {
    Earth,
    Moon,
    Global,
    Airplane,
    Uranus,
    Neptune,
}

const PLANET_DISTANCE_TO_SUN_SCALE: f32 = 100000.0;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Solar System".into(),
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    };

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
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_systems(Startup, (setup, button::setup_button, set_plane))
        .add_systems(
            Update,
            (
                move_mercury,
                move_moon,
                move_earth,
                axis,
                button_handler,
                camera_control,
                mouse_button_input,
                touchpad_gestures,
                scroll_events,
                airplane_control,
                airplane_direction,
            ),
        )
        .run()
    // #[cfg(debug_assertions)] // debug/dev builds only
    // {
    //     use bevy::diagnostic::LogDiagnosticsPlugin;
    //     app.add_plugins(LogDiagnosticsPlugin::default());
    // }
}

fn move_moon(
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut query: Query<(&mut Transform, &Moon)>,
    earth_query: Query<(&Transform, &Earth), Without<Moon>>,
) {
    let earth = earth_query.single();
    // moon orbit
    gizmos
        .circle(
            earth.0.translation,
            Vec3::Y,
            constant::moon::DISTANCE_TO_EARTH * SPACE_SCALE,
            Color::GRAY,
        )
        .segments(128);

    for (mut transform, moon) in &mut query {
        // let movement = transf.forward() + transf.left();
        transform.rotate_y(
            constant::TIME_SCALE * constant::moon::ROTATION_VELCITY * time.delta_seconds(),
        );

        let elapsed_seconds = time.elapsed_seconds();
        let angle = elapsed_seconds * constant::TIME_SCALE * constant::moon::ORBITAL_VELCITY;
        transform.translation = earth.0.translation
            + Vec3::new(
                moon.distance_to_earth * angle.cos(),
                0.0,
                moon.distance_to_earth * angle.sin(),
            );
    }
}

fn move_earth(time: Res<Time>, mut query: Query<(&mut Transform, &Earth)>) {
    for (mut transform, earth) in &mut query {
        // let movement = transf.forward() + transf.left();
        transform.rotate_y(TIME_SCALE * constant::earth::ROTATION_VELCITY * time.delta_seconds());

        let elapsed_seconds = time.elapsed_seconds();
        let angle = elapsed_seconds * TIME_SCALE * constant::earth::ORBITAL_VELCITY;
        transform.translation = Vec3::new(
            earth.distance_to_sun * angle.cos(),
            0.0,
            earth.distance_to_sun * angle.sin(),
        );
    }
}

fn move_mercury(time: Res<Time>, mut query: Query<(&mut Transform, &Mercury)>) {
    for (mut transform, mercury) in &mut query {
        // let movement = transf.forward() + transf.left();
        transform.rotate_y(TIME_SCALE * constant::mercury::ROTATION_VELCITY * time.delta_seconds());

        let elapsed_seconds = time.elapsed_seconds();
        let angle = elapsed_seconds * TIME_SCALE * constant::mercury::ORBITAL_VELCITY;
        transform.translation = Vec3::new(
            mercury.distance_to_sun * angle.cos(),
            0.0,
            mercury.distance_to_sun * angle.sin(),
        );
    }
}

// fn move_factory(entity: &impl Planet) -> impl Fn(Res<Time>, Query<(&mut Transform, Planet>)) {
//     fn move_planets(time: Res<Time>, mut query: Query<(&mut Transform, &impl Planet)>) {}
//     return move_planets;
// }

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // commands
    //     .spawn(PointLightBundle {
    //         point_light: PointLight {
    //             intensity: 9000.0,
    //             // range: 300.,
    //             // radius: 60.0,
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..default()
    //     })
    //     .with_children(|builder| {
    //         let scene = asset_server.load("sun.glb#Scene0");
    //         builder.spawn((SceneBundle {
    //             scene: scene,
    //             transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(50.)),
    //             ..default()
    //         },));
    //     });

    // Sun
    commands.spawn((SceneBundle {
        scene: asset_server.load("sun.glb#Scene0"),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(1. / 5.)),
        ..default()
    },));

    // Mercury
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("mercury.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::mercury::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(0.02)),
            ..default()
        },
        Mercury {
            distance_to_sun: constant::mercury::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
        },
    ));

    // Venus
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("venus.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::venus::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(0.02)),
            ..default()
        },
        Venus {
            distance_to_sun: constant::venus::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
        },
    ));

    // Earth
    let es = asset_server.load("earth.glb#Scene0");
    commands.spawn((
        SceneBundle {
            scene: es,
            transform: Transform::from_xyz(
                constant::earth::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(0.02)),
            ..default()
        },
        Earth {
            distance_to_sun: constant::earth::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
        },
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
            transform: Transform::from_xyz(100.0 + DISTANCE_TO_EARTH * SPACE_SCALE, 0., 0.)
                .with_scale(Vec3::splat(1.0 / 120.)),
            ..default()
        },
        Moon {
            radius: constant::moon::RADIUS * constant::SPACE_SCALE,
            distance_to_earth: constant::moon::DISTANCE_TO_EARTH * constant::SPACE_SCALE,
        },
    ));

    // Mars
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("mars.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::mars::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(0.02)),
            ..default()
        },
        Mars {
            distance_to_sun: constant::mars::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
        },
    ));

    // Jupiter
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Jupiter.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::jupiter::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(0.02)),
            ..default()
        },
        Jupiter {
            distance_to_sun: constant::jupiter::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
        },
    ));

    // Saturn
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Saturn.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::saturn::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(0.02)),
            ..default()
        },
        Saturn {
            distance_to_sun: constant::saturn::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
        },
    ));

    // Uranus
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Uranus.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::uranus::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(0.02)),
            ..default()
        },
        Uranus {
            distance_to_sun: constant::uranus::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
        },
    ));

    // Neptune
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Neptune.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::neptune::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(0.02)),
            ..default()
        },
        Neptune {
            distance_to_sun: constant::neptune::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
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

fn axis(mut gizmos: Gizmos, time: Res<Time>, query: Query<&Transform, &Earth>) {
    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::mercury::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
            Color::SILVER,
        )
        .segments(128);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::venus::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
            Color::OLIVE,
        )
        .segments(128);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::earth::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
            Color::rgb_u8(70, 130, 180),
        )
        .segments(128);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::mars::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
            Color::rgb_u8(232, 155, 0), // Yellow Ochre
        )
        .segments(128);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::jupiter::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
            Color::SILVER,
        )
        .segments(128);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::saturn::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
            Color::SILVER,
        )
        .segments(128);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::uranus::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
            Color::rgb_u8(32, 178, 170),
        )
        .segments(128);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::neptune::DISTANCE_TO_SUN / PLANET_DISTANCE_TO_SUN_SCALE,
            // Color::rgb_u8(0, 191, 255),
            Color::BLUE,
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
        Query<(&Transform, &Airplane)>,
        Query<(&Transform, &Uranus)>,
        Query<(&Transform, &Neptune)>,
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
        CameraFocusType::Airplane => {
            let query = set.p3();
            let plane = query.single();
            target += plane.0.translation;
        }
        CameraFocusType::Uranus => {
            let query = set.p4();
            let plane = query.single();
            target += plane.0.translation;
        }
        CameraFocusType::Neptune => {
            let query = set.p5();
            let plane = query.single();
            target += plane.0.translation;
        }
        CameraFocusType::Global => return,
    }
    let mut camera = set.p0();
    let mut camera = camera.single_mut();
    *camera.1 = Transform::from_translation(target + Vec3::new(50.0, 50.0, 50.0))
        .looking_at(target, Vec3::Y);
}
