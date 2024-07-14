use common::types::Coord;

use crate::{CoordId, Curve, DbCoord, Shape};

use super::{coord::CoordType, curve};

impl Shape {
    pub fn curves_len(&self) -> usize {
        (self.path.len() - 1) / 3
    }

    pub fn curve_select_of_coord_id(&self, id: CoordId) -> Option<usize> {
        match self.coord_index_select(id) {
            Some((index, coord_type)) => match coord_type {
                CoordType::P1 => Some((index - 1) / 3),
                _ => None,
            },
            None => None,
        }
    }

    pub fn curve_select(&self, index: usize) -> Option<Curve> {
        if index < self.curves_len() {
            let p0 = &self.path[index * 3];
            let cp0 = &self.path[(index * 3 + 1) % self.path.len()];
            let cp1 = &self.path[(index * 3 + 2) % self.path.len()];
            let p1 = &self.path[(index * 3 + 3) % self.path.len()];
            return Some(Curve { p0, cp0, cp1, p1 });
        }
        None
    }

    pub fn toggle_separate_join_handle(&mut self, index: usize) {
        if self.is_handles_joined(index) {
            self.handle_separate(index);
        } else {
            self.handle_join(index);
        }
    }

    fn is_handles_joined(&self, index: usize) -> bool {
        let curve = self.curve_select(index).expect("Curve should exist");
        let curve_next = self
            .curve_select(index + 1 % self.curves_len())
            .expect("Curve should exist");
        curve.cp1.id == curve.p1.id && curve_next.cp0.id == curve.p1.id
    }

    pub fn handle_join(&mut self, curve_index_p1: usize) {
        let index_p1 = (curve_index_p1 * 3 + 3) % self.path.len();
        let len = self.path.len();
        self.path[(index_p1 - 1) % len] = self.path[index_p1].clone();
        self.path[(index_p1 + 1) % len] = self.path[index_p1].clone();
    }

    pub fn handle_separate(&mut self, curve_index_p1: usize) {
        let index_p1 = (curve_index_p1 * 3 + 3) % self.path.len();
        let (cp1_left, cp0_right) = {
            let curve_a = self
                .curve_select(curve_index_p1)
                .expect("Curve should exist");
            let curve_b = self
                .curve_select((curve_index_p1 + 1) % self.curves_len())
                .expect("Curve should exist");

            curve::tangent_cornor_pts(curve_a, curve_b)
        };

        let len = self.path.len();
        self.path[(index_p1 - 1) % len].coord = cp1_left;
        self.path[(index_p1 + 1) % len].coord = cp0_right;
    }

    /// Cut curve_index at t without chnaging the curve by replacing the handles
    pub fn curve_insert_smooth(&mut self, curve_index: usize, t: f32) {
        let curve = self.curve_select(curve_index).expect("Curve should exist");

        let (_, cp1l, p1, cp1r, _) = curve.add_smooth_result(t);

        let cp1l = cp1l.into();
        let p1 = p1.into();
        let cp1r = cp1r.into();

        let index_cp1 = (curve_index * 3 + 2) % self.path.len();
        //for a straight line no handle
        if !(curve.is_straight()) {
            let new_coords = vec![cp1l, p1, cp1r];
            self.path.splice(index_cp1..index_cp1, new_coords);
        }
        //left has separate handle
        else if !curve.left_handle_free() {
            let new_coords = vec![cp1l, p1.clone(), p1];
            self.path.splice(index_cp1..index_cp1, new_coords);
        }
        //right has separate handle
        else if !curve.right_handle_free() {
            let new_coords = vec![p1.clone(), p1, cp1r];
            self.path.splice(index_cp1..index_cp1, new_coords);
        }
    }

    pub fn curve_insert_line(&mut self, curve_index: usize, coord: Coord) {
        let p1: DbCoord = coord.into();
        let new_coords = vec![p1.clone(), p1.clone(), p1];
        let index_cp1 = (curve_index * 3 + 2) % self.path.len();
        self.path.splice(index_cp1..index_cp1, new_coords);
    }
}
