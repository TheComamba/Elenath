use astro_utils::stars::{star_appearance::StarAppearance, star_data::StarData};
use serde::{Deserialize, Serialize};

use super::part_of_celestial_system::{BodyType, PartOfCelestialSystem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Star {
    data: Option<StarData>,
    appearance: StarAppearance,
    index: Option<usize>,
}

impl Star {
    pub(crate) fn from_data(data: StarData, index: Option<usize>) -> Self {
        let appearance = data.to_star_appearance();
        Star {
            data: Some(data),
            appearance,
            index,
        }
    }

    pub(crate) fn from_appearance(appearance: StarAppearance, index: Option<usize>) -> Self {
        Star {
            data: None,
            appearance,
            index,
        }
    }

    pub(crate) fn get_data(&self) -> Option<&StarData> {
        self.data.as_ref()
    }

    pub(crate) fn get_appearance(&self) -> &StarAppearance {
        &self.appearance
    }

    pub(super) fn set_index(&mut self, index: usize) {
        self.index = Some(index);
    }
}

impl PartOfCelestialSystem for Star {
    fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn get_body_type(&self) -> BodyType {
        BodyType::Star
    }
}
