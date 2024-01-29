use astro_utils::stars::{star_appearance::StarAppearance, star_data::StarData};
use serde::{Deserialize, Serialize};

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

    pub(crate) fn get_index(&self) -> Option<usize> {
        self.index
    }
}
