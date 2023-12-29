use super::celestial_body::CelestialBody;
use super::celestial_body_data::CelestialBodyData;
use astro_utils::units::time::Time;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CelestialSystem {
    central_body: CelestialBodyData,
    planets: Vec<CelestialBodyData>,
}

impl CelestialSystem {
    pub(crate) fn new(central_body: CelestialBodyData) -> Self {
        CelestialSystem {
            central_body,
            planets: vec![],
        }
    }

    pub(crate) fn get_current_data(&self, time: Time) -> Vec<CelestialBody> {
        let mut bodies = Vec::new();
        let central_body = CelestialBody::new(self.central_body.clone(), None, time);
        bodies.push(central_body.clone());
        for planet in &self.planets {
            let planet_body = CelestialBody::new(planet.clone(), Some(&central_body), time);
            bodies.push(planet_body);
        }
        bodies
    }

    pub(crate) fn add_planet(&mut self, planet: CelestialBodyData) {
        self.planets.push(planet);
    }

    pub(crate) fn get_central_body_data(&self) -> &CelestialBodyData {
        &self.central_body
    }

    pub(crate) fn get_bodies_data(&self) -> Vec<&CelestialBodyData> {
        let mut bodies = Vec::new();
        bodies.push(&self.central_body);
        for planet in &self.planets {
            bodies.push(planet);
        }
        bodies
    }
}
