use super::star::Star;

pub(crate) fn collect_constellation_names(all_stars: &[Star]) -> Vec<String> {
    let mut constellation_names: Vec<String> = Vec::new();
    for star in all_stars {
        if let Some(data) = star.get_data() {
            if let Some(constellation) = data.get_constellation() {
                if !constellation_names.contains(constellation) {
                    constellation_names.push(constellation.clone());
                }
            }
        }
    }
    constellation_names
}

pub(crate) fn collect_stars_in_constellation<'a>(
    constellation_name: &str,
    all_stars: &'a [Star],
) -> Vec<&'a Star> {
    let mut stars_in_constellation: Vec<&Star> = Vec::new();
    for star in all_stars {
        if let Some(data) = star.get_data() {
            if let Some(constellation) = data.get_constellation() {
                if constellation == constellation_name {
                    stars_in_constellation.push(star);
                }
            }
        }
    }
    stars_in_constellation
}
