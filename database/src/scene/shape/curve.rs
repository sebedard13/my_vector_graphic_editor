use super::{coord::DbCoord, Shape};
use common::types::Coord;

use crate::math;

pub struct Curve<'a> {
    pub p0: &'a DbCoord,
    pub cp0: &'a DbCoord,
    pub cp1: &'a DbCoord,
    pub p1: &'a DbCoord,
}

impl<'a> Curve<'a> {
    pub fn is_straight(&self) -> bool {
        self.p0.id == self.cp0.id && self.cp1.id == self.p1.id
    }

    pub fn right_handle_free(&self) -> bool {
        self.cp1.id != self.p1.id
    }

    pub fn left_handle_free(&self) -> bool {
        self.cp0.id != self.p0.id
    }
}

impl Shape {
    /// Visit each curve and calculate the closest point on the curve to the coord
    ///
    /// Return (curve index, t value , distance, closest point)
    pub fn closest_curve(&self, coord: &Coord) -> (usize, f32, f32, Coord) {
        let mut min_distance = std::f32::MAX;
        let mut min_index = 0;
        let mut min_t = 0.0;
        let mut min_coord = Coord::new(-1000.0, -1000.0);

        for curve_index in 0..self.curves_len() {
            let curve = self.curve_select(curve_index).expect("Curve should exist");
            let (t_min, distance, coord_closest) = curve.t_closest(coord);

            if distance < min_distance {
                min_distance = distance;
                min_index = curve_index;
                min_t = t_min;
                min_coord = coord_closest;
            }
        }
        (min_index, min_t, min_distance, min_coord)
    }

    pub fn is_closed(&self) -> bool {
        self.path[0].id == self.path[self.path.len() - 1].id
    }
}

impl Curve<'_> {
    /// Find the closest point on a curve defined by p0, cp0, cp1, p1
    /// It return the t value of the curve, the distance and the closest point
    pub fn t_closest(&self, coord: &Coord) -> (f32, f32, Coord) {
        math::curve::t_closest(
            coord,
            &self.p0.coord,
            &self.cp0.coord,
            &self.cp1.coord,
            &self.p1.coord,
        )
    }

    pub fn cubic_bezier(&self, t: f32) -> Coord {
        math::curve::cubic_bezier(
            t,
            &self.p0.coord,
            &self.cp0.coord,
            &self.cp1.coord,
            &self.p1.coord,
        )
    }

    pub fn intersection_with_y(&self, y: f32) -> Vec<f32> {
        math::curve2::intersection_with_y(
            &self.p0.coord,
            &self.cp0.coord,
            &self.cp1.coord,
            &self.p1.coord,
            y,
        )
    }

    pub fn add_smooth_result(&self, t: f32) -> (Coord, Coord, Coord, Coord, Coord) {
        math::curve::add_smooth_result(
            &self.p0.coord,
            &self.cp0.coord,
            &self.cp1.coord,
            &self.p1.coord,
            t,
        )
    }
}

pub fn tangent_cornor_pts(a: Curve, b: Curve) -> (Coord, Coord) {
    if a.p1.id != b.p0.id {
        panic!("Curves should be connected")
    }

    math::curve::tangent_cornor_pts(
        &a.p0.coord,
        &a.cp0.coord,
        &a.cp1.coord,
        &a.p1.coord,
        &b.cp0.coord,
        &b.cp1.coord,
        &b.p1.coord,
    )
}
