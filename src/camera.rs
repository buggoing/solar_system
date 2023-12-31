use bevy::{math::vec3, prelude::*};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{
    airplane::Airplane,
    constant,
    planets::{CommonPlanets, Earth, Planets},
    CameraFocus, Moon,
};

pub fn control_camera(
    camera_focus: Res<CameraFocus>,
    mut camera: Query<(&mut PanOrbitCamera, &mut Transform)>,
    earth: Query<(&Earth, &Transform), Without<PanOrbitCamera>>,
    moon: Query<(&Moon, &Transform), Without<PanOrbitCamera>>,
    planets: Query<(&CommonPlanets, &Transform), Without<PanOrbitCamera>>,
    airplane: Query<(&Airplane, &Transform), Without<PanOrbitCamera>>,
) {
    let mut target = Vec3::ZERO;
    match camera_focus.focus.as_str() {
        constant::earth::NAME => {
            let earth = earth.single();
            target += earth.1.translation;
        }
        constant::moon::NAME => {
            let moon = moon.single();
            target += moon.1.translation;
        }
        constant::airplane::NAME => {
            let plane = airplane.single();
            target = plane.1.translation;
            let mut camera = camera.single_mut();
            // camera.1.translation = target + vec3(10., 10., 10.);
            camera.0.target_focus = target;
            return;
        }
        "Global" => {
            return;
        }
        _ => {
            // let plane = query.single();
            for (planet, transform) in &planets {
                if planet.name() == camera_focus.focus {
                    target += transform.translation;
                }
            }
        }
    }
    // let mut camera = set.p0();
    let mut camera = camera.single_mut();
    let delta_translation = Vec3::new(50.0, 50.0, 50.0);
    // camera.1.translation = target + delta_translation;
    camera.0.target_focus = target;
}
