use std::any::Any;

use common::{
    pures::{Affine, Vec2},
    types::{Coord, Length2d},
    Rgba,
};
use coord::DbCoord;

use crate::{
    impl_layer_value,
    scene::{Layer, LayerId, Scene},
    DrawingContext,
};

use super::{LayerType, LayerValue};

pub mod coord;
pub mod cubic_path;
pub mod curve;

#[derive(Debug, Clone)]
pub struct Shape {
    pub id: LayerId,
    pub path: Vec<DbCoord>,
    pub color: Rgba,
}

impl_layer_value!(
    Shape,
    fn render(&self, renderer: &mut dyn DrawingContext) -> Result<(), String> {
        let transform = renderer.get_transform()?;
        let coords: Vec<Coord> = self
            .path
            .iter()
            .map(|c| c.coord.transform(&transform))
            .collect();
        renderer.set_fill(&self.color)?;
        renderer.start_shape(&coords[0])?;
        for i in (1..(coords.len() - 1)).step_by(3) {
            let cp0 = coords[i];
            let cp1 = coords[i + 1];
            let p1 = coords[i + 2];
            renderer.move_curve(&cp0, &cp1, &p1)?;
        }
        renderer.close_shape()?;
        renderer.end()?;
        Ok(())
    }
);

impl Scene {
    pub fn shape_insert(&mut self, mut shape: Shape) -> LayerId {
        if shape.id == LayerId::null() {
            shape.id.update();
        }
        let id = shape.id;
        shape.curves_path_update_id();
        self.layers.push(Layer {
            id: shape.id,
            layer_type: LayerType::Shape,
            value: Box::new(shape),
        });

        id
    }

    pub fn shape_select(&self, index: LayerId) -> Option<&Shape> {
        self.layer_select::<Shape>(index)
    }

    pub fn shape_select_mut(&mut self, index: LayerId) -> Option<&mut Shape> {
        self.layer_select_mut::<Shape>(index)
    }

    pub fn shape_put(&mut self, shape: Shape) {
        let index = self
            .layers
            .iter()
            .position(|l| l.id == shape.id)
            .expect("Valid shape id");
        self.layers[index].value = Box::new(shape);
    }

    pub fn shape_select_contains(&self, coord: &Coord) -> Option<&Shape> {
        let find_result = self.layers.iter().find(|l| {
            if let Some(shape) = l.value.as_any().downcast_ref::<Shape>() {
                if shape.contains(coord) {
                    return true;
                }
            }
            false
        });
        find_result.and_then(|l| l.value.as_any().downcast_ref::<Shape>())
    }

    pub fn shape_select_contains_mut(&mut self, coord: &Coord) -> Option<&mut Shape> {
        let find_result = self.layers.iter_mut().find(|l| {
            if let Some(shape) = l.value.as_any().downcast_ref::<Shape>() {
                if shape.contains(coord) {
                    return true;
                }
            }
            false
        });
        find_result.and_then(|l| l.value.as_any_mut().downcast_mut::<Shape>())
    }
}

impl Shape {
    pub fn new() -> Self {
        Shape {
            id: LayerId::null(),
            path: Vec::new(),
            color: Rgba::black(),
        }
    }

    //List of coordinates of curves. The first coordinate is the start of the curve.
    pub fn new_from_path(coords: Vec<coord::DbCoord>, transform: Affine) -> Self {
        assert_eq!((coords.len() - 1) % 3, 0);
        let mut shape = Shape::new();

        for i in 0..coords.len() {
            shape.path.push(coords[i].transform(&transform));
        }

        shape
    }

    //List of coordinates of lines. It will close the shape.
    pub fn new_from_lines(coords: Vec<coord::DbCoord>, transform: Affine) -> Self {
        let mut shape = Shape::new();
        if coords.is_empty() {
            return shape;
        }

        shape.path.push(coords[0].transform(&transform));
        shape.path.push(coords[0].transform(&transform));
        for i in 1..coords.len() {
            shape.path.push(coords[i].transform(&transform));
            shape.path.push(coords[i].transform(&transform));
            shape.path.push(coords[i].transform(&transform));
        }

        shape.path.push(coords[0].transform(&transform));
        shape.path.push(coords[0].transform(&transform));

        shape
    }

    pub fn new_circle(center: Coord, radius: Length2d) -> Self {
        let transform = Affine::identity().scale(radius.c).translate(center.c);

        //https://spencermortensen.com/articles/bezier-circle/
        let a = 1.000_055_19;
        let b = 0.553_426_86;
        let c = 0.998_735_85;

        let start = coord::DbCoord::new(0.0, a);

        let coords = vec![
            start.clone(),
            DbCoord::new(b, c),
            DbCoord::new(c, b),
            DbCoord::new(a, 0.0),
            DbCoord::new(c, -b),
            DbCoord::new(b, -c),
            DbCoord::new(0.0, -a),
            DbCoord::new(-b, -c),
            DbCoord::new(-c, -b),
            DbCoord::new(-a, 0.0),
            DbCoord::new(-c, b),
            DbCoord::new(-b, c),
            start,
        ];

        Shape::new_from_path(coords, transform)
    }

    pub fn path(&self) -> String {
        use std::fmt::Write;

        let mut path = String::new();
        for i in 0..self.path.len() {
            let coord = self.path[i].coord;
            if i == 0 {
                write!(&mut path, "M {} {} ", coord.x(), coord.y()).expect("Write should be ok");
            } else if (i - 1) % 3 == 0 {
                write!(&mut path, "C {} {} ", coord.x(), coord.y()).expect("Write should be ok");
            } else {
                write!(&mut path, "{} {} ", coord.x(), coord.y()).expect("Write should be ok");
            }
        }
        write!(&mut path, "Z").expect("Write should be ok");
        path
    }

    /// Return true if the coord is inside the shape
    /// Use the even-odd rule
    pub fn contains(&self, coord: &Coord) -> bool {
        let mut count = 0;
        for curve_index in 0..self.curves_len() {
            let curve = self.curve_select(curve_index).expect("Curve should exist");

            let t_intersections = curve.intersection_with_y(coord.y());
            for t in t_intersections {
                let x = curve.cubic_bezier(t).x();
                if x > coord.x() {
                    count += 1;
                }
            }
        }
        count % 2 == 1
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
}

#[cfg(test)]
mod test {
    use crate::scene::render::MockDrawingContext;

    use super::*;

    #[test]
    fn test_shape_render() {
        let shape = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(1.0, 1.0));
        let mut renderer = MockDrawingContext {};

        shape.render(&mut renderer).expect("Render should be ok");
    }
}
