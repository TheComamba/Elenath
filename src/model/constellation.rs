use super::star::Star;
use astro_utils::stars::star_data::StarData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Constellation {}

impl Constellation {
    pub(super) fn collect_constellation_names(all_stars: &[&StarData]) -> Vec<String> {
        let mut constellation_names: Vec<String> = Vec::new();
        for star in all_stars {
            if let Some(constellation) = star.get_constellation() {
                if !constellation_names.contains(constellation) {
                    constellation_names.push(constellation.clone());
                }
            }
        }
        constellation_names
    }

    pub(super) fn collect_stars_in_constellation(
        constellation_name: &str,
        all_stars: &[&Star],
    ) -> Vec<Star> {
        let mut stars_in_constellation: Vec<Star> = Vec::new();
        for star in all_stars {
            if let Some(data) = star.get_data() {
                if let Some(constellation) = data.get_constellation() {
                    if constellation == constellation_name {
                        stars_in_constellation.push((*star).clone());
                    }
                }
            }
        }
        stars_in_constellation
    }
}
