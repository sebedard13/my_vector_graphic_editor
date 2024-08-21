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
                .scene
                .shape_select(shape_selected.shape_index)
                .expect("not 404");

            let t = ctx.get_transform()?;

            ctx.set_stroke(&Rgba::new(0x3A, 0xD1, 0xEF, 255), 2.0)?;

            //Draw line between cp and p
            for curve in shape.curves() {
                ctx.start_shape(&curve.p0.coord().transform(&t))?;
                ctx.move_curve(
                    &curve.p0.coord().transform(&t),
                    &curve.cp0.coord().transform(&t),
                    &curve.cp0.coord().transform(&t),
                )?;
                ctx.close_shape()?;

                ctx.start_shape(&curve.cp1.coord().transform(&t))?;
                ctx.move_curve(
                    &curve.cp1.coord().transform(&t),
                    &curve.p1.coord().transform(&t),
                    &curve.p1.coord().transform(&t),
                )?;
                ctx.close_shape()?;
            }
            ctx.set_stroke(&Rgba::transparent(), 0.0)?;

            //Draw coord
            for db_coord in shape.path.iter() {
                let coord_state = shape_selected.coord_state(&selected, db_coord.id);
                let color = match coord_state {
                    CoordState::Hover => Rgba::new(0x0E, 0x90, 0xAA, 255),
                    CoordState::Selected => Rgba::new(0x3A, 0xD1, 0xEF, 255),
                    CoordState::None => Rgba::new(0xA1, 0xE9, 0xF7, 255),
                };

                let radius = ScreenLength2d::new(5.0, 5.0);

                let mut circle =
                    Shape::new_circle(db_coord.coord(), self.camera.transform_to_length2d(radius));
                circle.color = color;
                circle.render(ctx)?;
            }

            //Draw shape selection border
            ctx.set_fill(&Rgba::transparent())?;
            ctx.set_stroke(&Rgba::new(0x3A, 0xD1, 0xEF, 0x80), 1.0)?;
            ctx.start_shape(&shape.path.first().unwrap().coord().transform(&t))?;

            for curve in shape.curves() {
                ctx.move_curve(
                    &curve.cp0.coord().transform(&t),
                    &curve.cp1.coord().transform(&t),
                    &curve.p1.coord().transform(&t),
                )?;
            }
            ctx.close_shape()?;
            ctx.set_stroke(&Rgba::transparent(), 0.0)?;
        }

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
                .scene
                .shape_select(shape_selected.shape_index)
                .expect("not 404");

            let (_, _, distance, coord) = shape.closest_curve(&pos);

            if distance < min_distance {
                min_distance = distance;
                min_coord = coord;
            }
        }

        if !point_in_radius(
            &pos.c,
            &min_coord.c,
            &self
                .camera
                .transform_to_length2d(ScreenLength2d::new(10.0, 10.0))
                .c,
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