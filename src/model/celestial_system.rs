use std::path::PathBuf;

use super::celestial_body::CelestialBody;
use super::distant_star::DistantStar;
use super::planet_data::PlanetData;
use astro_utils::{stellar_properties::StellarProperties, units::time::Time};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CelestialSystem {
    central_body: StellarProperties,
    planets: Vec<PlanetData>,
    distant_stars: Vec<DistantStar>,
}

impl CelestialSystem {
    pub(crate) fn new(central_body: StellarProperties) -> Self {
        CelestialSystem {
            central_body,
            planets: vec![],
            distant_stars: vec![],
        }
    }

    pub(crate) fn write_to_file(&self, path: PathBuf) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub(crate) fn read_from_file(path: PathBuf) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let celestial_system = serde_json::from_reader(reader)?;
        Ok(celestial_system)
    }

    pub(crate) fn get_central_body_data(&self) -> &StellarProperties {
        &self.central_body
    }

    pub(crate) fn get_current_data(&self, time: Time) -> Vec<CelestialBody> {
        let mut bodies = Vec::new();
        let central_body = CelestialBody::central_body(self.central_body.clone());
        bodies.push(central_body.clone());
        for planet in &self.planets {
            bodies.push(CelestialBody::from_planet(planet, &central_body, time));
        }
        for star in &self.distant_stars {
            bodies.push(CelestialBody::from_distant_star(star));
        }
        bodies
    }

    pub(crate) fn add_planet(&mut self, planet: PlanetData) {
        self.planets.push(planet);
    }

    pub(crate) fn add_distant_star(&mut self, star: DistantStar) {
        self.distant_stars.push(star);
    }

    pub(crate) fn get_planets_data(&self) -> Vec<&PlanetData> {
        let mut bodies = Vec::new();
        for planet in &self.planets {
            bodies.push(planet);
        }
        bodies
    }
}
