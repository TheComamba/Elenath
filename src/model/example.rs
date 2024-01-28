use super::celestial_system::{CelestialSystem, SystemType};
use astro_utils::{
    data::{planets::*, stars::*},
    stars::random_stars::generate_random_stars,
    units::length::Length,
};

pub(crate) fn solar_system() -> CelestialSystem {
    let mut system = CelestialSystem::new(SystemType::Real, SUN_DATA.to_star_data());
    system.add_planet_data(MERCURY.to_planet_data());
    system.add_planet_data(VENUS.to_planet_data());
    system.add_planet_data(EARTH.to_planet_data());
    system.add_planet_data(MARS.to_planet_data());
    system.add_planet_data(CERES.to_planet_data());
    system.add_planet_data(JUPITER.to_planet_data());
    system.add_planet_data(SATURN.to_planet_data());
    system.add_planet_data(URANUS.to_planet_data());
    system.add_planet_data(NEPTUNE.to_planet_data());
    system.add_planet_data(PLUTO.to_planet_data());

    for data in BRIGHTEST_STARS.iter() {
        system.add_star_from_data(data.to_star_data());
    }

    system
}

pub(crate) fn generated_system() -> CelestialSystem {
    let central_body_data = SUN_DATA.to_star_data();
    let mut system = CelestialSystem::new(SystemType::Generated, central_body_data);

    let distant_stars = generate_random_stars(Length::from_light_years(100.0)).unwrap();
    for star in distant_stars {
        system.add_star_from_data(star);
    }
    system
}
