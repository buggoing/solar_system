use std::f32::consts::PI;

pub const ONE_DAY_SECONDS: i32 = 24 * 60 * 60;

pub mod Earth {
    use super::*;
    use std::f32::consts::PI;

    pub const VELCITY: f32 = 2.0 * PI / (365 * ONE_DAY_SECONDS) as f32;
    pub const RADIUS: f32 = 6371.0;
}

pub mod Moon {
    use super::*;
    use std::f32::consts::PI;
    pub const VELCITY: f32 = 2.0 * PI / ONE_DAY_SECONDS as f32;
    pub const RADIUS: f32 = 1737.4;
    pub const DISTANCE_TO_EARTH: f32 = 384400.0;
}

pub const TIME_SCALE: f32 = (24 * 60 * 6) as f32; // 10s => 1day
pub const SPACE_SCALE: f32 = 1.0 / Earth::RADIUS;
