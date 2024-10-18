use anyhow::Context;
use common::{
    math::point_in_radius,
    types::{Coord, ScreenLength2d},
    Rgba,
};

use crate::{DrawingContext, SceneUserContext, Shape, UserSelection};

use super::user_selection::CoordState;

impl SceneUserContext {
    pub fn draw(
        &self,
        selected: &UserSelection,
        ctx: &mut dyn DrawingContext,
    ) -> Result<(), String> {
        for shape_selected in &selected.shapes {
            let shape = self
                .scene()
                .shape_select(shape_selected.shape_id)
                .context(format!("Shape id {:?} not found", shape_selected.shape_id))
                .map_err(|e| e.to_string())?;

            let t = ctx.get_transform()?;

            ctx.set_stroke(&Rgba::new(0x3A, 0xD1, 0xEF, 255), 2.0)?;

            //Draw line between cp and p
            for curve in shape.curves() {
                ctx.start_shape(&(t * curve.p0.coord()))?;
                ctx.move_curve(
                    &(t * curve.p0.coord()),
                    &(t * curve.cp0.coord()),
                    &(t * curve.cp0.coord()),
                )?;
                ctx.close_shape()?;

                ctx.start_shape(&(t * curve.cp1.coord()))?;
                ctx.move_curve(
                    &(t * curve.cp1.coord()),
                    &(t * curve.p1.coord()),
                    &(t * curve.p1.coord()),
                )?;
                ctx.close_shape()?;
            }
            ctx.set_stroke(&Rgba::transparent(), 0.0)?;

            //Draw coord
            for db_coord in shape.path.iter() {
                let coord_state = shape_selected.coord_state(selected, db_coord.id);
                let color = match coord_state {
                    CoordState::Hover => Rgba::new(0x0E, 0x90, 0xAA, 255),
                    CoordState::Selected => Rgba::new(0x3A, 0xD1, 0xEF, 255),
                    CoordState::None => Rgba::new(0xA1, 0xE9, 0xF7, 255),
                };

                let radius = ScreenLength2d::new(5.0, 5.0);

                //TODO Don't mix buisness logic with rendering Shape with database id and rendering
                let mut circle =
                    Shape::new_circle(db_coord.coord(), self.camera.transform_to_length2d(radius));
                circle.color = color;
                circle.render(ctx)?;
            }

            //Draw shape selection border
            ctx.set_fill(&Rgba::transparent())?;
            ctx.set_stroke(&Rgba::new(0x3A, 0xD1, 0xEF, 0x80), 1.0)?;
            ctx.start_shape(&(t * shape.path.first().unwrap().coord()))?;

            for curve in shape.curves() {
                ctx.move_curve(
                    &(t * curve.cp0.coord()),
                    &(t * curve.cp1.coord()),
                    &(t * curve.p1.coord()),
                )?;
            }
            ctx.close_shape()?;
            ctx.set_stroke(&Rgba::transparent(), 0.0)?;
        }

        let t = ctx.get_transform()?;
        ctx.set_stroke(&Rgba::black(), 1.0)?;
        ctx.set_fill(&Rgba::transparent())?;
        ctx.start_shape(&(t * Coord::new(-1.0, -1.0)))?;
        ctx.move_line(&(t * Coord::new(1.0, -1.0)))?;
        ctx.move_line(&(t * Coord::new(1.0, 1.0)))?;
        ctx.move_line(&(t * Coord::new(-1.0, 1.0)))?;
        ctx.close_shape()?;
        ctx.set_stroke(&Rgba::transparent(), 0.0)?;

        Ok(())
    }

    pub fn draw_closest_pt(
        &self,
        selected: &UserSelection,
        ctx: &mut dyn DrawingContext,
    ) -> Result<(), String> {
        let mut min_distance = std::f32::MAX;
        let mut min_coord = Coord::new(0.0, 0.0);
        let pos = selected.mouse_position;

        if pos.is_none() {
            return Ok(());
        }
        let pos = pos.unwrap();

        for shape_selected in &selected.shapes {
            let shape = self
                .scene()
                .shape_select(shape_selected.shape_id)
                .expect("not 404");

            let (_, _, distance, coord) = shape.closest_curve(&pos);

            if distance < min_distance {
                min_distance = distance;
                min_coord = coord;
            }
        }

        if !point_in_radius(
            pos,
            min_coord,
            self.camera
                .transform_to_length2d(ScreenLength2d::new(10.0, 10.0)),
        ) {
            return Ok(());
        }

        let color = Rgba::new(0x0E, 0x90, 0xAA, 255);

        let center = min_coord;
        let radius = ScreenLength2d::new(3.0, 3.0);
        let radius = self.camera.transform_to_length2d(radius);
        let mut circle = Shape::new_circle(center, radius);
        circle.color = color;
        circle.render(ctx)?;
        Ok(())
    }
}
