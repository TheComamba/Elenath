use astro_utils::{angle::Angle, time::Time, Float};

#[derive(Debug, Clone)]
pub(crate) struct RotationParameters {
    axis_tilt: Angle,
    rotation_period: Time,
    initial_angle: Angle,
}

impl RotationParameters {
    pub(super) fn new(axis_tilt: Angle, rotation_period: Time, initial_angle: Angle) -> Self {
        RotationParameters {
            axis_tilt,
            rotation_period,
            initial_angle,
        }
    }
}
