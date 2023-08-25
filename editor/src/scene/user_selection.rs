use std::{cell::RefCell, rc::Rc};

use iced::{
    widget::canvas::{Fill, Frame, Path, Stroke},
    Color, Point,
};
use vgc::coord::Coord;

use crate::scene::{point_in_radius, Scene};

pub struct Selected {
    pub shapes: Vec<SelectedShape>,
}

pub struct SelectedShape {
    pub shape_index: usize,
    pub coords: Vec<Rc<RefCell<Coord>>>,
    pub hover_coord: Option<Rc<RefCell<Coord>>>,
}

impl Default for Selected {
    fn default() -> Self {
        Self {
            shapes: vec![SelectedShape::new(0)],
        }
    }
}

enum CoordState {
    Hover,
    Selected,
    None,
}

impl SelectedShape {
    fn new(shape_index: usize) -> Self {
        Self {
            shape_index: shape_index,
            coords: Vec::new(),
            hover_coord: None,
        }
    }

    fn coord_state(&self, coord_ref: &Rc<RefCell<Coord>>) -> CoordState {
        match &self.hover_coord {
            Some(hover_coord) if hover_coord == coord_ref => CoordState::Hover,
            _ => match self.coords.iter().find(|coord| *coord == coord_ref) {
                Some(_) => CoordState::Selected,
                None => CoordState::None,
            },
        }
    }
}

pub fn change_hover(scene: &mut Scene, cursor_position: Point) {
    'shape_loop: for shape_selected in &mut scene.selected.shapes {
        let shape = scene
            .vgc_data
            .get_shape(shape_selected.shape_index)
            .unwrap();
        let coords = shape.get_coords_of_shape_tmp();
        for ref_coord_type in coords {
            let coord = ref_coord_type.borrow();
            if point_in_radius(
                &scene.camera.project(cursor_position),
                &Point::new(coord.x, coord.y),
                scene.camera.fixed_length(12.0),
            ) {
                shape_selected.hover_coord = Some(ref_coord_type.clone());
                continue 'shape_loop;
            }
        }
        shape_selected.hover_coord = None;
    }
}

pub fn draw(scene: &Scene, frame: &mut Frame) {
    for shape_selected in &scene.selected.shapes {
        let shape = scene
            .vgc_data
            .get_shape(shape_selected.shape_index)
            .unwrap();
        {
            let refs_coord_type = shape.get_coords_of_shape_tmp();
            for ref_coord_type in refs_coord_type {
                let coord_state = shape_selected.coord_state(&ref_coord_type);
                let coord = ref_coord_type.borrow();
                let color = match coord_state {
                    CoordState::Hover => Color::from_rgb8(0x0E, 0x90, 0xAA),
                    CoordState::Selected => Color::from_rgb8(0x3A, 0xD1, 0xEF),
                    CoordState::None => Color::from_rgb8(0x3A, 0xD1, 0xEF),
                };
                let center = Point::new(coord.x, coord.y * 1.0 / scene.vgc_data.ratio as f32);
                frame.fill(
                    &Path::circle(center, scene.camera.fixed_length(5.0)),
                    Fill::from(color),
                );
            }
        }

        shape.visit_full_curves(|_, p0, cp0, cp1, p1| {
            let from = Point::new(p0.x, p0.y * 1.0 / scene.vgc_data.ratio as f32);

            let to = Point::new(cp0.x, cp0.y * 1.0 / scene.vgc_data.ratio as f32);
            let stroke = Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb8(0x3A, 0xD1, 0xEF));
            frame.stroke(&Path::line(from, to), stroke);

            let from = Point::new(cp1.x, cp1.y * 1.0 / scene.vgc_data.ratio as f32);

            let to = Point::new(p1.x, p1.y * 1.0 / scene.vgc_data.ratio as f32);
            let stroke = Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb8(0x3A, 0xD1, 0xEF));
            frame.stroke(&Path::line(from, to), stroke);
        });

        let path = Path::new(|p| {
            let start_coord = shape.start.borrow();
            p.move_to(Point::new(
                start_coord.x,
                start_coord.y * 1.0 / scene.vgc_data.ratio as f32,
            ));

            shape.visit_full_curves(move |_, _, cp0, cp1, p1| {
                p.bezier_curve_to(
                    Point::new(cp0.x, cp0.y * 1.0 / scene.vgc_data.ratio as f32),
                    Point::new(cp1.x, cp1.y * 1.0 / scene.vgc_data.ratio as f32),
                    Point::new(p1.x, p1.y * 1.0 / scene.vgc_data.ratio as f32),
                );
            });
        });

        let stroke = Stroke::default()
            .with_width(1.0)
            .with_color(Color::from_rgba8(0x3A, 0xD1, 0xEF, 0.5));
        frame.stroke(&path, stroke);
    }
}

pub fn draw_closest_pt(scene: &Scene, frame: &mut Frame, pos: Point) {
    let mut min_distance = std::f32::MAX;
    let mut min_coord = Coord::new(0.0, 0.0);
    let pos = scene.camera.project(pos);
    for shape_selected in &scene.selected.shapes {
        let shape = scene
            .vgc_data
            .get_shape(shape_selected.shape_index)
            .unwrap();

        let (_, _, distance, coord) = shape.closest_curve(&Coord::new(pos.x, pos.y));

        if distance < min_distance {
            min_distance = distance;
            min_coord = coord;
        }
    }

    if min_distance > scene.camera.fixed_length(10.0) {
        return;
    }

    let color = Color::from_rgb8(0x0E, 0x90, 0xAA);
    let center = Point::new(min_coord.x, min_coord.y * 1.0 / scene.vgc_data.ratio as f32);
    frame.fill(
        &Path::circle(center, scene.camera.fixed_length(3.0)),
        Fill::from(color),
    );
}
