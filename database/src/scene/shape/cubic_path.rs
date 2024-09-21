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
                CoordType::P0 => Some(self.curves_len() - 1),
                _ => None,
            },
            None => None,
        }
    }

    pub fn curves(&self) -> impl Iterator<Item = Curve> + '_ {
        (0..self.curves_len()).map(|index| self.curve_select(index).expect("Curve should exist"))
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
            .curve_select((index + 1) % self.curves_len())
            .expect("Curve should exist");
        curve.cp1.id == curve.p1.id && curve_next.cp0.id == curve.p1.id
    }

    pub fn handle_join(&mut self, curve_index_p1: usize) {
        let index_p1 = (curve_index_p1 * 3 + 3) % self.path.len();
        let len = self.path.len();
        self.path[(index_p1 - 1) % len] = self.path[index_p1];
        self.path[(index_p1 + 1) % len] = self.path[index_p1];
    }

    pub fn handle_separate(&mut self, curve_index_p1: usize) {
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
        let curve_len = self.curves_len();
        let cpl = &mut self.path[(curve_index_p1 * 3 + 2) % len];
        cpl.coord = cp1_left;
        cpl.id.update();

        let cpr = &mut self.path[(((curve_index_p1 + 1) % curve_len) * 3 + 1) % len];
        cpr.coord = cp0_right;
        cpr.id.update();
    }

    /// Cut curve_index at t without chnaging the curve by replacing the handles
    pub fn curve_insert_smooth(&mut self, curve_index: usize, t: f32) -> (CoordId, CoordId, CoordId) {
        let curve = self.curve_select(curve_index).expect("Curve should exist");

        let (cp0, cp1l, p1, cp1r, cp2) = curve.add_smooth_result(t);

        let cp0 = cp0;
        let cp1l = cp1l.into();
        let p1: DbCoord = p1.into();
        let cp1r = cp1r.into();
        let cp2 = cp2;

        let index_cp1 = (curve_index * 3 + 2) % self.path.len();

        let is_straight = curve.is_straight();
        let left_handle_free = curve.left_handle_free();
        let right_handle_free = curve.right_handle_free();

        let new_coords = vec![p1, p1, p1];
        self.path.splice(index_cp1..index_cp1, new_coords);

        //for a straight line no handle
        if !(is_straight) {
            self.path[index_cp1] = cp1l;
            self.path[index_cp1 + 2] = cp1r;
        }
        //left has separate handle
        if left_handle_free {
            self.path[index_cp1 - 1].coord = cp0;
        }
        //right has separate handle
        if right_handle_free {
            self.path[index_cp1 + 3].coord = cp2;
        }

        (cp1l.id, p1.id, cp1r.id)
    }

    pub fn curve_insert_line(&mut self, curve_index: usize, coord: Coord) {
        let p1: DbCoord = coord.into();
        let new_coords = vec![p1, p1, p1];
        let index_cp1 = (curve_index * 3 + 2) % self.path.len();
        self.path.splice(index_cp1..index_cp1, new_coords);
    }
}

#[cfg(test)]
mod tests {
    use common::pures::Affine;

    use super::*;

    #[test]
    fn given_closed_shape_when_toggle_handle_0_then_handles_separated() {
        let mut shape = Shape::new_from_lines(
            vec![
                DbCoord::new(0.0, 0.0),
                DbCoord::new(0.0, 1.0),
                DbCoord::new(1.0, 1.0),
            ],
            Affine::identity(),
        );
        let curve = shape
            .curve_select_of_coord_id(shape.path[0].id)
            .expect("Not 404");

        shape.toggle_separate_join_handle(curve);

        assert_ne!(shape.path[1].coord, Coord::new(0.0, 0.0));
        assert_ne!(shape.path[8].coord, Coord::new(0.0, 0.0));
    }

    #[test]
    fn given_closed_shape_when_insert_smooth_0_then_no_cp_free() {
        let mut shape = Shape::new_from_lines(
            vec![
                DbCoord::new(0.0, 0.0),
                DbCoord::new(0.0, 1.0),
                DbCoord::new(1.0, 1.0),
            ],
            Affine::identity(),
        );

        shape.curve_insert_smooth(0, 0.5);

        let mut ids = Vec::new();
        for coord in &shape.path {
            if !ids.contains(&coord.id) {
                ids.push(coord.id);
            }
        }

        assert_eq!(
            ids.len(),
            4,
            "There should be 4 coord because everything is a line"
        );
    }
}
