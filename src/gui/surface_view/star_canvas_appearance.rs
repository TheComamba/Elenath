use iced::{Point, Color};

pub(super) struct StarCanvasAppearance<'a>{
    name: &'a str,
    pos: Point,
    radius: f32,
    color: Color,
}