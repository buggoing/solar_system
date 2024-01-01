pub mod airplane;
pub mod button;
pub mod camera;
pub mod constant;
pub mod planets;

use std::sync::Arc;

use crate::constant::PLANET_GLTF_SCALE;
use crate::planets::CommonPlanets;
use crate::planets::Planets;
use airplane::{
    airplane_direction, control_airplane, control_bullet, set_plane, spawn_bullet, Airplane,
};
use bevy::asset::StrongHandle;
use bevy::scene::SceneInstance;
use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use button::{
    handle_button, mouse_button_input, scroll_events, touchpad_gestures, ChangeViewButton,
};
use camera::control_camera;
use constant::earth;
use constant::{moon::DISTANCE_TO_EARTH, SPACE_SCALE};
use planets::{move_planets, Earth};

#[derive(Component)]
pub struct Moon {
    distance_to_earth: f32,
}

#[derive(Resource)]
pub struct CameraFocus {
    focus: String,
}

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
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .insert_resource(CameraFocus {
            focus: ChangeViewButton::Global.name().into(),
        })
        .add_systems(Startup, (setup, button::setup_button, set_plane))
        .add_systems(
            Update,
            (
                move_moon,
                axis,
                handle_button,
                control_camera,
                mouse_button_input,
                touchpad_gestures,
                scroll_events,
                control_airplane,
                airplane_direction,
                move_planets::<CommonPlanets>,
                move_planets::<Earth>,
                spawn_bullet,
                control_bullet,
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        scene: asset_server.load("Sun.glb#Scene0"),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(
            PLANET_GLTF_SCALE * SPACE_SCALE * constant::sun::RADIUS,
        )),
        ..default()
    },));

    // Mercury
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Mercury.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::mercury::DISTANCE_TO_SUN * SPACE_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(
                PLANET_GLTF_SCALE * SPACE_SCALE * constant::mercury::RADIUS,
            )),
            ..default()
        },
        CommonPlanets::new(
            constant::mercury::DISTANCE_TO_SUN * SPACE_SCALE,
            constant::mercury::RADIUS,
            constant::mercury::ROTATION_VELCITY,
            constant::mercury::ORBITAL_VELCITY,
            constant::mercury::NAME.into(),
        ),
    ));

    // Venus
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Venus.glb#Scene0"),
            transform: Transform::from_xyz(constant::venus::DISTANCE_TO_SUN * SPACE_SCALE, 0., 0.)
                .with_scale(Vec3::splat(
                    PLANET_GLTF_SCALE * SPACE_SCALE * constant::venus::RADIUS,
                )),
            ..default()
        },
        CommonPlanets::new(
            constant::venus::DISTANCE_TO_SUN * SPACE_SCALE,
            constant::venus::RADIUS,
            constant::venus::ROTATION_VELCITY,
            constant::venus::ORBITAL_VELCITY,
            constant::venus::NAME.into(),
        ),
    ));

    // Earth
    let earch_x = constant::earth::DISTANCE_TO_SUN * SPACE_SCALE;
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Earth.glb#Scene0"),
            transform: Transform::from_xyz(earch_x, 0., 0.).with_scale(Vec3::splat(
                PLANET_GLTF_SCALE * SPACE_SCALE * constant::earth::RADIUS,
            )),
            ..default()
        },
        Earth::new(
            constant::earth::DISTANCE_TO_SUN * SPACE_SCALE,
            constant::earth::RADIUS,
            constant::earth::ROTATION_VELCITY,
            constant::earth::ORBITAL_VELCITY,
            constant::earth::NAME.into(),
        ),
    ));
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
            scene: asset_server.load("Moon.glb#Scene0"),
            transform: Transform::from_xyz(
                (DISTANCE_TO_EARTH + constant::earth::RADIUS) * SPACE_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(
                PLANET_GLTF_SCALE * SPACE_SCALE * constant::moon::RADIUS,
            )),
            ..default()
        },
        Moon {
            distance_to_earth: constant::moon::DISTANCE_TO_EARTH * SPACE_SCALE,
        },
    ));

    // Mars
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Mars.glb#Scene0"),
            transform: Transform::from_xyz(constant::mars::DISTANCE_TO_SUN * SPACE_SCALE, 0., 0.)
                .with_scale(Vec3::splat(
                    PLANET_GLTF_SCALE * SPACE_SCALE * constant::mars::RADIUS,
                )),
            ..default()
        },
        CommonPlanets::new(
            constant::mars::DISTANCE_TO_SUN * SPACE_SCALE,
            constant::mars::RADIUS,
            constant::mars::ROTATION_VELCITY,
            constant::mars::ORBITAL_VELCITY,
            constant::mars::NAME.into(),
        ),
    ));

    // Jupiter
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Jupiter.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::jupiter::DISTANCE_TO_SUN * SPACE_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(
                PLANET_GLTF_SCALE * SPACE_SCALE * constant::jupiter::RADIUS,
            )),
            ..default()
        },
        CommonPlanets::new(
            constant::jupiter::DISTANCE_TO_SUN * SPACE_SCALE,
            constant::jupiter::RADIUS,
            constant::jupiter::ROTATION_VELCITY,
            constant::jupiter::ORBITAL_VELCITY,
            constant::jupiter::NAME.into(),
        ),
    ));

    // Saturn
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Saturn.glb#Scene0"),
            transform: Transform::from_xyz(constant::saturn::DISTANCE_TO_SUN * SPACE_SCALE, 0., 0.)
                .with_scale(Vec3::splat(
                    PLANET_GLTF_SCALE * SPACE_SCALE * constant::saturn::RADIUS,
                )),
            ..default()
        },
        CommonPlanets::new(
            constant::saturn::DISTANCE_TO_SUN * SPACE_SCALE,
            constant::saturn::RADIUS,
            constant::saturn::ROTATION_VELCITY,
            constant::saturn::ORBITAL_VELCITY,
            constant::saturn::NAME.into(),
        ),
    ));

    // Uranus
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Uranus.glb#Scene0"),
            transform: Transform::from_xyz(constant::uranus::DISTANCE_TO_SUN * SPACE_SCALE, 0., 0.)
                .with_scale(Vec3::splat(
                    PLANET_GLTF_SCALE * SPACE_SCALE * constant::uranus::RADIUS,
                )),
            ..default()
        },
        // Uranus {
        //     distance_to_sun: constant::uranus::DISTANCE_TO_SUN * SPACE_SCALE,
        // }
        CommonPlanets::new(
            constant::uranus::DISTANCE_TO_SUN * SPACE_SCALE,
            constant::uranus::RADIUS,
            constant::uranus::ROTATION_VELCITY,
            constant::uranus::ORBITAL_VELCITY,
            constant::uranus::NAME.into(),
        ),
    ));

    // Neptune
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("Neptune.glb#Scene0"),
            transform: Transform::from_xyz(
                constant::neptune::DISTANCE_TO_SUN * SPACE_SCALE,
                0.,
                0.,
            )
            .with_scale(Vec3::splat(
                PLANET_GLTF_SCALE * SPACE_SCALE * constant::neptune::RADIUS,
            )),
            ..default()
        },
        CommonPlanets::new(
            constant::neptune::DISTANCE_TO_SUN * SPACE_SCALE,
            constant::neptune::RADIUS,
            constant::neptune::ROTATION_VELCITY,
            constant::neptune::ORBITAL_VELCITY,
            constant::neptune::NAME.into(),
        ),
    ));

    commands.spawn((
        MyCamera,
        Camera3dBundle {
            // projection: Projection::Perspective(PerspectiveProjection {
            //     far: 10000.0, // change the maximum render distance
            //     ..default()
            // }),
            transform: Transform::from_xyz(earch_x + 100.0, earch_x + 100.0, 0.0)
                .looking_at(Vec3::new(earch_x, 0.0, 0.0), Vec3::Z),
            ..default()
        },
        PanOrbitCamera { ..default() },
    ));
}

#[derive(Component)]
struct MyCamera;

fn axis(mut gizmos: Gizmos) {
    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::mercury::DISTANCE_TO_SUN * SPACE_SCALE,
            Color::SILVER,
        )
        .segments(256);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::venus::DISTANCE_TO_SUN * SPACE_SCALE,
            Color::OLIVE,
        )
        .segments(256);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::earth::DISTANCE_TO_SUN * SPACE_SCALE,
            Color::rgb_u8(70, 130, 180),
        )
        .segments(1024);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::mars::DISTANCE_TO_SUN * SPACE_SCALE,
            Color::rgb_u8(232, 155, 0), // Yellow Ochre
        )
        .segments(256);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::jupiter::DISTANCE_TO_SUN * SPACE_SCALE,
            Color::SILVER,
        )
        .segments(256);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::saturn::DISTANCE_TO_SUN * SPACE_SCALE,
            Color::SILVER,
        )
        .segments(256);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::uranus::DISTANCE_TO_SUN * SPACE_SCALE,
            Color::rgb_u8(32, 178, 170),
        )
        .segments(256);

    gizmos
        .circle(
            Vec3::ZERO,
            Vec3::Y,
            constant::neptune::DISTANCE_TO_SUN * SPACE_SCALE,
            // Color::rgb_u8(0, 191, 255),
            Color::BLUE,
        )
        .segments(256);

    gizmos.ray(
        Vec3::new(0., 0., 0.),
        Vec3::new(-100., 0., 0.),
        Color::GREEN,
    );
    gizmos.ray(
        Vec3::new(0., 0., 0.),
        Vec3::new(
            constant::earth::DISTANCE_TO_SUN * SPACE_SCALE + 100.,
            0.,
            0.,
        ),
        Color::GREEN,
    );

    gizmos.ray(
        Vec3::new(0., 0., 0.),
        Vec3::new(0., -100., 0.),
        Color::GREEN,
    );
    gizmos.ray(Vec3::new(0., 0., 0.), Vec3::new(0., 100., 0.), Color::GREEN);
}
