use serde::{Deserialize, Serialize};
use std::mem::swap;

use crate::coord::{Coord, CoordDS, CoordIndex};

#[derive(Deserialize, Serialize, Debug)]
pub struct Shape {
    pub start: CoordIndex,
    pub curves: Vec<Curve>,
    pub color: Rgba,
}

impl Shape {
    pub fn add_coord(&mut self, coord_ds: &mut CoordDS, mut curve: Curve, index: usize) {
        let curve_after = self.curves.get_mut(index).expect("Index should be valid because we should not be able to add a curve at the end of the shape because the last element close the curve with a link to the start coord in shape");

        swap(&mut curve.cp0, &mut curve.cp1);
        swap(&mut curve.cp0, &mut curve_after.cp0);
        self.curves.insert(index, curve);
    }
    pub fn separate_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        let p0 = {
            if index == 0 {
                coord_ds.get(&self.start)
            } else {
                coord_ds.get(&self.curves[index - 1].p1)
            }
        };
        let cp0 = coord_ds.get(&self.curves[index].cp0);
        let cp1 = coord_ds.get(&self.curves[index].cp1);
        let c1 = coord_ds.get(&self.curves[index].p1);

        let coords_separate = tangent_pts(1.0, p0, cp0, cp1, c1);

        let coord_index0 = coord_ds.insert(coords_separate[0].clone()); //TODO clone not good
        self.curves[index].cp1 = coord_index0;
        let coord_index1 = coord_ds.insert(coords_separate[1].clone());
        if index + 1 >= self.curves.len() {
            self.curves[index].cp0 = coord_index1;
        } else {
            self.curves[index + 1].cp1 = coord_index1;
        }
    }

    pub fn join_cp0_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        let coord_index = &self.curves[index].p1;
        let curve_after = (index + 1) % self.curves.len();
        self.curves[curve_after].cp0 = coord_index.clone();

        //TODO delete unused coord
    }

    pub fn join_cp1_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        let coord_index = &self.curves[index].p1;
        self.curves[index].cp1 = coord_index.clone();

        //TODO delete unused coord
    }

    pub fn join_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        self.join_cp0_handle(coord_ds, index);
        self.join_cp1_handle(coord_ds, index);
    }

    pub fn to_path(&self, coord_ds: &CoordDS) -> String {
        let mut path = String::new();
        let start = coord_ds.get(&self.start);
        path.push_str(&format!("M {} {}", start.x, start.y));
        for curve in &self.curves {
            let cp0 = coord_ds.get(&curve.cp0);
            let cp1 = coord_ds.get(&curve.cp1);
            let p1 = coord_ds.get(&curve.p1);
            path.push_str(&format!(
                " C {} {} {} {} {} {}",
                cp0.x, cp0.y, cp1.x, cp1.y, p1.x, p1.y
            ));
        }
        path.push_str(" Z");
        path
    }
}

// A curve is a cubic bezier curve, defined by 4 points:
// - cp0 is the control point for the point before the current curve
// - cp1 is the control point before the current point
// - p1 is the current point
//
// The curve is drawn from the previous curve point [i-1].p1, with [i].h1 and [i].h2 as control points and [i].p1 for the final points.
#[derive(Deserialize, Serialize, Debug)]
pub struct Curve {
    pub cp0: CoordIndex,
    pub cp1: CoordIndex,
    pub p1: CoordIndex,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Curve {
    pub fn new(c1: CoordIndex, c2: CoordIndex, p: CoordIndex) -> Curve {
        Curve {
            cp0: c1,
            cp1: c2,
            p1: p,
        }
    }
}

fn cubic_bezier(t: f32, c0: &Coord, h0: &Coord, h1: &Coord, p1: &Coord) -> Coord {
    (1.0 - t) * (1.0 - t) * (1.0 - t) * c0
        + 3.0 * (1.0 - t) * (1.0 - t) * t * h0
        + 3.0 * (1.0 - t) * t * t * h1
        + t * t * t * p1
}

fn cubic_bezier_derivative(t: f32, c0: &Coord, h0: &Coord, h1: &Coord, c1: &Coord) -> Coord {
    3.0 * (1.0 - t) * (1.0 - t) * (h0 - c0)
        + 6.0 * (1.0 - t) * t * (h1 - h0)
        + 3.0 * t * t * (c1 - h1)
}

fn tangent_pts(t: f32, c0: &Coord, h0: &Coord, h1: &Coord, c1: &Coord) -> [Coord; 2] {
    if c0 == c1 && c0 == h0 && c0 == h1 {
        return [
            c0 - &Coord { x: 0.1, y: 0.1 },
            c0 + &Coord { x: 0.1, y: 0.1 },
        ];
    }

    let vector = cubic_bezier_derivative(t, c0, h0, h1, c1);
    let coord = cubic_bezier(t, c0, h0, h1, c1);

    let t_at = {
        let t_x = (c0.x - c1.x).abs();
        let t_y = (c0.y - c1.y).abs();
        if t_x > t_y {
            t_x / 2.0
        } else {
            t_y / 2.0
        }
    };

    let vector = &vector / (vector.x.powi(2) + vector.y.powi(2)).sqrt();

    [
        Coord {
            x: coord.x - t_at * vector.x,
            y: coord.y - t_at * vector.y,
        },
        Coord {
            x: coord.x + t_at * vector.x,
            y: coord.y + t_at * vector.y,
        },
    ]
}
