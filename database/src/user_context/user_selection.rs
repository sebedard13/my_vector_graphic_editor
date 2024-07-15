use common::types::{Coord, ScreenLength2d};
use common::Rgba;

use crate::{CoordId, LayerId};
use common::math::point_in_radius;

use crate::user_context::SceneUserContext;

#[derive(Debug, Default)]
pub struct UserSelection {
    pub shapes: Vec<SelectedShape>,
    pub hover_coord: Option<HoverCoord>,
}

#[derive(Debug, Default)]
pub struct SelectedShape {
    pub shape_index: LayerId,
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
            let shape = shape_selected.shape_index;
            let shape = match canvas_context.scene.shape_select(shape) {
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

    pub fn change_hover(&mut self, canvas_context: &SceneUserContext, cursor_position: Coord) {
        'shape_loop: for shape_selected in &mut self.shapes {
            let shape = canvas_context
                .scene
                .shape_select(shape_selected.shape_index)
                .unwrap();
            let db_coords = &shape.path;
            for db_coord in db_coords {
                if point_in_radius(
                    &cursor_position.c,
                    &db_coord.coord.c,
                    &canvas_context
                        .camera
                        .transform_to_length2d(ScreenLength2d::new(12.0, 12.0))
                        .c,
                ) {
                    self.hover_coord =
                        Some(HoverCoord::new(shape_selected.shape_index, db_coord.id));
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
            let closest_shapes = canvas_context.scene.shape_select_contains(&start_press);

            if let Some(shape) = closest_shapes {
                let pos = selected_shapes
                    .iter()
                    .position(|shape_selected| shape_selected.shape_index == shape.id);

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
                    .scene
                    .shape_select(selected_shape.shape_index)
                    .unwrap();
                let coords = &shape.path;
                for db_coord in coords {
                    let coord = db_coord.coord;
                    if point_in_radius(
                        &start_press.c,
                        &coord.c,
                        &canvas_context
                            .camera
                            .transform_to_length2d(ScreenLength2d::new(12.0, 12.0))
                            .c,
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
                .scene
                .shape_select(shape_selected.shape_index)
                .unwrap();
            let db_coords = &shape.path;
            for db_coord in db_coords {
                let coord = db_coord.coord;
                if point_in_radius(
                    &start_press.c,
                    &coord.c,
                    &canvas_context
                        .camera
                        .transform_to_length2d(ScreenLength2d::new(12.0, 12.0))
                        .c,
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

        let shape = canvas_context.scene.shape_select_contains(&start_press);

        if let Some(shape) = shape {
            let shapes = &mut self.shapes;
            let pos = shapes
                .iter()
                .position(|shape_selected| shape_selected.shape_index == shape.id);

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
            .position(|shape_selected| shape_selected.shape_index == shape_index);

        if let Some(index) = pos {
            shapes.remove(index);
        }
    }
}

enum CoordState {
    Hover,
    Selected,
    None,
}

impl SelectedShape {
    pub fn new(shape_index: LayerId) -> Self {
        Self {
            shape_index,
            coords: Vec::new(),
        }
    }

    fn coord_state(&self, selected: &UserSelection, coord_ref: &CoordId) -> CoordState {
        match &selected.hover_coord {
            Some(hover_coord) if hover_coord.id == *coord_ref => CoordState::Hover,
            _ => match self.coords.iter().find(|coord| *coord == coord_ref) {
                Some(_) => CoordState::Selected,
                None => CoordState::None,
            },
        }
    }
}

// pub fn draw(selected: &Selected, canvas_context: &CanvasContent, ctx: &CanvasRenderingContext2d) {
//     for shape_selected in &selected.shapes {
//         let shape = canvas_context
//             .vgc_data
//             .shape_select(shape_selected.shape_index)
//             .unwrap();

//         ctx.set_line_width(2.0);
//         ctx.set_stroke_style(&Rgba::new(0x3A, 0xD1, 0xEF, 255).to_css_string().into());

//         //Draw line between cp and p
//         shape.visit_full_curves(|_, p0, cp0, cp1, p1| {
//             ctx.begin_path();
//             let from = canvas_context.camera.unproject(p0.clone());

//             ctx.move_to(from.c.x as f64, from.c.y as f64);
//             let to = canvas_context.camera.unproject(cp0.clone());
//             ctx.line_to(to.c.x as f64, to.c.y as f64);
//             ctx.stroke();

//             ctx.begin_path();
//             let from = canvas_context.camera.unproject(cp1.clone());
//             ctx.move_to(from.c.x as f64, from.c.y as f64);
//             let to = canvas_context.camera.unproject(p1.clone());
//             ctx.line_to(to.c.x as f64, to.c.y as f64);
//             ctx.stroke();
//         });

//         let refs_coord_type = shape.get_coords_of_shape_tmp();
//         for ref_coord_type in refs_coord_type {
//             let coord_state = shape_selected.coord_state(&selected, &ref_coord_type);
//             let coord = ref_coord_type.borrow();
//             let color = match coord_state {
//                 CoordState::Hover => Rgba::new(0x0E, 0x90, 0xAA, 255),
//                 CoordState::Selected => Rgba::new(0x3A, 0xD1, 0xEF, 255),
//                 CoordState::None => Rgba::new(0xA1, 0xE9, 0xF7, 255),
//             };
//             let center = canvas_context.camera.unproject(coord.clone());

//             ctx.begin_path();
//             ctx.set_fill_style(&color.to_css_string().into());
//             ctx.ellipse(
//                 center.c.x as f64,
//                 center.c.y as f64,
//                 5.0,
//                 5.0,
//                 PI / 4.0,
//                 0.0,
//                 2.0 * PI,
//             )
//             .expect("valid");
//             ctx.fill();
//         }

//         ctx.begin_path();
//         let start_coord = shape.start.borrow();
//         let start_coord = canvas_context.camera.unproject(start_coord.clone());
//         ctx.move_to(start_coord.c.x.into(), start_coord.c.y.into());

//         shape.visit_full_curves(move |_, _, cp0, cp1, p1| {
//             let cp0 = canvas_context.camera.unproject(cp0.clone());
//             let cp1 = canvas_context.camera.unproject(cp1.clone());
//             let p1 = canvas_context.camera.unproject(p1.clone());

//             ctx.bezier_curve_to(
//                 cp0.c.x.into(),
//                 cp0.c.y.into(),
//                 cp1.c.x.into(),
//                 cp1.c.y.into(),
//                 p1.c.x.into(),
//                 p1.c.y.into(),
//             );
//         });

//         ctx.set_line_width(1.0);
//         ctx.set_stroke_style(&Rgba::new(0x3A, 0xD1, 0xEF, 0x80).to_css_string().into());
//         ctx.stroke();
//     }
// }

// pub fn draw_closest_pt(
//     selected: &Selected,
//     canvas_context: &CanvasContent,
//     ctx: &CanvasRenderingContext2d,
//     mouse_pos: ScreenCoord,
// ) {
//     let mut min_distance = std::f32::MAX;
//     let mut min_coord = Coord::new(0.0, 0.0);
//     let pos = canvas_context.camera.project(mouse_pos.clone());

//     for shape_selected in &selected.shapes {
//         let shape = canvas_context
//             .vgc_data
//             .shape_select(shape_selected.shape_index)
//             .unwrap();

//         let (_, _, distance, coord) = shape.closest_curve(&pos);

//         if distance < min_distance {
//             min_distance = distance;
//             min_coord = coord;
//         }
//     }

//     if !point_in_radius(
//         &pos.c,
//         &min_coord.c,
//         &canvas_context
//             .camera
//             .transform_to_length2d(ScreenLength2d::new(10.0, 10.0))
//             .c,
//     ) {
//         return;
//     }

//     let color = Rgba::new(0x0E, 0x90, 0xAA, 255);

//     let center = canvas_context.camera.unproject(min_coord);

//     ctx.begin_path();
//     ctx.set_fill_style(&color.to_css_string().into());
//     ctx.ellipse(
//         center.c.x as f64,
//         center.c.y as f64,
//         3.0,
//         3.0,
//         PI / 4.0,
//         0.0,
//         2.0 * PI,
//     )
//     .expect("valid");
//     ctx.fill();
// }