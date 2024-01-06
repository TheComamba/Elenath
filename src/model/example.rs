use super::celestial_system::CelestialSystem;
use astro_utils::data::{planets::*, stars::*};

pub(crate) fn solar_system() -> CelestialSystem {
    let mut system = CelestialSystem::new(SUN_DATA.to_star());
    system.add_planet(MERCURY.to_planet());
    system.add_planet(VENUS.to_planet());
    system.add_planet(EARTH.to_planet());
    system.add_planet(MARS.to_planet());
    system.add_planet(CERES.to_planet());
    system.add_planet(JUPITER.to_planet());
    system.add_planet(SATURN.to_planet());
    system.add_planet(URANUS.to_planet());
    system.add_planet(NEPTUNE.to_planet());
    system.add_planet(PLUTO.to_planet());

    for data in STARS_TO_TWO_POINT_FIVE_APPARENT_MAG {
        system.add_distant_star(data.to_star());
    }

    system
}
