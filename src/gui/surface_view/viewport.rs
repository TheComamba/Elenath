use astro_utils::{
    coordinates::{
        direction::Direction, equatorial::EquatorialCoordinates, spherical::SphericalCoordinates,
    },
    planets::{planet_data::PlanetData, surface_normal::surface_normal_at_time},
    units::{angle::Angle, time::Time},
};

pub(super) struct Viewport {
    pub(super) center_direction: Direction,
    pub(super) top_direction: Direction,
    pub(super) height: f32,
}

impl Viewport {
    pub(super) fn calculate(
        planet_data: &PlanetData,
        surface_position: &SphericalCoordinates,
        time_since_epoch: Time,
        local_view_direction: &SphericalCoordinates,
        viewport_opening_angle: Angle,
    ) -> Self {
        todo!()
    }
}

pub(super) fn observer_normal(
    planet: &PlanetData,
    surface_position: SphericalCoordinates,
    time_since_epoch: Time,
) -> Direction {
    let observer_equatorial_position =
        EquatorialCoordinates::new(surface_position, planet.get_rotation_axis().clone());
    //TODO: Define Angle at Epoch
    let planet_angle_at_epoch = Angle::from_degrees(0.0);
    surface_normal_at_time(
        observer_equatorial_position,
        planet_angle_at_epoch,
        time_since_epoch,
        planet.get_sideral_rotation_period(),
    )
}
