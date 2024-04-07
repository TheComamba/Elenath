use super::star::Star;
use astro_utils::{
    planets::planet_data::PlanetData,
    stars::{constellation::constellation::Constellation, data::StarData, fate::StarFate},
    units::{distance::DISTANCE_ZERO, time::TIME_ZERO},
};
use serde::{Deserialize, Serialize};
use simple_si_units::base::Time;
use std::path::PathBuf;

pub(crate) mod constellations;
pub(crate) mod part;
pub(crate) mod planets;
pub(crate) mod stars;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct CelestialSystem {
    central_body: StarData,
    planets: Vec<PlanetData>,
    distant_stars: Vec<Star>,
    constellations: Vec<Constellation>,
    time_since_epoch: Time<f64>,
}

impl CelestialSystem {
    pub(crate) fn new(mut central_body: StarData) -> Self {
        central_body.set_distance_at_epoch(DISTANCE_ZERO);
        CelestialSystem {
            central_body,
            planets: vec![],
            distant_stars: vec![],
            constellations: vec![],
            time_since_epoch: TIME_ZERO,
        }
    }

    pub(crate) fn set_time_since_epoch(&mut self, time_since_epoch: Time<f64>) {
        self.time_since_epoch = time_since_epoch;
        for star in &mut self.distant_stars {
            star.recalculate_appearance_if_necessary(time_since_epoch);
        }
        self.update_constellations();
    }

    pub(crate) fn get_time_since_epoch(&self) -> Time<f64> {
        self.time_since_epoch
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

    pub(crate) fn get_supernovae(&self) -> Vec<Star> {
        let mut supernovae: Vec<Star> = self
            .get_stars()
            .into_iter()
            .filter(|s| {
                if let Some(data) = s.get_data() {
                    data.get_fate() == &StarFate::TypeIISupernova
                } else {
                    false
                }
            })
            .collect();
        supernovae.sort_by(|a, b| {
            a.get_data()
                .unwrap()
                .get_time_until_death(self.time_since_epoch)
                .partial_cmp(
                    &b.get_data()
                        .unwrap()
                        .get_time_until_death(self.time_since_epoch),
                )
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        supernovae
    }
}
