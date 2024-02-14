use super::{viewport::Viewport, widget::SurfaceViewState};
use crate::{
    gui::{
        shared_canvas_functionality::contains_workaround,
        surface_view::canvas_appearance::CanvasAppearance,
    },
    model::celestial_system::CelestialSystem,
};
use astro_utils::stars::{
    constellation::{collect_constellations, Constellation},
    star_data::StarData,
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
        let stars: Vec<StarData> = celestial_system
            .get_stars()
            .iter()
            .map(|s| s.get_data())
            .filter_map(|s| s)
            .cloned()
            .collect();

        for constellation in collect_constellations(&stars[..]) {
            self.draw_constellation(frame, bounds, constellation, &viewport);
        }
    }

    fn draw_constellation(
        &self,
        frame: &mut Frame,
        bounds: Rectangle,
        constellation: Constellation,
        viewport: &Viewport,
    ) {
        let appearances: Vec<_> = constellation
            .get_stars()
            .iter()
            .map(|s| CanvasAppearance::from_star_appearance(&s, viewport))
            .filter_map(|a| a)
            .collect();

        let color = Color {
            r: 1.,
            g: 1.,
            b: 1.,
            a: 0.5,
        };

        for connection in constellation.get_connections() {
            let (i, j) = connection.get_indices();
            let p_i = frame.center() + appearances[i].center_offset;
            let p_j = frame.center() + appearances[j].center_offset;
            let stroke = Stroke {
                style: Style::Solid(Color::WHITE),
                ..Default::default()
            };
            frame.stroke(&Path::line(p_i, p_j), stroke);
        }

        // let outline = Path::new(|b| {
        //     for appearance in &appearances {
        //         let position = frame.center() + appearance.center_offset;
        //         let radius = appearance.radius * 50.;
        //         b.circle(position, radius);
        //     }
        // });
        // let stroke = Stroke {
        //     style: Style::Solid(color),
        //     width: 1.,
        //     ..Default::default()
        // };
        // frame.stroke(&outline, stroke);

        let center = weighted_average_position(&appearances);
        let position = frame.center() + center;
        if contains_workaround(&bounds, position) {
            let name_widget = Text {
                content: constellation.get_name().to_string(),
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

fn find_nearest_neighbour(
    index: usize,
    stars: &[CanvasAppearance],
    excluding: &Vec<usize>,
) -> Option<usize> {
    let mut nearest_neighbour = None;
    let offset = stars[index].center_offset;
    for j in 0..stars.len() {
        if index != j && !excluding.contains(&j) {
            let offset_j = stars[j].center_offset;
            let distance = distance_squared(&offset, &offset_j);
            if let Some(nn) = nearest_neighbour {
                let nn_offset = stars[nn as usize].center_offset;
                let nn_distance = distance_squared(&offset, &nn_offset);
                if distance < nn_distance {
                    nearest_neighbour = Some(j);
                }
            } else {
                nearest_neighbour = Some(j);
            }
        }
    }
    nearest_neighbour
}

fn distance_squared(offset_i: &Vector, offset_j: &Vector) -> f32 {
    let diff = offset_i.clone() - offset_j.clone();
    diff.x.powi(2) + diff.y.powi(2)
}

fn weighted_average_position(stars: &[CanvasAppearance]) -> Vector {
    let mut sum = Vector::new(0., 0.);
    let mut total_weight = 0.;
    for star in stars {
        let weight = star.radius.powi(2) * star.color.a;
        sum = sum + star.center_offset * weight;
        total_weight += weight;
    }
    sum * (1. / total_weight)
}

fn connections(stars: &[CanvasAppearance]) -> Vec<(usize, usize)> {
    prims_algorithm(stars)
}

fn prims_algorithm(stars: &[CanvasAppearance]) -> Vec<(usize, usize)> {
    let mut connections = Vec::new();
    if stars.len() < 2 {
        return connections;
    }
    let mut visited = vec![0];
    while visited.len() < stars.len() {
        let mut current_best = (0, 0, f32::MAX);
        for i in &visited {
            let nn = find_nearest_neighbour(*i, stars, &visited);
            if let Some(nn) = nn {
                let offset_i = stars[*i].center_offset;
                let offset_nn = stars[nn].center_offset;
                let distance_squared = distance_squared(&offset_i, &offset_nn);
                if distance_squared < current_best.2 {
                    current_best = (*i, nn, distance_squared);
                }
            }
        }
        connections.push((current_best.0, current_best.1));
        visited.push(current_best.1);
    }
    connections
}
