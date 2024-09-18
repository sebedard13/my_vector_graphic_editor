use common::{
    pures::Affine,
    types::{Coord, Length2d},
    Rgba,
};
use coord::DbCoord;
use serde::{Deserialize, Serialize};

use crate::{
    scene::{Layer, LayerId, Scene},
    DrawingContext,
};

use super::LayerType;

pub mod boolean;
pub mod coord;
pub mod cubic_path;
pub mod curve;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub id: LayerId,
    pub path: Vec<DbCoord>,
    pub color: Rgba,
}

impl Shape {
    pub fn render(&self, renderer: &mut dyn DrawingContext) -> Result<(), String> {
        let transform = renderer.get_transform()?;
        let coords: Vec<Coord> = self.path.iter().map(|c| transform * c.coord).collect();
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
}

impl Scene {
    pub fn shape_insert(&mut self, mut shape: Shape) -> LayerId {
        if shape.id == LayerId::null() {
            shape.id.update();
        }
        let id = shape.id;
        shape.curves_path_update_id();
        self.layers.push(Layer {
            id: shape.id,
            value: LayerType::Shape(shape),
            name: format!("Shape {}", id.value()),
        });

        id
    }

    pub fn shape_select(&self, index: LayerId) -> Option<&Shape> {
        if let Some(LayerType::Shape(value)) = self.layer_select(index) {
            Some(value)
        } else {
            None
        }
    }

    pub fn shape_select_mut(&mut self, index: LayerId) -> Option<&mut Shape> {
        if let Some(LayerType::Shape(value)) = self.layer_select_mut(index) {
            Some(value)
        } else {
            None
        }
    }

    pub fn shape_put(&mut self, shape: Shape) {
        let index = self
            .layers
            .iter()
            .position(|l| l.id == shape.id)
            .expect("Valid shape id");
        self.layers[index].value = LayerType::Shape(shape);
    }

    pub fn shape_select_contains(&self, coord: &Coord) -> Option<&Shape> {
        let find_result = self.layers.iter().find(|l| {
            if let LayerType::Shape(shape) = &l.value {
                if shape.contains(coord) {
                    return true;
                }
            }
            false
        });
        find_result.and_then(|l| {
            if let LayerType::Shape(shape) = &l.value {
                Some(shape)
            } else {
                None
            }
        })
    }

    pub fn shape_select_contains_mut(&mut self, coord: &Coord) -> Option<&mut Shape> {
        let find_result = self.layers.iter_mut().find(|l| {
            if let LayerType::Shape(shape) = &l.value {
                if shape.contains(coord) {
                    return true;
                }
            }
            false
        });
        find_result.and_then(|l| {
            if let LayerType::Shape(shape) = &mut l.value {
                Some(shape)
            } else {
                None
            }
        })
    }

    // pub fn shapes_closest(&self, coord: &Coord) -> Vec<(usize, usize, f32, Coord)> {
    //     let mut vec = Vec::new();
    //     for (shape_index, shape) in self.shapes.iter().enumerate() {
    //         let (curve_index, _, distance, coord) = shape.closest_curve(coord);

    //         vec.push((shape_index, curve_index, distance, coord));
    //     }
    //     vec.sort_by(|(_, _, distance1, _), (_, _, distance2, _)| {
    //         distance1
    //             .partial_cmp(distance2)
    //             .expect("No Nan value possible")
    //     });
    //     vec
    // }
}

impl Default for Shape {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape {
    pub fn new() -> Self {
        Shape {
            id: LayerId::null(),
            path: Vec::new(),
            color: Rgba::transparent(),
        }
    }

    //List of coordinates of curves. The first coordinate is the start of the curve.
    pub fn new_from_path(coords: Vec<coord::DbCoord>, transform: Affine) -> Self {
        assert_eq!((coords.len() - 1) % 3, 0);
        let mut shape = Shape::new();

        for coord in &coords {
            shape.path.push(coord.transform(&transform));
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
        for coord in coords.iter().skip(1) {
            shape.path.push(coord.transform(&transform));
            shape.path.push(coord.transform(&transform));
            shape.path.push(coord.transform(&transform));
        }

        shape.path.push(coords[0].transform(&transform));
        shape.path.push(coords[0].transform(&transform));

        shape.close();
        shape
    }

    pub fn new_circle(center: Coord, radius: Length2d) -> Self {
        let transform = Affine::identity().scale(radius).translate(center);

        //https://spencermortensen.com/articles/bezier-circle/
        let a = 1.000_055_2;
        let b = 0.553_426_86;
        let c = 0.998_735_85;

        let start = coord::DbCoord::new(0.0, a);

        let coords = vec![
            start,
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
                write!(&mut path, "M {} {} ", coord.x, coord.y).expect("Write should be ok");
            } else if (i - 1) % 3 == 0 {
                write!(&mut path, "C {} {} ", coord.x, coord.y).expect("Write should be ok");
            } else {
                write!(&mut path, "{} {} ", coord.x, coord.y).expect("Write should be ok");
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

            let t_intersections = curve.intersection_with_y(coord.y);
            for t in t_intersections {
                let x = curve.cubic_bezier(t).x;
                if x > coord.x {
                    count += 1;
                }
            }
        }
        count % 2 == 1
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    ///Creates a new shape from a string of path coordinates.
    /// # Example
    /// 
    /// ```rust
    /// use database::Shape;
    /// let shape = Shape::quick_from_string("M 0 0 C 1.000 1.000 2 2 0 0 Z");
    /// assert_eq!(shape.curves_len(), 1);
    /// assert_eq! (shape.curve_select(0).unwrap().cp0.coord().x, 1.0);
    /// ```
    pub fn quick_from_string(string: &str) -> Self {
        let coords = string_to_coords(string);
        Shape::new_from_path(coords, Affine::identity())
    }
}

fn string_to_coords(string: &str) -> Vec<DbCoord> {
    let mut coords = vec![];
    let mut x = 0.0;
    let mut i = 0;
    for current in string.split_whitespace() {
        if current == "M" || current == "L" || current == "C" || current == "Z" {
            continue;
        }
        if i % 2 == 0 {
            x = current.parse::<f32>().unwrap();
        } else {
            coords.push(DbCoord::new(x, current.parse::<f32>().unwrap()));
        }
        i += 1;
    }

    coords
}

#[cfg(test)]
mod test {
    use crate::scene::render::MockDrawingContext;

    use super::*;

    #[test]
    fn test_shape_render() {
        let shape = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(1.0, 1.0));
        let mut renderer = MockDrawingContext::default();

        shape.render(&mut renderer).expect("Render should be ok");
    }

    #[test]
    fn given_shape_when_contains_outside_on_axis_then_return_false() {
        let shape = Shape::quick_from_string(
            "M 540 0 C 540 0 
            540 45 540 45 C 540 45 
            585 45 585 45 C 585 45
            585 0 585 0 C 585 0
            540 0 540 0 Z",
        );
        let coord = Coord::new(0.0, 0.0);
        assert!(!shape.contains(&coord));
    }
}
