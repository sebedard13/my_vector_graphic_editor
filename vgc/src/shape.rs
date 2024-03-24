use crate::coord::{CoordPtr, CoordType};
use crate::curve::{self, add_smooth_result};
use crate::curve::Curve;
use crate::curve2::intersection;
use common::types::Coord;
use common::Rgba;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Shape {
    pub start: Rc<RefCell<Coord>>,
    pub(crate) curves: Vec<Curve>,
    pub color: Rgba,
}

impl Shape {
    pub fn toggle_separate_join_handle(&mut self, index: usize) {
        if self.is_handles_joined(index) {
            self.separate_handle(index);
        } else {
            self.join_handle(index);
        }
    }

    fn is_handles_joined(&self, index: usize) -> bool {
        let curve = &self.curves[index];
        curve.cp0 == curve.p1 || curve.cp1 == curve.p1
    }

    pub fn is_closed(&self) -> bool {
        if self.curves.is_empty() {
            return false;
        }
        let last_curve = self
            .curves
            .last()
            .expect("Shape should have at least one curve ");
        last_curve.p1 == self.start
    }

    pub fn close(&mut self) {
        if !self.is_closed() {
            let (_, _, _, p1) = self.get_coords_of_curve(self.curves.len() - 1);

            self.curves.push(Curve {
                cp0: p1,
                cp1: self.start.clone(),
                p1: self.start.clone(),
            });
        }
    }

    pub fn separate_handle(&mut self, curve_index_p1: usize) {
        let (coord_index0, coord_index1) = {
            let p0 = {
                if curve_index_p1 == 0 {
                    self.start.borrow()
                } else {
                    self.curves[curve_index_p1 - 1].p1.borrow()
                }
            };
            let current_curve = &self.curves[curve_index_p1];
            let cp0 = &current_curve.cp0.borrow();
            let cp1 = &current_curve.cp1.borrow();
            let p1 = &current_curve.p1.borrow();

            let next_curve = &self.curves[(curve_index_p1 + 1) % self.curves.len()];
            let cp2 = &next_curve.cp0.borrow();
            let cp3 = &next_curve.cp1.borrow();
            let p2 = &next_curve.p1.borrow();

            curve::tangent_cornor_pts(&p0, &cp0, &cp1, &p1, &cp2, &cp3, &p2)
        };

        self.curves[curve_index_p1].cp1 = Rc::new(RefCell::new(coord_index0));
        let len = self.curves.len();
        self.curves[(curve_index_p1 + 1) % len].cp0 = Rc::new(RefCell::new(coord_index1));
    }

    pub fn join_handle(&mut self, curve_index_p1: usize) {
        //cp0
        let coord_index = &self.curves[curve_index_p1].p1;
        let curve_after = (curve_index_p1 + 1) % self.curves.len();

        self.curves[curve_after].cp0 = coord_index.clone();

        //cp1
        let coord_index = &self.curves[curve_index_p1].p1;

        self.curves[curve_index_p1].cp1 = coord_index.clone();
    }

    pub fn to_path(&self) -> String {
        let mut path = String::new();
        let start = self.start.borrow();
        path.push_str(&format!("M {} {}", start.x(), start.y()));
        for curve in &self.curves {
            path.push(' ');
            path.push_str(&curve.to_path());
        }
        if self.is_closed() {
            path.push_str(" Z");
        }
        path
    }

    /// Visit each curve of the shape and call the visitor function with the curve index and 4 coords of the curve so p0, cp0, cp1, p1
    pub fn visit_full_curves(
        &self,
        mut visitor: impl FnMut(usize, &Coord, &Coord, &Coord, &Coord),
    ) {
        let start = self.start.borrow();
        let mut prev_coord = start;
        for (index, curve) in self.curves.iter().enumerate() {
            let cp0 = curve.cp0.borrow();
            let cp1 = curve.cp1.borrow();
            let p1 = curve.p1.borrow();

            visitor(index, &prev_coord, &cp0, &cp1, &p1);

            prev_coord = p1;
        }
    }

    /// Visit each curve and calculate the closest point on the curve to the coord
    ///
    /// Return (curve index, t value , distance, closest point)
    pub fn closest_curve(&self, coord: &Coord) -> (usize, f32, f32, Coord) {
        let mut min_distance = std::f32::MAX;
        let mut min_index = 0;
        let mut min_t = 0.0;
        let mut min_coord = self.start.borrow().clone();

        self.visit_full_curves(|curve_index, p0, cp0, cp1, p1| {
            let (t_min, distance, coord_closest) = curve::t_closest(coord, p0, cp0, cp1, p1);

            if distance < min_distance {
                min_distance = distance;
                min_index = curve_index;
                min_t = t_min;
                min_coord = coord_closest;
            }
        });
        (min_index, min_t, min_distance, min_coord)
    }

    pub fn get_coords_of_curve(
        &self,
        curve_index: usize,
    ) -> (CoordPtr, CoordPtr, CoordPtr, CoordPtr) {
        let mut prev_coord = self.start.clone();

        if curve_index > 0 {
            let prev_curve = self
                .curves
                .get(curve_index - 1)
                .expect("Index should be valid");
            prev_coord = prev_curve.p1.clone();
        }
        let curve = self.curves.get(curve_index).expect("Index should be valid");
        let cp0 = curve.cp0.clone();
        let cp1 = curve.cp1.clone();
        let p1 = curve.p1.clone();

        (prev_coord, cp0, cp1, p1)
    }

    pub fn push_coord(
        &mut self,
        cp0: Rc<RefCell<Coord>>,
        cp1: Rc<RefCell<Coord>>,
        p1: Rc<RefCell<Coord>>,
    ) {
        self.curves.push(Curve::new(cp0, cp1, p1));
    }

    pub fn get_coords_of_shape_tmp(&self) -> Vec<Rc<RefCell<Coord>>> {
        let mut vec = Vec::new();
        vec.push(self.start.clone());
        for curve in self.curves.iter() {
            vec.push(curve.cp0.clone());
            vec.push(curve.cp1.clone());
            vec.push(curve.p1.clone());
        }
        vec
    }

    pub fn move_coord(&mut self, coord_type: &CoordType, x: f32, y: f32) {
        match coord_type {
            CoordType::Start => {
                let mut coord = self.start.borrow_mut();
                coord.set_x(x);
                coord.set_y(y);
            }
            CoordType::Cp0(index_curve) => {
                let mut coord = self.curves[*index_curve].cp0.borrow_mut();
                coord.set_x(x);
                coord.set_y(y);
            }
            CoordType::Cp1(index_curve) => {
                let mut coord = self.curves[*index_curve].cp1.borrow_mut();
                coord.set_x(x);
                coord.set_y(y);
            }
            CoordType::P1(index_curve) => {
                let mut coord = self.curves[*index_curve].p1.borrow_mut();
                coord.set_x(x);
                coord.set_y(y);
            }
        }
    }

    /// Cut curve_index at t without chnaging the curve by replacing the handles
    pub fn insert_coord_smooth(&mut self, curve_index: usize, t: f32) {
        let (p0, cp0i, cp2i, p2) = self.get_coords_of_curve(curve_index);

        let (cp0, cp1l, p1, cp1r, cp2) = curve::add_smooth_result(
            &p0.borrow(),
            &cp0i.borrow(),
            &cp2i.borrow(),
            &p2.borrow(),
            t,
        );

        self.insert_coord_at(curve_index, p1);

        //for a straight line no handle
        if !(Rc::ptr_eq(&p0, &cp0i) && Rc::ptr_eq(&cp2i, &p2)) {
            self.curves[curve_index].cp1 = Rc::new(RefCell::new(cp1l));
            self.curves[curve_index + 1].cp0 = Rc::new(RefCell::new(cp1r));
        }

        //left has separate handle
        if !Rc::ptr_eq(&p0, &cp0i) {
            self.curves[curve_index].cp0 = Rc::new(RefCell::new(cp0));
        }

        //right has separate handle
        if !Rc::ptr_eq(&cp2i, &p2) {
            //Index valid because we just inserted
            self.curves[curve_index + 1].cp1 = Rc::new(RefCell::new(cp2));
        }
    }

    /// Cut curve_index with coord like cp0 coord coord coord cp1 p1
    pub fn insert_coord_at(&mut self, curve_index: usize, coord: Coord) {
        let p1 = Rc::new(RefCell::new(coord));

        let cp0 = self.curves[curve_index].cp0.clone();

        let new_curve = Curve::new(cp0, p1.clone(), p1.clone());

        self.curves[curve_index].cp0 = p1;

        self.curves.insert(curve_index, new_curve);
    }

    pub fn remove_start(&mut self) {}

    pub fn remove_curve(&mut self, curve_index: usize) {
        if self.is_closed() && self.curves.len() - 1 == curve_index {
            let curve = self.curves.remove(0);
            self.curves[curve_index - 1].cp1 = curve.cp1;
            self.curves[curve_index - 1].p1 = curve.p1.clone();
            self.start = curve.p1;
            return;
        }

        let cp0 = self.curves[curve_index].cp0.clone();

        self.curves.remove(curve_index);

        if let Some(curve) = self.curves.get_mut(curve_index) {
            curve.cp0 = cp0;
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.is_closed() {
            self.curves.len() == 1
        } else {
            self.curves.is_empty()
        }
    }

    pub fn merge(&self, other: &Shape) -> Option<Shape> {
        let mut merged = Shape {
            start: self.start.clone(), // We assume that the start is not in other
            curves: Vec::new(),
            color: self.color.clone(),
        };

        let mut closed = false;
        let mut i_main = 0;
        let mut is_a_main = true;
        while !closed {
            let (m_p0, m_cp0, m_cp1, m_p1) = if is_a_main {
                i_main = i_main % self.curves.len();
                self.get_coords_of_curve(i_main)
            } else {
                i_main = i_main % other.curves.len();
                other.get_coords_of_curve(i_main)
            };

            let max_len_other = if is_a_main {
                self.curves.len()
            } else {
                other.curves.len()
            };

            let mut has_done = false;
            for i_b in 0..max_len_other {
                let (b_p0, b_cp0, b_cp1, b_p1) = if is_a_main {
                    other.get_coords_of_curve(i_b)
                } else {
                    self.get_coords_of_curve(i_b)
                };

                let intersection_points = intersection(
                    &m_p0.borrow(),
                    &m_cp0.borrow(),
                    &m_cp1.borrow(),
                    &m_p1.borrow(),
                    &b_p0.borrow(),
                    &b_cp0.borrow(),
                    &b_cp1.borrow(),
                    &b_p1.borrow(),
                );

                if !intersection_points.is_empty() {
                    let point = intersection_points[0];

                    let (new_cp0, new_cp1, new_p1, _, _) = add_smooth_result(
                        &m_p0.borrow(),
                        &m_cp0.borrow(),
                        &m_cp1.borrow(),
                        &m_p1.borrow(),
                        point.t1,
                    );

                    println!("new_p1: {:?}", new_p1);
                    merged.curves.push(Curve::new(
                        Rc::new(RefCell::new(new_cp0)),
                        Rc::new(RefCell::new(new_cp1)),
                        Rc::new(RefCell::new(point.coord)),
                    ));

                    let (_, _, _, new_cp0, new_cp1) = add_smooth_result(
                        &b_p0.borrow(),
                        &b_cp0.borrow(),
                        &b_cp1.borrow(),
                        &b_p1.borrow(),
                        point.t2,
                    );

                    merged.curves.push(Curve::new(
                        Rc::new(RefCell::new(new_cp0)),
                        Rc::new(RefCell::new(new_cp1)),
                        b_p1.clone(),
                    ));
                    is_a_main = !is_a_main;
                    i_main = i_b + 1;
                    has_done = true;
                    break;
                }
            }

            if has_done {
                continue;
            }

            merged.curves.push(Curve::new(m_cp0, m_cp1, m_p1));
            i_main += 1;

            if *merged.start.borrow() == *merged.curves.last().unwrap().p1.borrow() {
                closed = true;
            }
        }

        Some(merged)
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{create_circle, generate_from_push, Vgc};
    use common::{types::Coord, Rgba};

    #[test]
    fn cloest_pt() {
        let vgc = generate_from_push(vec![vec![
            Coord::new(0.43, 0.27),
            Coord::new(0.06577811, 0.2938202),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(0.7942219, 0.24617982),
            Coord::new(0.43, 0.27),
        ]]);

        let shape = vgc.get_shape(0).expect("Shape should exist");

        let (_, _, _, coord) = shape.closest_curve(&Coord::new(1.008, 0.612));
        assert_ne!(&coord.y(), &1.0);
    }

    #[test]
    fn insert_coord_at() {
        let mut vgc = generate_from_push(vec![vec![
            Coord::new(0.0, 0.0),
            Coord::new(0.1, 0.1),
            Coord::new(0.9, 0.9),
            Coord::new(1.0, 1.0),
        ]]);

        let shape = vgc.get_shape_mut(0).expect("Shape should exist");

        shape.insert_coord_at(0, Coord::new(0.5, 0.5));

        let (p0, c0, c1l, p1) = shape.get_coords_of_curve(0);
        let (p1_2, c1r, c2, p2) = shape.get_coords_of_curve(1);

        assert_eq!(*p0.borrow(), Coord::new(0.0, 0.0));

        assert_eq!(*c0.borrow(), Coord::new(0.1, 0.1));
        assert_eq!(*c1l.borrow(), Coord::new(0.5, 0.5));
        assert_eq!(*p1.borrow(), Coord::new(0.5, 0.5));

        assert_eq!(*p1_2.borrow(), Coord::new(0.5, 0.5));

        assert_eq!(*c1r.borrow(), Coord::new(0.5, 0.5));

        assert_eq!(*c2.borrow(), Coord::new(0.9, 0.9));
        assert_eq!(*p2.borrow(), Coord::new(1.0, 1.0));

        assert_eq!(shape.curves.len(), 3); //Because close

        //+1 or +2 beacause of current borrow at 280 and 281
        assert_eq!(Rc::strong_count(&p0), 3 + 1); //Because close
        assert_eq!(Rc::strong_count(&c0), 1 + 1);
        assert_eq!(Rc::strong_count(&c1l), 3 + 4);
        assert_eq!(Rc::strong_count(&p1), 3 + 4);

        assert_eq!(Rc::strong_count(&p1_2), 3 + 4);
        assert_eq!(Rc::strong_count(&c1r), 3 + 4);

        assert_eq!(Rc::strong_count(&c2), 1 + 1);
        assert_eq!(Rc::strong_count(&p2), 2 + 1);
    }

    #[test]
    fn when_merge_two_circle() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.2, 0.0), 0.2, 0.2);

        let s1 = vgc.get_shape(0).expect("Shape should exist");
        let s2 = vgc.get_shape(1).expect("Shape should exist");

        let merged = s1.merge(&s2).expect("Should merge");

        assert_eq!(*(merged.curves[1].p1.borrow()), Coord::new(0.2, 0.20001104));
        assert_eq!(merged.curves.len(), 8);          
        assert_eq!(merged.to_path(),"M 0 0.20001104 C 0.03648475 0.19992407 0.07062003 0.19018893 0.09999999 0.17321143 C 0.12937993 0.19018891 0.16351523 0.19992408 0.2 0.20001104 C 0.3106854 0.19974719 0.3997472 0.110685386 0.40001106 0 C 0.3997472 -0.110685386 0.3106854 -0.19974719 0.2 -0.20001104 C 0.16351524 -0.19992407 0.12937997 -0.19018893 0.10000002 -0.17321143 C 0.07062003 -0.19018894 0.03648475 -0.19992407 0 -0.20001104 C -0.110685386 -0.19974719 -0.19974719 -0.110685386 -0.20001104 0 C -0.19974719 0.110685386 -0.110685386 0.19974719 0 0.20001104 Z");
    }
}
