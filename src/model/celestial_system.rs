use super::celestial_body::CelestialBody;
use super::celestial_body_data::CelestialBodyData;
use astro_utils::units::time::Time;

pub(crate) struct CelestialSystem {
    bodies: Vec<CelestialBodyData>,
    central_body_name: String,
}

impl CelestialSystem {
    pub(crate) fn new(central_body: CelestialBodyData) -> Self {
        let central_body_name = central_body.get_name().clone();
        CelestialSystem {
            bodies: vec![central_body],
            central_body_name,
        }
    }

    pub(crate) fn get_current_data(&self, time: Time) -> Vec<CelestialBody> {
        let mut system = Vec::new();
        system
    }
}
