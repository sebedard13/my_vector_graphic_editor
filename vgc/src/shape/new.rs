use std::{cell::RefCell, rc::Rc};

use common::{
    pures::{Affine, Vec2},
    types::Coord,
    Rgba,
};

use super::Shape;

impl Shape {
    pub fn new(start: Coord, fill: Rgba) -> Self {
        let start = Rc::new(RefCell::new(start));
        Shape {
            start,
            curves: vec![],
            color: fill,
        }
    }

    //List of coordinates of curves. The first coordinate is the start of the curve.
    pub fn new_from_path(coords: &Vec<Coord>, transform: Affine, fill: Rgba) -> Self {
        let start = coords[0].transform(&transform);
        let mut shape = Shape::new(start, fill);

        for i in (1..coords.len()).step_by(3) {
            shape.push_coord(
                Rc::new(RefCell::new(coords[i].transform(&transform))),
                Rc::new(RefCell::new(coords[i + 1].transform(&transform))),
                Rc::new(RefCell::new(coords[i + 2].transform(&transform))),
            );
        }

        shape
    }

    pub fn new_circle(center: Coord, radius: Vec2, fill: Rgba) -> Self {
        let transform = Affine::identity().scale(radius).translate(center.c);

        //https://spencermortensen.com/articles/bezier-circle/
        let a = 1.000_055_19;
        let b = 0.553_426_86;
        let c = 0.998_735_85;

        let coords = vec![
            Coord::new(0.0, a),
            Coord::new(b, c),
            Coord::new(c, b),
            Coord::new(a, 0.0),
            Coord::new(c, -b),
            Coord::new(b, -c),
            Coord::new(0.0, -a),
            Coord::new(-b, -c),
            Coord::new(-c, -b),
            Coord::new(-a, 0.0),
            Coord::new(-c, b),
            Coord::new(-b, c),
            Coord::new(0.0, a),
        ];

        Shape::new_from_path(&coords, transform, fill)
    }

    ///Creates a new shape from a string of path coordinates.
    /// # Example
    /// ```
    /// use vgc::shape::Shape;
    /// let shape = Shape::quick_from_string("M 0 0 C 1.000 1.000 2 2 0 0 Z");
    /// assert_eq!(shape.curves.len(), 1);
    /// assert_eq! (shape.curves[0].cp0.borrow().c.x, 1.0);
    /// ```
    pub fn quick_from_string(string: &str) -> Self {
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
                coords.push(Coord::new(x, current.parse::<f32>().unwrap()));
            }
            i += 1;
        }

        Shape::new_from_path(&coords, Affine::identity(), Rgba::new(0, 0, 0, 255))
    }
}
