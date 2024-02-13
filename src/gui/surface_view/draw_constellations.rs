use super::{viewport::Viewport, widget::SurfaceViewState};
use crate::{
    gui::{
        shared_canvas_functionality::contains_workaround,
        surface_view::star_appearance::StarCanvasAppearance,
    },
    model::{celestial_system::CelestialSystem, constellation::*, star::Star},
};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::canvas::{Frame, Path, Stroke, Style, Text},
    Color, Rectangle, Vector,
};

impl SurfaceViewState {
    pub(super) fn draw_constellations(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        celestial_system: &CelestialSystem,
        viewport: &Viewport,
    ) {
        let stars = celestial_system.get_stars();

        for constellation_name in collect_constellation_names(&stars[..]) {
            let stars = collect_stars_in_constellation(&constellation_name, &stars[..]);
            self.draw_constellation(frame, bounds, constellation_name, stars, &viewport);
        }
    }

    fn draw_constellation(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        constellation_name: String,
        stars: Vec<&Star>,
        viewport: &Viewport,
    ) {
        let appearances: Vec<_> = stars
            .iter()
            .map(|s| s.get_appearance())
            .map(|s| StarCanvasAppearance::from_star_appearance(s, viewport))
            .filter_map(|a| a)
            .collect();

        let color = Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 0.5,
        };

        let max_line_distance = largest_nearest_neghbour_distance(&appearances) * 1.1;

        for i in 0..appearances.len() {
            for j in i + 1..appearances.len() {
                let v_i = &appearances[i].center_offset;
                let v_j = &appearances[j].center_offset;
                if distance(v_i, v_j) < max_line_distance {
                    let stroke = Stroke {
                        style: Style::Solid(Color::WHITE),
                        ..Default::default()
                    };
                    let p_i = frame.center() + *v_i;
                    let p_j = frame.center() + *v_j;
                    frame.stroke(&Path::line(p_i, p_j), stroke);
                }
            }
        }

        let center = weighted_average_position(&appearances);
        let position = frame.center() + center;
        if contains_workaround(&bounds, position) {
            let name_widget = Text {
                content: constellation_name,
                position,
                color,
                size: 20.,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                ..Default::default()
            };
            frame.fill_text(name_widget);
        }
    }
}

fn largest_nearest_neghbour_distance(stars: &[StarCanvasAppearance]) -> f32 {
    const MIN: f32 = 10.;
    const MAX: f32 = 300.;
    let mut largest_nearest_neighbour_distance = MIN;
    for i in 0..stars.len() {
        let mut nearest_neighbour_distance = MAX;
        for j in i + 1..stars.len() {
            let offset_i = stars[i].center_offset;
            let offset_j = stars[j].center_offset;
            let distance = distance(&offset_i, &offset_j);
            if distance < nearest_neighbour_distance {
                nearest_neighbour_distance = distance;
            }
        }
        if nearest_neighbour_distance > largest_nearest_neighbour_distance {
            largest_nearest_neighbour_distance = nearest_neighbour_distance;
        }
    }
    largest_nearest_neighbour_distance
}

fn distance(offset_i: &Vector, offset_j: &Vector) -> f32 {
    let diff = offset_i.clone() - offset_j.clone();
    (diff.x.powi(2) + diff.y.powi(2)).sqrt()
}

fn weighted_average_position(stars: &[StarCanvasAppearance]) -> Vector {
    let mut sum = Vector::new(0., 0.);
    let mut total_weight = 0.;
    for star in stars {
        let weight = star.radius.powi(2) * star.color.a;
        sum = sum + star.center_offset * weight;
        total_weight += weight;
    }
    sum * (1. / total_weight)
}
