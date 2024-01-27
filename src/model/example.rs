use super::celestial_system::{CelestialSystem, SystemType};
use astro_utils::data::{planets::*, stars::*};

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
