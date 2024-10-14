use common::types::{Coord, ScreenLength2d};
use common::Rgba;

use crate::{CoordId, LayerId};
use common::math::point_in_radius;

use crate::user_context::SceneUserContext;

#[derive(Debug, Default)]
pub struct UserSelection {
    pub shapes: Vec<SelectedShape>,
    pub mouse_position: Option<Coord>,
    pub hover_coord: Option<HoverCoord>,
    pub color: Rgba,
    pub stroke_size: f32,
    pub stroke_color: Rgba,
}

#[derive(Debug, Default)]
pub struct SelectedShape {
    pub shape_id: LayerId,
    pub coords: Vec<CoordId>,
}

#[derive(Debug, Clone)]
pub struct HoverCoord {
    pub shape_index: LayerId,
    pub id: CoordId,
}

impl HoverCoord {
    pub fn new(shape_index: LayerId, coord: CoordId) -> Self {
        Self {
            shape_index,
            id: coord,
        }
    }
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

impl UserSelection {
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

    pub fn get_selected_colors(&self, canvas_context: &SceneUserContext) -> Vec<Rgba> {
        let shapes = &self.shapes;

        if shapes.is_empty() {
            return vec![];
        }

        let mut colors = Vec::new();
        for shape_selected in shapes {
            let shape = shape_selected.shape_id;
            let shape = match canvas_context.scene().shape_select(shape) {
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

    pub fn get_selected_stroke_sizes(&self, canvas_context: &SceneUserContext) -> Vec<f32> {
        let shapes = &self.shapes;

        if shapes.is_empty() {
            return vec![];
        }

        let mut stroke_sizes = Vec::new();
        for shape_selected in shapes {
            let shape = shape_selected.shape_id;
            let shape = match canvas_context.scene().shape_select(shape) {
                Some(shape) => shape,
                None => continue,
            };

            stroke_sizes.push(shape.stroke.size);
        }

        stroke_sizes
    }

    pub fn get_selected_stroke_colors(&self, canvas_context: &SceneUserContext) -> Vec<Rgba> {
        let shapes = &self.shapes;

        if shapes.is_empty() {
            return vec![];
        }

        let mut stroke_colors = Vec::new();
        for shape_selected in shapes {
            let shape = shape_selected.shape_id;
            let shape = match canvas_context.scene().shape_select(shape) {
                Some(shape) => shape,
                None => continue,
            };

            stroke_colors.push(shape.stroke.color.clone());
        }

        stroke_colors
    }

    pub fn change_hover(&mut self, canvas_context: &SceneUserContext, cursor_position: Coord) {
        'shape_loop: for shape_selected in &mut self.shapes {
            let shape = canvas_context
                .scene()
                .shape_select(shape_selected.shape_id)
                .unwrap();
            let db_coords = &shape.path;
            for db_coord in db_coords {
                if point_in_radius(
                    cursor_position,
                    db_coord.coord(),
                    canvas_context
                        .camera
                        .transform_to_length2d(ScreenLength2d::new(12.0, 12.0)),
                ) {
                    self.hover_coord = Some(HoverCoord::new(shape_selected.shape_id, db_coord.id));
                    continue 'shape_loop;
                }
            }
            self.hover_coord = None;
        }
    }

    pub fn change_selection(&mut self, canvas_context: &SceneUserContext, start_press: Coord) {
        let selected_shapes = &mut self.shapes;

        if selected_shapes.is_empty() {
            //Add shape
            let closest_shapes = canvas_context.scene().shape_select_contains(&start_press);

            if let Some(shape) = closest_shapes {
                let pos = selected_shapes
                    .iter()
                    .position(|shape_selected| shape_selected.shape_id == shape.id);

                match pos {
                    Some(index) => {
                        let elment = selected_shapes.remove(index);
                        selected_shapes.clear();
                        selected_shapes.push(elment);
                    }
                    None => {
                        selected_shapes.clear();
                        selected_shapes.push(SelectedShape::new(shape.id));
                    }
                }
            }
        } else {
            //Coord

            for selected_shape in selected_shapes.iter_mut() {
                selected_shape.coords.clear();
            }

            for selected_shape in selected_shapes {
                let shape = canvas_context
                    .scene()
                    .shape_select(selected_shape.shape_id)
                    .unwrap();
                let coords = &shape.path;
                for db_coord in coords {
                    let coord = db_coord.coord();
                    if point_in_radius(
                        start_press,
                        coord,
                        canvas_context
                            .camera
                            .transform_to_length2d(ScreenLength2d::new(12.0, 12.0)),
                    ) {
                        selected_shape.coords.push(db_coord.id);
                        return;
                    }
                }
            }
        }
    }

    pub fn add_selection(&mut self, canvas_context: &SceneUserContext, start_press: Coord) {
        //Coord
        for shape_selected in &mut self.shapes {
            let shape = canvas_context
                .scene()
                .shape_select(shape_selected.shape_id)
                .unwrap();
            let db_coords = &shape.path;
            for db_coord in db_coords {
                let coord = db_coord.coord();
                if point_in_radius(
                    start_press,
                    coord,
                    canvas_context
                        .camera
                        .transform_to_length2d(ScreenLength2d::new(12.0, 12.0)),
                ) {
                    let pos = shape_selected
                        .coords
                        .iter()
                        .position(|coord_id| *coord_id == db_coord.id);
                    match pos {
                        Some(index) => {
                            shape_selected.coords.swap_remove(index);
                        }
                        None => {
                            shape_selected.coords.push(db_coord.id);
                        }
                    }
                    return;
                }
            }
        }

        let shape = canvas_context.scene().shape_select_contains(&start_press);

        if let Some(shape) = shape {
            let shapes = &mut self.shapes;
            let pos = shapes
                .iter()
                .position(|shape_selected| shape_selected.shape_id == shape.id);

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
                    shapes.push(SelectedShape::new(shape.id));
                }
            }
        }
    }

    pub fn remove_shape(&mut self, shape_index: LayerId) {
        let shapes = &mut self.shapes;
        let pos = shapes
            .iter()
            .position(|shape_selected| shape_selected.shape_id == shape_index);

        if let Some(index) = pos {
            shapes.remove(index);
        }
    }
}

pub enum CoordState {
    Hover,
    Selected,
    None,
}

impl SelectedShape {
    pub fn new(shape_index: LayerId) -> Self {
        Self {
            shape_id: shape_index,
            coords: Vec::new(),
        }
    }

    pub fn coord_state(&self, selected: &UserSelection, coord_ref: CoordId) -> CoordState {
        match &selected.hover_coord {
            Some(hover_coord) if hover_coord.id == coord_ref => CoordState::Hover,
            _ => match self.coords.iter().find(|coord| **coord == coord_ref) {
                Some(_) => CoordState::Selected,
                None => CoordState::None,
            },
        }
    }
}
