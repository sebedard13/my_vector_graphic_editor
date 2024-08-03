use common::{pures::Affine, types::Coord, Rgba};

use crate::{scene::shape::boolean::ShapeIntersection, Shape};

use super::DrawingContext;

const F_WIDTH: f32 = 45.0; // px
const I_WIDTH: i32 = F_WIDTH as i32; // px

#[allow(dead_code)]
pub fn render_transparent_grid(renderer: &mut impl DrawingContext) -> Result<(), String> {
    let transform = renderer.get_transform()?;
    log::info!("transform: {:?}", transform);
    let max_view = renderer.get_max_view()?;
    log::info!("max_view: {:?}", max_view);

    let max_view_top_left = Coord {
        c: max_view.top_left.c,
    };

    let max_view_bottom_right = Coord {
        c: max_view.bottom_right.c,
    };

    let max_view_shape = Shape::new_from_lines(
        vec![
            max_view_top_left.into(),
            Coord::new(max_view_bottom_right.x(), max_view_top_left.y()).into(),
            max_view_bottom_right.into(),
            Coord::new(max_view_top_left.x(), max_view_bottom_right.y()).into(),
        ],
        Affine::identity(),
    );
    let scene = Shape::new_from_lines(
        vec![
            Coord::new(-1.0, -1.0).into(),
            Coord::new(-1.0, 1.0).into(),
            Coord::new(1.0, 1.0).into(),
            Coord::new(1.0, -1.0).into(),
        ],
        transform,
    );

    let (scene_corner0, scene_corner2) = {
        let mut min = Coord::new(f32::INFINITY, f32::INFINITY);
        let mut max = Coord::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
        for curve in scene.curves() {
            min = Coord::min(&min, &curve.p1.coord);
            max = Coord::max(&max, &curve.p1.coord);
        }
        (min, max)
    };

    let x_start = ((scene_corner0.x() / (F_WIDTH * 2.0)).floor() as i32) * I_WIDTH * 2;
    let y_start = ((scene_corner0.y() / (F_WIDTH * 2.0)).floor() as i32) * I_WIDTH * 2;
    let x_end = ((scene_corner2.x() / (F_WIDTH * 2.0)).ceil() as i32) * I_WIDTH * 2;
    let y_end = ((scene_corner2.y() / (F_WIDTH * 2.0)).ceil() as i32) * I_WIDTH * 2;

    //White grid
    renderer.fill_background(&Rgba::white())?;
    renderer.set_fill(&Rgba::from_small_hex_string("#cac7c7"))?;

    println!("Max view path: {:?}", max_view_shape.path());
    for x in (x_start..x_end).step_by((I_WIDTH * 2) as usize) {
        for y in (y_start..y_end).step_by((I_WIDTH * 2) as usize) {
            let corner0 = Coord::new(x as f32, y as f32);
            let corner1 = Coord::new(x as f32, (y + I_WIDTH) as f32);
            let corner2 = Coord::new((x + I_WIDTH) as f32, (y + I_WIDTH) as f32);
            let corner3 = Coord::new((x + I_WIDTH) as f32, y as f32);

            let shape = Shape::new_from_lines(
                vec![
                    corner0.into(),
                    corner1.into(),
                    corner2.into(),
                    corner3.into(),
                ],
                Affine::identity(),
            );

            let mut shapes: Vec<Shape> = vec![];

            match shape.intersection(&scene) {
                ShapeIntersection::A => shapes.push(shape),
                ShapeIntersection::B => {
                    scene.render(renderer)?;

                    return Ok(());
                }
                ShapeIntersection::New(mut new_shapes) => {
                    shapes.append(&mut new_shapes);
                }
                ShapeIntersection::None => continue,
            }

            for shape in shapes {
                match shape.intersection(&max_view_shape) {
                    ShapeIntersection::A => shape.render(renderer)?,
                    ShapeIntersection::B => continue,
                    ShapeIntersection::New(new_shapes) => {
                        for shape in new_shapes {
                            shape.render(renderer)?;
                        }
                    }
                    ShapeIntersection::None => continue,
                }
            }
        }
    }

    for x in ((x_start - I_WIDTH)..x_end).step_by((I_WIDTH * 2) as usize) {
        for y in ((y_start - I_WIDTH)..y_end).step_by((I_WIDTH * 2) as usize) {
            let corner0 = Coord::new(x as f32, y as f32);
            let corner1 = Coord::new(x as f32, (y + I_WIDTH) as f32);
            let corner2 = Coord::new((x + I_WIDTH) as f32, (y + I_WIDTH) as f32);
            let corner3 = Coord::new((x + I_WIDTH) as f32, y as f32);

            let shape = Shape::new_from_lines(
                vec![
                    corner0.into(),
                    corner1.into(),
                    corner2.into(),
                    corner3.into(),
                ],
                Affine::identity(),
            );

            let mut shapes = vec![];
            match shape.intersection(&scene) {
                ShapeIntersection::A => shapes.push(shape),
                ShapeIntersection::B => {
                    scene.render(renderer)?;

                    return Ok(());
                }
                ShapeIntersection::New(mut new_shapes) => {
                    shapes.append(&mut new_shapes);
                }
                ShapeIntersection::None => continue,
            }

            for shape in shapes {
                match shape.intersection(&max_view_shape) {
                    ShapeIntersection::A => shape.render(renderer)?,
                    ShapeIntersection::B => continue,
                    ShapeIntersection::New(new_shapes) => {
                        for shape in new_shapes {
                            shape.render(renderer)?;
                        }
                    }
                    ShapeIntersection::None => continue,
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use common::{
        pures::Vec2,
        types::{ScreenCoord, ScreenRect},
    };

    use crate::scene::render::MockDrawingContext;

    use super::*;

    #[test]

    fn test_render_transparent_grid() {
        //transform: Affine { m00: 375.0, m10: 0.0, m01: 0.0, m11: 250.0, m02: 279.5, m12: 191.5 }
        // max_view: ScreenRect { top_left: ScreenCoord { c: Vec2 { x: 0.0, y: 0.0 } }, bottom_right: ScreenCoord { c: Vec2 { x: 559.0, y: 383.0 } }

        let transform = Affine::identity()
            .scale(Vec2::new(375.0, 250.0))
            .translate(Vec2::new(279.5, 191.5));
        let mut renderer = MockDrawingContext {
            transform: transform,
            max_view: ScreenRect {
                top_left: ScreenCoord::new(0.0, 0.0),
                bottom_right: ScreenCoord::new(559.0, 383.0),
            },
        };
        render_transparent_grid(&mut renderer).unwrap();
    }
}
