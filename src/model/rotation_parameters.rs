pub(crate) struct RotationParameters {
    axis_tilt: f32,
    rotation_period: f32,
    initial_angle: f32,
}

impl RotationParameters {
    pub(super) fn new(axis_tilt: f32, rotation_period: f32, initial_angle: f32) -> Self {
        RotationParameters {
            axis_tilt,
            rotation_period,
            initial_angle,
        }
    }
}
