use super::{celestial_system::CelestialSystem, distant_star::DistantStar};
use crate::model::{orbital_parameters::OrbitalParameters, planet_data::PlanetData};
use astro_utils::data::{planets::*, stars::*};

fn mercury() -> PlanetData {
    PlanetData::new(
        String::from("Mercury"),
        MERCURY_MASS,
        OrbitalParameters::new(
            MERCURY_SEMI_MAJOR_AXIS,
            MERCURY_ECCENTRICITY,
            MERCURY_ORBIT_ORIENTATION,
        ),
        MERCURY_RADIUS,
        MERCURY_GEOMETRIC_ALBEDO,
        MERCURY_COLOR,
        MERCURY_SIDERIAL_ROTATION_PERIOD,
        MERCURY_NORTH.to_direction(),
    )
}

fn venus() -> PlanetData {
    PlanetData::new(
        String::from("Venus"),
        VENUS_MASS,
        OrbitalParameters::new(
            VENUS_SEMI_MAJOR_AXIS,
            VENUS_ECCENTRICITY,
            VENUS_ORBIT_ORIENTATION,
        ),
        VENUS_RADIUS,
        VENUS_GEOMETRIC_ALBEDO,
        VENUS_COLOR,
        VENUS_SIDERIAL_ROTATION_PERIOD,
        VENUS_NORTH.to_direction(),
    )
}

fn earth() -> PlanetData {
    PlanetData::new(
        String::from("Earth"),
        EARTH_MASS,
        OrbitalParameters::new(
            EARTH_SEMI_MAJOR_AXIS,
            EARTH_ECCENTRICITY,
            EARTH_ORBIT_ORIENTATION,
        ),
        EARTH_RADIUS,
        EARTH_GEOMETRIC_ALBEDO,
        EARTH_COLOR,
        EARTH_SIDERIAL_ROTATION_PERIOD,
        EARTH_NORTH.to_direction(),
    )
}

fn mars() -> PlanetData {
    PlanetData::new(
        String::from("Mars"),
        MARS_MASS,
        OrbitalParameters::new(
            MARS_SEMI_MAJOR_AXIS,
            MARS_ECCENTRICITY,
            MARS_ORBIT_ORIENTATION,
        ),
        MARS_RADIUS,
        MARS_GEOMETRIC_ALBEDO,
        MARS_COLOR,
        MARS_SIDERIAL_ROTATION_PERIOD,
        MARS_NORTH.to_direction(),
    )
}

fn ceres() -> PlanetData {
    PlanetData::new(
        String::from("Ceres"),
        CERES_MASS,
        OrbitalParameters::new(
            CERES_SEMI_MAJOR_AXIS,
            CERES_ECCENTRICITY,
            CERES_ORBIT_ORIENTATION,
        ),
        CERES_RADIUS,
        CERES_GEOMETRIC_ALBEDO,
        CERES_COLOR,
        CERES_SIDERIAL_ROTATION_PERIOD,
        CERES_NORTH.to_direction(),
    )
}

fn jupiter() -> PlanetData {
    PlanetData::new(
        String::from("Jupiter"),
        JUPITER_MASS,
        OrbitalParameters::new(
            JUPITER_SEMI_MAJOR_AXIS,
            JUPITER_ECCENTRICITY,
            JUPITER_ORBIT_ORIENTATION,
        ),
        JUPITER_RADIUS,
        JUPITER_GEOMETRIC_ALBEDO,
        JUPITER_COLOR,
        JUPITER_SIDERIAL_ROTATION_PERIOD,
        JUPITER_NORTH.to_direction(),
    )
}

fn saturn() -> PlanetData {
    PlanetData::new(
        String::from("Saturn"),
        SATURN_MASS,
        OrbitalParameters::new(
            SATURN_SEMI_MAJOR_AXIS,
            SATURN_ECCENTRICITY,
            SATURN_ORBIT_ORIENTATION,
        ),
        SATURN_RADIUS,
        SATURN_GEOMETRIC_ALBEDO,
        SATURN_COLOR,
        SATURN_SIDERIAL_ROTATION_PERIOD,
        SATURN_NORTH.to_direction(),
    )
}

fn uranus() -> PlanetData {
    PlanetData::new(
        String::from("Uranus"),
        URANUS_MASS,
        OrbitalParameters::new(
            URANUS_SEMI_MAJOR_AXIS,
            URANUS_ECCENTRICITY,
            URANUS_ORBIT_ORIENTATION,
        ),
        URANUS_RADIUS,
        URANUS_GEOMETRIC_ALBEDO,
        URANUS_COLOR,
        URANUS_SIDERIAL_ROTATION_PERIOD,
        URANUS_NORTH.to_direction(),
    )
}

fn neptune() -> PlanetData {
    PlanetData::new(
        String::from("Neptune"),
        NEPTUNE_MASS,
        OrbitalParameters::new(
            NEPTUNE_SEMI_MAJOR_AXIS,
            NEPTUNE_ECCENTRICITY,
            NEPTUNE_ORBIT_ORIENTATION,
        ),
        NEPTUNE_RADIUS,
        NEPTUNE_GEOMETRIC_ALBEDO,
        NEPTUNE_COLOR,
        NEPTUNE_SIDERIAL_ROTATION_PERIOD,
        NEPTUNE_NORTH.to_direction(),
    )
}

fn pluto() -> PlanetData {
    PlanetData::new(
        String::from("Pluto"),
        PLUTO_MASS,
        OrbitalParameters::new(
            PLUTO_SEMI_MAJOR_AXIS,
            PLUTO_ECCENTRICITY,
            PLUTO_ORBIT_ORIENTATION,
        ),
        PLUTO_RADIUS,
        PLUTO_GEOMETRIC_ALBEDO,
        PLUTO_COLOR,
        PLUTO_SIDERIAL_ROTATION_PERIOD,
        PLUTO_NORTH.to_direction(),
    )
}

fn _moon() -> PlanetData {
    PlanetData::new(
        String::from("Moon"),
        MOON_MASS,
        OrbitalParameters::new(
            MOON_SEMI_MAJOR_AXIS,
            MOON_ECCENTRICITY,
            MOON_ORBIT_ORIENTATION,
        ),
        MOON_RADIUS,
        MOON_GEOMETRIC_ALBEDO,
        MOON_COLOR,
        MOON_SIDERIAL_ROTATION_PERIOD,
        MOON_NORTH.to_direction(),
    )
}

fn sirius() -> DistantStar {
    DistantStar::new(
        SIRIUS_PROPERTIES,
        SIRIUS_POSITION.to_direction(),
        SIRIUS_DISTANCE,
    )
}

fn canopus() -> DistantStar {
    DistantStar::new(
        CANOPUS_PROPERTIES,
        CANOPUS_POSITION.to_direction(),
        CANOPUS_DISTANCE,
    )
}

fn rigil_kentaurus() -> DistantStar {
    DistantStar::new(
        RIGIL_KENTAURUS_PROPERTIES,
        RIGIL_KENTAURUS_POSITION.to_direction(),
        RIGIL_KENTAURUS_DISTANCE,
    )
}

fn arcturus() -> DistantStar {
    DistantStar::new(
        ARCTURUS_PROPERTIES,
        ARCTURUS_POSITION.to_direction(),
        ARCTURUS_DISTANCE,
    )
}

fn vega() -> DistantStar {
    DistantStar::new(VEGA_PROPERTIES, VEGA_POSITION.to_direction(), VEGA_DISTANCE)
}

fn capella() -> DistantStar {
    DistantStar::new(
        CAPELLA_PROPERTIES,
        CAPELLA_POSITION.to_direction(),
        CAPELLA_DISTANCE,
    )
}

fn rigel() -> DistantStar {
    DistantStar::new(
        RIGEL_PROPERTIES,
        RIGEL_POSITION.to_direction(),
        RIGEL_DISTANCE,
    )
}

fn procyon() -> DistantStar {
    DistantStar::new(
        PROCYON_PROPERTIES,
        PROCYON_POSITION.to_direction(),
        PROCYON_DISTANCE,
    )
}

fn achernar() -> DistantStar {
    DistantStar::new(
        ACHERNAR_PROPERTIES,
        ACHERNAR_POSITION.to_direction(),
        ACHERNAR_DISTANCE,
    )
}

fn betelgeuse() -> DistantStar {
    DistantStar::new(
        BETELGEUSE_PROPERTIES,
        BETELGEUSE_POSITION.to_direction(),
        BETELGEUSE_DISTANCE,
    )
}

fn hadar() -> DistantStar {
    DistantStar::new(
        HADAR_PROPERTIES,
        HADAR_POSITION.to_direction(),
        HADAR_DISTANCE,
    )
}

fn altair() -> DistantStar {
    DistantStar::new(
        ALTAIR_PROPERTIES,
        ALTAIR_POSITION.to_direction(),
        ALTAIR_DISTANCE,
    )
}

fn acrux() -> DistantStar {
    DistantStar::new(
        ACRUX_PROPERTIES,
        ACRUX_POSITION.to_direction(),
        ACRUX_DISTANCE,
    )
}

fn aldebaran() -> DistantStar {
    DistantStar::new(
        ALDEBARAN_PROPERTIES,
        ALDEBARAN_POSITION.to_direction(),
        ALDEBARAN_DISTANCE,
    )
}

fn antares() -> DistantStar {
    DistantStar::new(
        ANTARES_PROPERTIES,
        ANTARES_POSITION.to_direction(),
        ANTARES_DISTANCE,
    )
}

fn spica() -> DistantStar {
    DistantStar::new(
        SPICA_PROPERTIES,
        SPICA_POSITION.to_direction(),
        SPICA_DISTANCE,
    )
}

fn pollux() -> DistantStar {
    DistantStar::new(
        POLLUX_PROPERTIES,
        POLLUX_POSITION.to_direction(),
        POLLUX_DISTANCE,
    )
}

fn formalhaut() -> DistantStar {
    DistantStar::new(
        FORMALHAUT_PROPERTIES,
        FORMALHAUT_POSITION.to_direction(),
        FORMALHAUT_DISTANCE,
    )
}

fn deneb() -> DistantStar {
    DistantStar::new(
        DENEB_PROPERTIES,
        DENEB_POSITION.to_direction(),
        DENEB_DISTANCE,
    )
}

fn mimosa() -> DistantStar {
    DistantStar::new(
        MIMOSA_PROPERTIES,
        MIMOSA_POSITION.to_direction(),
        MIMOSA_DISTANCE,
    )
}

fn regulus() -> DistantStar {
    DistantStar::new(
        REGULUS_PROPERTIES,
        REGULUS_POSITION.to_direction(),
        REGULUS_DISTANCE,
    )
}

fn adhara() -> DistantStar {
    DistantStar::new(
        ADHARA_PROPERTIES,
        ADHARA_POSITION.to_direction(),
        ADHARA_DISTANCE,
    )
}

fn shaula() -> DistantStar {
    DistantStar::new(
        SHAULA_PROPERTIES,
        SHAULA_POSITION.to_direction(),
        SHAULA_DISTANCE,
    )
}

fn castor() -> DistantStar {
    DistantStar::new(
        CASTOR_PROPERTIES,
        CASTOR_POSITION.to_direction(),
        CASTOR_DISTANCE,
    )
}

fn gacrux() -> DistantStar {
    DistantStar::new(
        GACRUX_PROPERTIES,
        GACRUX_POSITION.to_direction(),
        GACRUX_DISTANCE,
    )
}

fn bellatrix() -> DistantStar {
    DistantStar::new(
        BELLATRIX_PROPERTIES,
        BELLATRIX_POSITION.to_direction(),
        BELLATRIX_DISTANCE,
    )
}

fn elnath() -> DistantStar {
    DistantStar::new(
        ELNATH_PROPERTIES,
        ELNATH_POSITION.to_direction(),
        ELNATH_DISTANCE,
    )
}

fn miaplacidus() -> DistantStar {
    DistantStar::new(
        MIAPLACIDUS_PROPERTIES,
        MIAPLACIDUS_POSITION.to_direction(),
        MIAPLACIDUS_DISTANCE,
    )
}

fn alnilam() -> DistantStar {
    DistantStar::new(
        ALNILAM_PROPERTIES,
        ALNILAM_POSITION.to_direction(),
        ALNILAM_DISTANCE,
    )
}

fn gamma_velorum() -> DistantStar {
    DistantStar::new(
        GAMMA_VELORUM_PROPERTIES,
        GAMMA_VELORUM_POSITION.to_direction(),
        GAMMA_VELORUM_DISTANCE,
    )
}

fn alnair() -> DistantStar {
    DistantStar::new(
        ALNAIR_PROPERTIES,
        ALNAIR_POSITION.to_direction(),
        ALNAIR_DISTANCE,
    )
}

pub(crate) fn solar_system() -> CelestialSystem {
    let mut system = CelestialSystem::new(SUN_PROPERTIES);
    system.add_planet(mercury());
    system.add_planet(venus());
    system.add_planet(earth());
    system.add_planet(mars());
    system.add_planet(ceres());
    system.add_planet(jupiter());
    system.add_planet(saturn());
    system.add_planet(uranus());
    system.add_planet(neptune());
    system.add_planet(pluto());
    system.add_distant_star(sirius());
    system.add_distant_star(canopus());
    system.add_distant_star(rigil_kentaurus());
    system.add_distant_star(arcturus());
    system.add_distant_star(vega());
    system.add_distant_star(capella());
    system.add_distant_star(rigel());
    system.add_distant_star(procyon());
    system.add_distant_star(achernar());
    system.add_distant_star(betelgeuse());
    system.add_distant_star(hadar());
    system.add_distant_star(altair());
    system.add_distant_star(acrux());
    system.add_distant_star(aldebaran());
    system.add_distant_star(antares());
    system.add_distant_star(spica());
    system.add_distant_star(pollux());
    system.add_distant_star(formalhaut());
    system.add_distant_star(deneb());
    system.add_distant_star(mimosa());
    system.add_distant_star(regulus());
    system.add_distant_star(adhara());
    system.add_distant_star(shaula());
    system.add_distant_star(castor());
    system.add_distant_star(gacrux());
    system.add_distant_star(bellatrix());
    system.add_distant_star(elnath());
    system.add_distant_star(miaplacidus());
    system.add_distant_star(alnilam());
    system.add_distant_star(gamma_velorum());
    system.add_distant_star(alnair());
    system
}
