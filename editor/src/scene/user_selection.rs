use std::{cell::RefCell, rc::Rc};

use iced::{
    widget::canvas::{Fill, Frame, Path, Stroke},
    Color, Point,
};
use vgc::coord::Coord;

use crate::scene::{point_in_radius, Scene};

#[derive(Debug, Default)]
pub struct Selected {
    pub shapes: Vec<SelectedShape>,
}

#[derive(Debug, Default)]
pub struct SelectedShape {
    pub shape_index: usize,
    pub coords: Vec<Rc<RefCell<Coord>>>,
    pub hover_coord: Option<Rc<RefCell<Coord>>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectedLevel {
    None,
    Shape,
    Coord,
}
impl SelectedLevel {
    pub fn minus(&self) -> Self {
        match self {
            SelectedLevel::None => SelectedLevel::None,
            SelectedLevel::Shape => SelectedLevel::None,
            SelectedLevel::Coord => SelectedLevel::Shape,
        }
    }

    pub fn plus(&self) -> Self {
        match self {
            SelectedLevel::None => SelectedLevel::Shape,
            SelectedLevel::Shape => SelectedLevel::Coord,
            SelectedLevel::Coord => SelectedLevel::Coord,
        }
    }
}

impl Selected {
    pub fn get_selected_level(&self) -> SelectedLevel {
        if self.shapes.is_empty() {
            return SelectedLevel::None;
        }

        for shape_selected in &self.shapes {
            if !shape_selected.coords.is_empty() {
                return SelectedLevel::Coord;
            }
        }

        SelectedLevel::Shape
    }

    pub fn clear_to_level(&mut self, selected_level: SelectedLevel) {
        match selected_level {
            SelectedLevel::None => {
                self.shapes.clear();
            }
            SelectedLevel::Shape => {
                for shape_selected in &mut self.shapes {
                    shape_selected.coords.clear();
                }
            }
            SelectedLevel::Coord => {}
        }
    }
}

pub enum ColorSelected {
    None,
    MultipleNotSame,
    Single(Color),
}

pub fn get_color_selected(scene: &Scene) -> ColorSelected {
    let shapes = &scene.selected.shapes;

    if shapes.is_empty() {
        return ColorSelected::None;
    }

    let mut color = None;
    for shape_selected in shapes {
        let shape = shape_selected.shape_index;
        let shape = match scene.vgc_data.get_shape(shape) {
            Some(shape) => shape,
            None => continue,
        };
        let shape_color = &shape.color;
        match color {
            None => color = Some(shape_color),
            Some(c) if c != shape_color => return ColorSelected::MultipleNotSame,
            _ => {}
        }
    }

    match color {
        None => ColorSelected::None,
        Some(c) => ColorSelected::Single(Color::from_rgba8(c.r, c.g, c.b, c.a as f32 / 255.0)),
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
            shape_index,
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
                &cursor_position,
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

        //Draw line between cp and p
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

        let refs_coord_type = shape.get_coords_of_shape_tmp();
        for ref_coord_type in refs_coord_type {
            let coord_state = shape_selected.coord_state(&ref_coord_type);
            let coord = ref_coord_type.borrow();
            let color = match coord_state {
                CoordState::Hover => Color::from_rgb8(0x0E, 0x90, 0xAA),
                CoordState::Selected => Color::from_rgb8(0x3A, 0xD1, 0xEF),
                CoordState::None => Color::from_rgb8(0xA1, 0xE9, 0xF7),
            };
            let center = Point::new(coord.x, coord.y * 1.0 / scene.vgc_data.ratio as f32);
            frame.fill(
                &Path::circle(center, scene.camera.fixed_length(5.0)),
                Fill::from(color),
            );
        }

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

pub fn change_selection(scene: &mut Scene, start_press: Point) {
    let shapes = &mut scene.selected.shapes;
    if shapes.is_empty() {
        //Add shape
        let closest_shapes = scene
            .vgc_data
            .shapes_closest(&Coord::new(start_press.x, start_press.y));

        let first = closest_shapes.first();
        if let Some((shape_index, ..)) = first {
            let pos = shapes
                .iter()
                .position(|shape_selected| shape_selected.shape_index == *shape_index);

            match pos {
                Some(index) => {
                    let elment = shapes.remove(index);
                    shapes.clear();
                    shapes.push(elment);
                }
                None => {
                    shapes.clear();
                    shapes.push(SelectedShape::new(*shape_index));
                }
            }
        }
    } else {
        //Coord

        for shape_selected in shapes.iter_mut() {
            shape_selected.coords.clear();
        }

        for shape_selected in shapes {
            let shape = scene
                .vgc_data
                .get_shape(shape_selected.shape_index)
                .unwrap();
            let coords = shape.get_coords_of_shape_tmp();
            for ref_coord_type in coords {
                let coord = ref_coord_type.borrow();
                if point_in_radius(
                    &start_press,
                    &Point::new(coord.x, coord.y),
                    scene.camera.fixed_length(12.0),
                ) {
                    shape_selected.coords.push(ref_coord_type.clone());
                    return;
                }
            }
        }
    }
}

pub fn add_selection(scene: &mut Scene, start_press: Point) {
    //Coord
    for shape_selected in &mut scene.selected.shapes {
        let shape = scene
            .vgc_data
            .get_shape(shape_selected.shape_index)
            .unwrap();
        let coords = shape.get_coords_of_shape_tmp();
        for ref_coord_type in coords {
            let coord = ref_coord_type.borrow();
            if point_in_radius(
                &start_press,
                &Point::new(coord.x, coord.y),
                scene.camera.fixed_length(12.0),
            ) {
                let pos = shape_selected
                    .coords
                    .iter()
                    .position(|coord| *coord == ref_coord_type);
                match pos {
                    Some(index) => {
                        shape_selected.coords.swap_remove(index);
                    }
                    None => {
                        shape_selected.coords.push(ref_coord_type.clone());
                    }
                }
                return;
            }
        }
    }

    let shapes = scene
        .vgc_data
        .shapes_closest(&Coord::new(start_press.x, start_press.y));

    let first = shapes.first();
    if let Some((shape_index, ..)) = first {
        let shapes = &mut scene.selected.shapes;
        let pos = shapes
            .iter()
            .position(|shape_selected| shape_selected.shape_index == *shape_index);

        match pos {
            Some(index) => {
                if shapes
                    .get(index)
                    .expect("Valid index because founds with iter().position")
                    .coords
                    .is_empty()
                {
                    shapes.swap_remove(index);
                }
            }
            None => {
                shapes.push(SelectedShape::new(*shape_index));
            }
        }
    }
}
