use common::types::{Coord, ScreenLength2d};
use common::{dbg_str, Rgba};
use common::{math::point_in_radius, types::ScreenCoord};

use crate::scene::shape::boolean::ShapeUnion;
use crate::user_context::user_selection::SelectedShape;
use crate::{LayerId, Shape};

use super::user_selection::UserSelection;
use super::SceneUserContext;

impl SceneUserContext {
    pub fn set_color_of(&mut self, selected: &mut UserSelection, color: Rgba) {
        selected.color = color.clone();
        for shape in &selected.shapes {
            if let Some(shape) = self.scene.shape_select_mut(shape.shape_index) {
                shape.color = color.clone();
            }
        }
    }
    pub fn move_coords_of(&mut self, selected: &UserSelection, movement: ScreenLength2d) {
        let movement = self.camera.transform_to_length2d_with_rotation(movement);

        for selected_shape in &selected.shapes {
            let shape = self
                .scene
                .shape_select_mut(selected_shape.shape_index)
                .expect("not 404");
            for coord_id in &selected_shape.coords {
                let (coord_id, coord) = {
                    let coord = shape.coord_select_mut(*coord_id).expect("not 404");
                    let res_vec2 = coord.coord() + Coord::from(movement);
                    (coord.id, Coord::new(res_vec2.x, res_vec2.y))
                };

                shape.coord_set(coord_id, coord);
            }
        }
    }

    pub fn add_or_remove_coord(
        &mut self,
        selected: &mut UserSelection,
        mouse_position: ScreenCoord,
    ) {
        let vgc_data = &mut self.scene;
        let camera = &mut self.camera;
        let pos = camera.project(mouse_position);

        log::debug!("hover_coord: {:?}", selected.hover_coord);
        if selected.hover_coord.is_some() {
            let hover_coord = selected.hover_coord.take().unwrap();
            let shape = vgc_data
                .shape_select_mut(hover_coord.shape_index)
                .expect("Not 404");
            shape.coord_delete(hover_coord.id).expect("Not 404");

            if shape.is_empty() {
                vgc_data.layer_delete(hover_coord.shape_index);
                selected.remove_shape(hover_coord.shape_index);
            }
            return;
        }

        // if click is on the path of curve, add a point
        let mut min_distance = std::f32::MAX;
        let mut min_shape_index = LayerId::null();
        let mut min_curve_index = 0;
        let mut min_coord = Coord::new(100.0, 100.0);
        let mut min_t = 0.0;

        for shape_selected in &selected.shapes {
            let shape = vgc_data.shape_select(shape_selected.shape_index).unwrap();

            let (curve_index, t, distance, coord) = shape.closest_curve(&pos);

            if distance < min_distance {
                min_distance = distance;
                min_shape_index = shape_selected.shape_index;
                min_curve_index = curve_index;
                min_coord = coord;
                min_t = t;
            }
        }

        let fixed_length = camera.transform_to_length2d(ScreenLength2d::new(10.0, 10.0));
        if point_in_radius(pos, min_coord, fixed_length) {
            let shape = vgc_data
                .shape_select_mut(min_shape_index)
                .expect("Shape is valid because it was selected");

            shape.curve_insert_smooth(min_curve_index, min_t);
        }
    }

    pub fn toggle_handle(&mut self, selected: &UserSelection) {
        let vgc_data = &mut self.scene;

        if let Some(hover_coord) = &selected.hover_coord {
            let shape = vgc_data
                .shape_select_mut(hover_coord.shape_index)
                .expect("Not 404");
            log::debug!("hover_coord: {:?}, {:?}", shape, hover_coord);
            let curve_index = shape
                .curve_select_of_coord_id(hover_coord.id)
                .expect("Not 404");
            shape.toggle_separate_join_handle(curve_index);
        }
    }

    pub fn draw_shape(&mut self, selected: &mut UserSelection) {
        let vgc_data = &mut self.scene;
        let camera = &mut self.camera;
        if selected.mouse_position.is_none() {
            return;
        }
        let mouse_position = selected.mouse_position.unwrap();

        let radius = camera.transform_to_length2d_no_scale(ScreenLength2d::new(50.0, 50.0));

        let mut shape = Shape::new_circle(mouse_position, radius);
        shape.color = selected.color.clone();

        if selected.shapes.is_empty() {
            let id = vgc_data.shape_insert(shape);
            vgc_data.layer_move_top(id);
            selected.shapes.push(SelectedShape::new(id));
            return;
        }

        while selected.shapes.len() > 1 {
            selected.shapes.pop();
        }

        //for selected shape try to union the new shape
        let shape_selected = &selected.shapes[0];

        let result = {
            let selected_shape = vgc_data
                .shape_select(shape_selected.shape_index)
                .expect("Not 404");
            log::debug!(
                "{}",
                dbg_str!(
                    "start Union A: {} \nB: {}",
                    selected_shape.path(),
                    shape.path()
                )
            );
            selected_shape.union(&shape)
        };
        log::info!("{}", dbg_str!("Union good"));
        let selected_shape = vgc_data
            .shape_select_mut(shape_selected.shape_index)
            .expect("Not 404");
        match result {
            ShapeUnion::New(new_shape) => {
                selected_shape.path = new_shape.path;
            }
            ShapeUnion::A => {}
            ShapeUnion::B => {
                selected_shape.path = shape.path;
            }
            ShapeUnion::None => {}
        }
    }
}

impl SceneUserContext {
    pub fn load(vec: Vec<u8>) -> Result<SceneUserContext, String> {
        let main_slice = vec.as_slice();
        let scene_user_context_data = postcard::from_bytes::<SceneUserContext>(main_slice);
        if scene_user_context_data.is_err() {
            let error = scene_user_context_data.unwrap_err();
            log::error!("Error: {:?}", error);
            return Err("Deserialization should be valid".to_string());
        }

        Ok(scene_user_context_data.unwrap())
    }

    pub fn save(&self) -> Result<Vec<u8>, String> {
        let vec = postcard::to_allocvec(self).map_err(|_| "Serizalization should be valid")?;
        Ok(vec)
    }
}
