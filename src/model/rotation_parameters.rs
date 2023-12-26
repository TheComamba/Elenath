use astro_utils::{time::Time, Float};

#[derive(Debug, Clone)]
pub(crate) struct RotationParameters {
    axis_tilt: Float,
    rotation_period: Time,
    initial_angle: Float,
}

impl RotationParameters {
    pub(super) fn new(axis_tilt: Float, rotation_period: Time, initial_angle: Float) -> Self {
        RotationParameters {
            axis_tilt,
            rotation_period,
            initial_angle,
        }
    }
}
