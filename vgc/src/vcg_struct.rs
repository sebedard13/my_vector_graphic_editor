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

    pub fn toggle_separate_join_handle(&mut self, coord_ds: &mut CoordDS, index: usize) {
        if self.is_handle(index) {
            println!("separate");
            self.separate_handle(coord_ds, index);
        } else {
            println!("join");
            self.join_handle(coord_ds, index);
        }
    }

    fn is_handle(&self, index: usize) -> bool {
        let curve = &self.curves[index];
        curve.cp0 == curve.p1 || curve.cp1 == curve.p1
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


        println!("coords_separate: {:?}", coords_separate);


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

fn cubic_bezier_derivative(t: f32, p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Coord {
    3.0 * (1.0 - t) * (1.0 - t) * (cp0 - p0)
        + 6.0 * (1.0 - t) * t * (cp1 - cp0)
        + 3.0 * t * t * (p1 - cp1)
}

/// Return the normalized tangent vector at t of curve defined by p0, cp0, cp1, p1
/// Panic if no tangent vector found by having the same point for p0, cp0, cp1 and p1 
fn tangent_vector(t: f32, p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Coord {
    let tangent_vector = cubic_bezier_derivative(t, p0, cp0, cp1, p1);
    if tangent_vector != (Coord { x: 0.0, y: 0.0 }) {
        //Normalize vector
        return tangent_vector.normalize();
    }

    //Exception with (t = 1 and cp1 == p1) or (t = 0 and cp0 == p0)
    let t = t.clamp(0.0001, 0.9999);

    let tangent_vector = cubic_bezier_derivative(t, p0, cp0, cp1, p1);
    if tangent_vector != (Coord { x: 0.0, y: 0.0 }) {
        //Normalize vector
        return tangent_vector.normalize();
    }


    panic!("No tangent vector found for t: {}, c0: {:?}, h0: {:?}, h1: {:?}, c1: {:?}", t, p0, cp0, cp1, p1);
}


/// Return two points to create a smooth curve at t of curve defined by p0, cp0, cp1, p1
/// if t = 0.0 or 1.0 use tangent_cornor_pts() to use the sum of vector of two curve
fn tangent_pts(t: f32, p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> [Coord; 2] {
  
    if p0 == p1 && p0 == cp0 && p0 == cp1 {
        return [
            p0 - &Coord { x: 0.1, y: 0.1 },
            p0 + &Coord { x: 0.1, y: 0.1 },
        ];
    }


    let tangent_vector = cubic_bezier_derivative(t, p0, cp0, cp1, p1);

    let coord = cubic_bezier(t, p0, cp0, cp1, p1);

    let t_at = {
        let t_x = (p0.x - p1.x).abs();
        let t_y = (p0.y - p1.y).abs();
        if t_x > t_y {
            t_x / 2.0
        } else {
            t_y / 2.0
        }
    };

   
    [
        Coord {
            x: coord.x - t_at * tangent_vector.x,
            y: coord.y - t_at * tangent_vector.y,
        },
        Coord {
            x: coord.x + t_at * tangent_vector.x,
            y: coord.y + t_at * tangent_vector.y,
        },
    ]
}


#[cfg(test)]
mod test{
    use crate::{vcg_struct::tangent_vector, coord::Coord};


    #[test]
    fn tangent_vector_corner_same() {
        let p0 = Coord { x: 1.0, y: 1.0 };
        let cp0 = Coord { x: 1.0, y: 1.0 };
        let cp1 = Coord { x: 1.0, y: 1.0 };
        let p1 = Coord { x: 1.0, y: 1.0 };

        let tangent = tangent_vector(1.0, &p0, &cp0, &cp1, &p1);

        println!("tangent: {:?}", tangent);
        assert!(!tangent.x.is_nan());
        assert!(!tangent.y.is_nan());

        assert_ne!(tangent,  Coord { x: 0.0, y: 0.0 });
    }
}