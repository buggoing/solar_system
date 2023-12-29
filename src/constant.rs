use std::f32::consts::PI;

pub const ONE_DAY_SECONDS: f32 = (24 * 60 * 60) as f32;

pub mod mercury {
    use super::*;

    const ORBITAL_PERIOD: f32 = 87.97; // day
    const ROTATION_PERIOD: f32 = 59.0; // day
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / (ROTATION_PERIOD * ONE_DAY_SECONDS);
    pub const RADIUS: f32 = 2439.7;
    pub const DISTANCE_TO_SUN: f32 = 58_000_000.0;
}

pub mod venus {
    //
    use super::*;

    const ORBITAL_PERIOD: f32 = 224.7; // day
    const ROTATION_PERIOD: f32 = 243.0; // day
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / (ROTATION_PERIOD * ONE_DAY_SECONDS);
    pub const RADIUS: f32 = 6051.8;
    pub const DISTANCE_TO_SUN: f32 = 108_000_000.0;
}

pub mod earth {
    use super::*;

    const ORBITAL_PERIOD: f32 = 365.26; // day
    const ROTATION_PERIOD: f32 = 1.0; // day
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / ONE_DAY_SECONDS;
    pub const RADIUS: f32 = 6371.0;
    pub const DISTANCE_TO_SUN: f32 = 149_597_871.0;
}

pub mod moon {
    use super::*;
    const ORBITAL_PERIOD: f32 = 27.3; // day
    const ROTATION_PERIOD: f32 = 27.3;
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / (ROTATION_PERIOD * ONE_DAY_SECONDS) as f32;
    pub const RADIUS: f32 = 1737.4;
    pub const DISTANCE_TO_EARTH: f32 = 384400.0;
}

pub mod mars {
    use super::*;

    const ORBITAL_PERIOD: f32 = 687.0; // day
    const ROTATION_PERIOD: f32 = (24. + 37. / 60.) / 24.; // day 1d37min
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / (ROTATION_PERIOD * ONE_DAY_SECONDS);
    pub const RADIUS: f32 = 3389.5;
    pub const DISTANCE_TO_SUN: f32 = 227_900_000.0;
}

pub mod jupiter {
    use super::*;

    const ORBITAL_PERIOD: f32 = 11.86 * 365.; // day 11.86years
    const ROTATION_PERIOD: f32 = (9. + 50.0 / 60.) / 24.; // day 9h50min
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / (ROTATION_PERIOD * ONE_DAY_SECONDS);
    pub const RADIUS: f32 = 69911.;
    pub const DISTANCE_TO_SUN: f32 = 778_500_000.0;
}

pub mod saturn {
    use super::*;

    const ORBITAL_PERIOD: f32 = 29.46 * 365.; // day 29.46years
    const ROTATION_PERIOD: f32 = (10. + 39.0 / 60.) / 24.; // day 10h39min
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / (ROTATION_PERIOD * ONE_DAY_SECONDS);
    pub const RADIUS: f32 = 58232.;
    pub const DISTANCE_TO_SUN: f32 = 1_434_000_000.0;
}

pub mod uranus {
    use super::*;

    const ORBITAL_PERIOD: f32 = 84.01 * 365.; // day 84.01years
    const ROTATION_PERIOD: f32 = (17. + 14.0 / 60.) / 24.; // day
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / (ROTATION_PERIOD * ONE_DAY_SECONDS);
    pub const RADIUS: f32 = 25362.;
    pub const DISTANCE_TO_SUN: f32 = 2_871_000_000.0;
}

pub mod neptune {
    use super::*;

    const ORBITAL_PERIOD: f32 = 164.82 * 365.; // day 164.82years
    const ROTATION_PERIOD: f32 = (16. + 6.0 / 60.) / 24.; // day
    pub const ORBITAL_VELCITY: f32 = 2.0 * PI / (ORBITAL_PERIOD * ONE_DAY_SECONDS);
    pub const ROTATION_VELCITY: f32 = 2.0 * PI / (ROTATION_PERIOD * ONE_DAY_SECONDS);
    pub const RADIUS: f32 = 24622.;
    pub const DISTANCE_TO_SUN: f32 = 4_495_000_000.0;
}

pub const TIME_SCALE: f32 = (24 * 60 * 6) as f32; // 10s => 1day
pub const SPACE_SCALE: f32 = 1.0 / earth::RADIUS;
