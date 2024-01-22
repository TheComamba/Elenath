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
    pub(super) px_per_unit_height: f32,
}

impl Viewport {
    pub(super) fn calculate(
        observer_normal: &Direction,
        local_view_direction: &SphericalCoordinates,
        opening_angle: Angle,
        rotation_axis: &Direction,
        canvas_height: f32,
    ) -> Self {
        let view_direction = local_view_direction.to_direction();
        let center_direction = view_direction.active_rotation_to_new_z_axis(observer_normal);
        let ortho = match center_direction.cross_product(rotation_axis) {
            Ok(ortho) => ortho,
            Err(_) => match observer_normal.cross_product(rotation_axis) {
                Ok(ortho) => ortho,
                Err(_) => center_direction.some_orthogonal_vector(),
            },
        };
        let top_direction = center_direction.rotated(opening_angle / 2., &ortho);
        let viewport_height = (opening_angle / 2.).sin() * 2.; //Viewport is at unit distance
        let px_per_unit_height = canvas_height / viewport_height;
        Self {
            center_direction,
            top_direction,
            px_per_unit_height,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use astro_utils::coordinates::{direction::Direction, spherical::SphericalCoordinates};

    const TEST_ACCURACY: f32 = 1e-5;
    const SOME_ANGLE: Angle = Angle::from_radians(1.0);
    const SOME_HEIGHT: f32 = 100.;

    #[test]
    fn view_direction_z_does_not_influence_center_direction_and_makes_rotation_axis_irrelevant() {
        let ordinates = vec![-1., 0., 1., 12.];
        for x1 in ordinates.clone().iter() {
            for y1 in ordinates.clone().iter() {
                for z1 in ordinates.clone().iter() {
                    for x2 in ordinates.clone().iter() {
                        for y2 in ordinates.clone().iter() {
                            for z2 in ordinates.clone().iter() {
                                let view_direction = SphericalCoordinates::Z_DIRECTION;
                                let observer_normal = Direction::new(*x1, *y1, *z1);
                                let rotation_axis = Direction::new(*x2, *y2, *z2);
                                if observer_normal.is_err() || rotation_axis.is_err() {
                                    continue;
                                }
                                let observer_normal = observer_normal.unwrap();
                                let rotation_axis = rotation_axis.unwrap();
                                let viewport = Viewport::calculate(
                                    &observer_normal,
                                    &view_direction,
                                    SOME_ANGLE,
                                    &rotation_axis,
                                    SOME_HEIGHT,
                                );
                                assert!(viewport
                                    .center_direction
                                    .eq_within(&observer_normal, TEST_ACCURACY));
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn tilting_view() {
        let observer_normal = Direction::X;
        let rotation_axis = Direction::Z;
        let west_view = SphericalCoordinates::X_DIRECTION;
        let south_view = SphericalCoordinates::Y_DIRECTION;
        let east_view = -SphericalCoordinates::X_DIRECTION;
        let north_view = -SphericalCoordinates::Y_DIRECTION;
        let westward_viewport = Viewport::calculate(
            &observer_normal,
            &west_view,
            SOME_ANGLE,
            &rotation_axis,
            SOME_HEIGHT,
        );
        let southward_viewport = Viewport::calculate(
            &observer_normal,
            &south_view,
            SOME_ANGLE,
            &rotation_axis,
            SOME_HEIGHT,
        );
        let eastward_viewport = Viewport::calculate(
            &observer_normal,
            &east_view,
            SOME_ANGLE,
            &rotation_axis,
            SOME_HEIGHT,
        );
        let northward_viewport = Viewport::calculate(
            &observer_normal,
            &north_view,
            SOME_ANGLE,
            &rotation_axis,
            SOME_HEIGHT,
        );
        assert!(westward_viewport
            .center_direction
            .eq_within(&-&Direction::Y, TEST_ACCURACY));
        assert!(southward_viewport
            .center_direction
            .eq_within(&-&Direction::Z, TEST_ACCURACY));
        assert!(eastward_viewport
            .center_direction
            .eq_within(&Direction::Y, TEST_ACCURACY));
        assert!(northward_viewport
            .center_direction
            .eq_within(&Direction::Z, TEST_ACCURACY));
    }

    #[test]
    fn top_direction_aligns_with_rotation_axis() {
        let ordinates = vec![-1., 0., 1., 12.];
        for x1 in ordinates.clone().iter() {
            for y1 in ordinates.clone().iter() {
                for z1 in ordinates.clone().iter() {
                    for x2 in ordinates.clone().iter() {
                        for y2 in ordinates.clone().iter() {
                            for z2 in ordinates.clone().iter() {
                                for x3 in ordinates.clone().iter() {
                                    for y3 in ordinates.clone().iter() {
                                        for z3 in ordinates.clone().iter() {
                                            let observer_normal = Direction::new(*x1, *y1, *z1);
                                            let rotation_axis = Direction::new(*x2, *y2, *z2);
                                            let view_direction = Direction::new(*x3, *y3, *z3);
                                            if observer_normal.is_err()
                                                || rotation_axis.is_err()
                                                || view_direction.is_err()
                                            {
                                                continue;
                                            }
                                            let observer_normal = observer_normal.unwrap();
                                            let rotation_axis = rotation_axis.unwrap();
                                            let view_direction =
                                                view_direction.unwrap().to_spherical();
                                            let viewport = Viewport::calculate(
                                                &observer_normal,
                                                &view_direction,
                                                SOME_ANGLE,
                                                &rotation_axis,
                                                SOME_HEIGHT,
                                            );

                                            let ortho = rotation_axis
                                                .cross_product(&viewport.center_direction);
                                            if ortho.is_err() {
                                                continue;
                                            }
                                            let ortho = ortho.unwrap();
                                            let overlap =
                                                ortho.dot_product(&viewport.top_direction);

                                            println!(
                                                "center_direction: {}",
                                                viewport.center_direction
                                            );
                                            println!("top_direction: {}", viewport.top_direction);
                                            println!("rotation_axis: {}", rotation_axis);
                                            println!("ortho: {}", ortho);
                                            println!("overlap: {}", overlap);
                                            assert!(overlap.abs() < TEST_ACCURACY);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn opening_angle_zero() {
        let observer_normal = Direction::X;
        let rotation_axis = Direction::Z;
        let view_direction = SphericalCoordinates::Z_DIRECTION;
        let viewport = Viewport::calculate(
            &observer_normal,
            &view_direction,
            Angle::from_degrees(0.0),
            &rotation_axis,
            SOME_HEIGHT,
        );
        let expected_top_direction = viewport.center_direction;
        assert!(viewport
            .top_direction
            .eq_within(&expected_top_direction, TEST_ACCURACY));
    }

    #[test]
    fn opening_angle_90_degrees() {
        let observer_normal = Direction::X;
        let rotation_axis = Direction::Z;
        let view_direction = SphericalCoordinates::Z_DIRECTION;
        let viewport = Viewport::calculate(
            &observer_normal,
            &view_direction,
            Angle::from_degrees(90.0),
            &rotation_axis,
            SOME_HEIGHT,
        );

        let expected_top_direction = Direction::new(1., 0., 1.).unwrap();
        assert!(viewport
            .top_direction
            .eq_within(&expected_top_direction, TEST_ACCURACY));
    }

    #[test]
    fn opening_angle_180_degrees() {
        let observer_normal = Direction::X;
        let rotation_axis = Direction::Z;
        let view_direction = SphericalCoordinates::Z_DIRECTION;
        let viewport = Viewport::calculate(
            &observer_normal,
            &view_direction,
            Angle::from_degrees(180.0),
            &rotation_axis,
            SOME_HEIGHT,
        );

        let expected_top_direction = rotation_axis;
        assert!(viewport
            .top_direction
            .eq_within(&expected_top_direction, TEST_ACCURACY));
    }
}
