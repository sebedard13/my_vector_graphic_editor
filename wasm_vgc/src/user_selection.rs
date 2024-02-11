use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use common::types::Coord;
use common::Rgba;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;

use crate::{CanvasContent, Point};

/// Return true if the cursor is in the radius of the center
///```rust
///
/// let cursor = Cursor::Available(Point::new(10.0, 10.0));
/// let center = Point::new(0.0, 0.0);
/// let radius = 5.0;
/// assert_eq!(point_in_radius(cursor, center, radius), false);
/// let cursor = Cursor::Available(Point::new(-3.0, 0.0));
/// assert_eq!(point_in_radius(cursor, center, radius), true);
///```
pub fn point_in_radius(point: &Point, center: &Point, radius: f32) -> bool {
    let x = point.x - center.x;
    let y = point.y - center.y;
    let distance = x * x + y * y;
    distance < (radius * radius)
}

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct Selected {
    #[wasm_bindgen(skip)]
    pub shapes: Vec<SelectedShape>,
}

#[derive(Debug, Default)]
pub struct SelectedShape {
    pub shape_index: usize,
    pub coords: Vec<Rc<RefCell<Coord>>>,
    pub hover_coord: Option<Rc<RefCell<Coord>>>,
}

#[wasm_bindgen]
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

#[wasm_bindgen]
impl Selected {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn get_selected_colors(&self, canvas_context: &CanvasContent) -> Vec<Rgba> {
        let shapes = &self.shapes;

        if shapes.is_empty() {
            return vec![];
        }

        let mut colors = Vec::new();
        for shape_selected in shapes {
            let shape = shape_selected.shape_index;
            let shape = match canvas_context.vgc_data.get_shape(shape) {
                Some(shape) => shape,
                None => continue,
            };

            if colors.contains(&shape.color) {
                continue;
            }
            colors.push(shape.color.clone());
        }

        colors
    }

    pub fn change_hover(&mut self, canvas_context: &CanvasContent, cursor_position: Point) {
        'shape_loop: for shape_selected in &mut self.shapes {
            let shape = canvas_context
                .vgc_data
                .get_shape(shape_selected.shape_index)
                .unwrap();
            let coords = shape.get_coords_of_shape_tmp();
            for ref_coord_type in coords {
                let coord = ref_coord_type.borrow();

                if point_in_radius(
                    &cursor_position,
                    &Point::new(coord.x(), coord.y()),
                    canvas_context.camera.fixed_length(12.0),
                ) {
                    shape_selected.hover_coord = Some(ref_coord_type.clone());
                    continue 'shape_loop;
                }
            }
            shape_selected.hover_coord = None;
        }
    }

    pub fn change_selection(&mut self, canvas_context: &CanvasContent, start_press: Point) {
        let shapes = &mut self.shapes;
        if shapes.is_empty() {
            //Add shape
            let closest_shapes = canvas_context
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
                let shape = canvas_context
                    .vgc_data
                    .get_shape(shape_selected.shape_index)
                    .unwrap();
                let coords = shape.get_coords_of_shape_tmp();
                for ref_coord_type in coords {
                    let coord = ref_coord_type.borrow();
                    if point_in_radius(
                        &start_press,
                        &Point::new(coord.x(), coord.y()),
                        canvas_context.camera.fixed_length(12.0),
                    ) {
                        shape_selected.coords.push(ref_coord_type.clone());
                        return;
                    }
                }
            }
        }
    }

    pub fn add_selection(&mut self, canvas_context: &CanvasContent, start_press: Point) {
        //Coord
        for shape_selected in &mut self.shapes {
            let shape = canvas_context
                .vgc_data
                .get_shape(shape_selected.shape_index)
                .unwrap();
            let coords = shape.get_coords_of_shape_tmp();
            for ref_coord_type in coords {
                let coord = ref_coord_type.borrow();
                if point_in_radius(
                    &start_press,
                    &Point::new(coord.x(), coord.y()),
                    canvas_context.camera.fixed_length(12.0),
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

        let shapes = canvas_context
            .vgc_data
            .shapes_closest(&Coord::new(start_press.x, start_press.y));

        let first = shapes.first();
        if let Some((shape_index, ..)) = first {
            let shapes = &mut self.shapes;
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

#[wasm_bindgen]
pub fn draw(selected: &Selected, canvas_context: &CanvasContent, ctx: &CanvasRenderingContext2d) {
    for shape_selected in &selected.shapes {
        let shape = canvas_context
            .vgc_data
            .get_shape(shape_selected.shape_index)
            .unwrap();

        ctx.set_line_width(2.0);
        ctx.set_stroke_style(&Rgba::new(0x3A, 0xD1, 0xEF, 255).to_css_string().into());

        //Draw line between cp and p
        shape.visit_full_curves(|_, p0, cp0, cp1, p1| {
            ctx.begin_path();
            let from = canvas_context.camera.unproject((p0.x(), p0.y()));

            ctx.move_to(from.0 as f64, from.1 as f64);
            let to = canvas_context.camera.unproject((cp0.x(), cp0.y()));
            ctx.line_to(to.0 as f64, to.1 as f64);
            ctx.stroke();

            ctx.begin_path();
            let from = canvas_context.camera.unproject((cp1.x(), cp1.y()));
            ctx.move_to(from.0 as f64, from.1 as f64);
            let to = canvas_context.camera.unproject((p1.x(), p1.y()));
            ctx.line_to(to.0 as f64, to.1 as f64);
            ctx.stroke();
        });

        let refs_coord_type = shape.get_coords_of_shape_tmp();
        for ref_coord_type in refs_coord_type {
            let coord_state = shape_selected.coord_state(&ref_coord_type);
            let coord = ref_coord_type.borrow();
            let color = match coord_state {
                CoordState::Hover => Rgba::new(0x0E, 0x90, 0xAA, 255),
                CoordState::Selected => Rgba::new(0x3A, 0xD1, 0xEF, 255),
                CoordState::None => Rgba::new(0xA1, 0xE9, 0xF7, 255),
            };
            let center = Point::new(coord.x(), coord.y() as f32);
            let center = canvas_context.camera.unproject((center.x, center.y));

            ctx.begin_path();
            ctx.set_fill_style(&color.to_css_string().into());
            ctx.ellipse(
                center.0.into(),
                center.1.into(),
                5.0,
                5.0,
                PI / 4.0,
                0.0,
                2.0 * PI,
            )
            .expect("valid");
            ctx.fill();
        }

        ctx.begin_path();
        let start_coord = shape.start.borrow();
        let start_coord = canvas_context
            .camera
            .unproject((start_coord.x(), start_coord.y()));
        ctx.move_to(start_coord.0.into(), start_coord.1.into());

        shape.visit_full_curves(move |_, _, cp0, cp1, p1| {
            let cp0 = canvas_context.camera.unproject((cp0.x(), cp0.y()));
            let cp1 = canvas_context.camera.unproject((cp1.x(), cp1.y()));
            let p1 = canvas_context.camera.unproject((p1.x(), p1.y()));

            ctx.bezier_curve_to(
                cp0.0.into(),
                cp0.1.into(),
                cp1.0.into(),
                cp1.1.into(),
                p1.0.into(),
                p1.1.into(),
            );
        });

        ctx.set_line_width(1.0);
        ctx.set_stroke_style(&Rgba::new(0x3A, 0xD1, 0xEF, 0x80).to_css_string().into());
        ctx.stroke();
    }
}

#[wasm_bindgen]
pub fn draw_closest_pt(
    selected: &Selected,
    canvas_context: &CanvasContent,
    ctx: &CanvasRenderingContext2d,
    mouse_pos: Point,
) {
    let mut min_distance = std::f32::MAX;
    let mut min_coord = Coord::new(0.0, 0.0);
    let pos = canvas_context.camera.project((mouse_pos.x, mouse_pos.y));
    for shape_selected in &selected.shapes {
        let shape = canvas_context
            .vgc_data
            .get_shape(shape_selected.shape_index)
            .unwrap();

        let (_, _, distance, coord) = shape.closest_curve(&Coord::new(pos.0, pos.1));

        if distance < min_distance {
            min_distance = distance;
            min_coord = coord;
        }
    }

    if min_distance > canvas_context.camera.fixed_length(10.0) {
        return;
    }

    let color = Rgba::new(0x0E, 0x90, 0xAA, 255);
    let center = Point::new(min_coord.x(), min_coord.y());

    let center = canvas_context.camera.unproject((center.x, center.y));

    ctx.begin_path();
    ctx.set_fill_style(&color.to_css_string().into());
    ctx.ellipse(
        center.0.into(),
        center.1.into(),
        3.0,
        3.0,
        PI / 4.0,
        0.0,
        2.0 * PI,
    )
    .expect("valid");
    ctx.fill();
}
