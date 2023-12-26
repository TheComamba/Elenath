pub(crate) struct OrbitalParameters {
    semi_major_axis: f64,
    eccentricity: f64,
    inclination: f64,
    longitude_of_ascending_node: f64,
    argument_of_periapsis: f64,
    true_anomaly: f64,
}

impl OrbitalParameters {
    pub(super) fn new(
        semi_major_axis: f64,
        eccentricity: f64,
        inclination: f64,
        longitude_of_ascending_node: f64,
        argument_of_periapsis: f64,
        true_anomaly: f64,
    ) -> Self {
        OrbitalParameters {
            semi_major_axis,
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
            true_anomaly,
        }
    }
}
