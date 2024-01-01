use crate::constant::TIME_SCALE;
use bevy::prelude::*;

pub trait Planets {
    fn radius(&self) -> f32;
    fn distance_to_sun(&self) -> f32;
    fn rotation_velocity(&self) -> f32;
    fn orbital_velocity(&self) -> f32;
    fn name(&self) -> &str;
}

#[derive(Component)]
pub struct CommonPlanets {
    distance_to_sun: f32,
    radius: f32,
    rotation_velocity: f32,
    orbital_velocity: f32,
    name: String,
}

impl CommonPlanets {
    pub fn new(
        distance_to_sun: f32,
        radius: f32,
        rotation_velocity: f32,
        orbital_velocity: f32,
        name: String,
    ) -> Self {
        Self {
            distance_to_sun,
            radius,
            rotation_velocity,
            orbital_velocity,
            name,
        }
    }
}

impl Planets for CommonPlanets {
    fn distance_to_sun(&self) -> f32 {
        self.distance_to_sun
    }

    fn radius(&self) -> f32 {
        self.radius
    }

    fn rotation_velocity(&self) -> f32 {
        self.rotation_velocity
    }
    fn orbital_velocity(&self) -> f32 {
        self.orbital_velocity
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Component)]
pub struct Earth {
    distance_to_sun: f32,
    radius: f32,
    rotation_velocity: f32,
    orbital_velocity: f32,
    name: String,
}

impl Earth {
    pub fn new(
        distance_to_sun: f32,
        radius: f32,
        rotation_velocity: f32,
        orbital_velocity: f32,
        name: String,
    ) -> Self {
        Self {
            distance_to_sun,
            radius,
            rotation_velocity,
            orbital_velocity,
            name,
        }
    }
}

impl Planets for Earth {
    fn distance_to_sun(&self) -> f32 {
        self.distance_to_sun
    }

    fn radius(&self) -> f32 {
        self.radius
    }

    fn rotation_velocity(&self) -> f32 {
        self.rotation_velocity
    }
    fn orbital_velocity(&self) -> f32 {
        self.orbital_velocity
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub fn move_planets<T>(time: Res<Time>, mut query: Query<(&mut Transform, &T), Without<Earth>>)
where
    T: Planets + bevy::prelude::Component,
{
    for (mut transform, planet) in &mut query {
        // let movement = transf.forward() + transf.left();
        transform.rotate_y(TIME_SCALE * planet.rotation_velocity() * time.delta_seconds());

        let elapsed_seconds = time.elapsed_seconds();
        let angle = elapsed_seconds * TIME_SCALE * planet.orbital_velocity();
        transform.translation = Vec3::new(
            planet.distance_to_sun() * angle.cos(),
            0.0,
            planet.distance_to_sun() * angle.sin(),
        );
    }
}

pub fn move_earth(time: Res<Time>, mut query: Query<(&mut Transform, &Earth)>) {
    for (mut transform, planet) in &mut query {
        // let movement = transf.forward() + transf.left();
        transform.rotate_y(TIME_SCALE * planet.rotation_velocity() * time.delta_seconds());

        let elapsed_seconds = time.elapsed_seconds();
        let angle = elapsed_seconds * TIME_SCALE * planet.orbital_velocity();
        transform.translation = Vec3::new(
            planet.distance_to_sun() * angle.cos(),
            0.0,
            planet.distance_to_sun() * angle.sin(),
        );
    }
}
