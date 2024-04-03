/*
Implementation of boolean operations on shapes.
For Shape A and B
Union : A OR B
Intersection : A AND B
Difference : A NOR B
*/

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use common::types::Coord;

use crate::{
    coord::CoordPtr,
    curve::{add_smooth_result, Curve},
    curve2::intersection,
    shape::Shape,
};


// When calculating the union of two shapes, we need to find all the intersection points between the two shapes.
// GreinerShape is a representation of a shape where all intersection points are added as separate coordinates and marked as such.
// It contains a doubly linked list of CoordOfIntersection.
struct GreinerShape {
    pub start: Rc<RefCell<CoordOfIntersection>>,
}

impl GreinerShape {
    pub fn new(start: Rc<RefCell<CoordOfIntersection>>) -> Self {
        Self { start }
    }

    #[allow(dead_code)] // For testing
    pub fn len(&self) -> usize {
        let mut current = self.start.clone();
        let mut count = 1;
        while current.borrow().next.is_some() {
            {
                let borrow_current = current.borrow();
                let next = borrow_current.next.as_ref().unwrap();
                if Rc::ptr_eq(&next, &self.start) {
                    break;
                }
            }

            let clone = current.borrow().next.clone().unwrap();
            current = clone;
            count += 1;
        }
        count
    }

    #[allow(dead_code)] // For testing
    pub fn get(&self, index: usize) -> Rc<RefCell<CoordOfIntersection>> {
        let mut current = self.start.clone();
        let mut count = 0;
        while count < index {
            let clone = current.borrow().next.clone().unwrap();
            current = clone;
            count += 1;
        }
        current
    }

    #[allow(dead_code)] // For testing
    pub fn print_coords_table(&self) {
        let mut current = self.start.clone();
        println!("Coord, Intersection, Entry");
        loop {
            println!(
                "{:?}, {}, {}",
                current.borrow().coord,
                current.borrow().intersect,
                current.borrow().entry
            );
            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next; //cp0

            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next; //cp1

            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next; //p1

            if Rc::ptr_eq(&current, &self.start) {
                break;
            }
        }
    }
}

impl Drop for GreinerShape {
    fn drop(&mut self) {
        self.start.borrow_mut().prev = None;
        self.start.borrow_mut().next = None;
    }
}

struct CoordOfIntersection {
    pub curve_index: usize,
    pub t: f32,
    pub neighbor: Option<Weak<RefCell<CoordOfIntersection>>>,
    pub next: Option<Rc<RefCell<CoordOfIntersection>>>,
    pub prev: Option<Weak<RefCell<CoordOfIntersection>>>,
    pub entry: bool,
    pub intersect: bool,
    pub coord: Coord,
    pub rel_coord: Option<CoordPtr>,
}

impl CoordOfIntersection {
    pub fn from_existing(rel_coord: &CoordPtr) -> Self {
        Self {
            curve_index: 0, //we don't need this
            t: 0.0,         //we don't need this
            neighbor: None,
            next: None,
            prev: None,
            entry: false,
            intersect: false,
            coord: rel_coord.borrow().clone(),
            rel_coord: Some(rel_coord.clone()),
        }
    }

    pub fn from_intersection(coord: Coord, t: f32, curve_index: usize) -> Self {
        Self {
            curve_index,
            t,
            neighbor: None,
            next: None,
            prev: None,
            entry: false,
            intersect: true,
            coord: coord,
            rel_coord: None,
        }
    }

    pub fn from_new(coord: Coord) -> Self {
        Self {
            curve_index: 0, //we don't need this
            t: 0.0,         //we don't need this
            neighbor: None,
            next: None,
            prev: None,
            entry: false,
            intersect: false,
            coord: coord,
            rel_coord: None,
        }
    }

    pub fn coord_ptr(&self) -> CoordPtr {
        match &self.rel_coord {
            Some(rel_coord) => rel_coord.clone(),
            None => Rc::new(RefCell::new(self.coord.clone())),
        }
    }
}

#[allow(dead_code)]
fn find_all_intersecion(a: &Shape, b: &Shape) -> (GreinerShape, GreinerShape) {
    let mut intersection_a: Vec<Rc<RefCell<CoordOfIntersection>>> =
        Vec::with_capacity(a.curves.len());
    let mut intersection_b: Vec<Rc<RefCell<CoordOfIntersection>>> =
        Vec::with_capacity(b.curves.len());

    for i in 0..a.curves.len() {
        let (a_p0, a_cp0, a_cp1, a_p1) = a.get_coords_of_curve(i);

        for j in 0..b.curves.len() {
            let (b_p0, b_cp0, b_cp1, b_p1) = b.get_coords_of_curve(j);

            let intersection_points = intersection(
                &a_p0.borrow(),
                &a_cp0.borrow(),
                &a_cp1.borrow(),
                &a_p1.borrow(),
                &b_p0.borrow(),
                &b_cp0.borrow(),
                &b_cp1.borrow(),
                &b_p1.borrow(),
            );

            for point in intersection_points {
                let point_a = Rc::new(RefCell::new(CoordOfIntersection::from_intersection(
                    point.coord,
                    point.t1,
                    i,
                )));

                let point_b = Rc::new(RefCell::new(CoordOfIntersection::from_intersection(
                    point.coord,
                    point.t2,
                    j,
                )));

                point_a.borrow_mut().neighbor = Some(Rc::downgrade(&point_b));
                point_b.borrow_mut().neighbor = Some(Rc::downgrade(&point_a));

                intersection_a.push(point_a);
                intersection_b.push(point_b);
            }
        }
    }
    let sort_fn = |a: &Rc<RefCell<CoordOfIntersection>>, b: &Rc<RefCell<CoordOfIntersection>>| {
        let ord_curve = a.borrow().curve_index.cmp(&b.borrow().curve_index);
        if ord_curve == std::cmp::Ordering::Equal {
            a.borrow()
                .t
                .partial_cmp(&b.borrow().t)
                .expect("Should be a number")
        } else {
            ord_curve
        }
    };

    intersection_a.sort_by(sort_fn);

    let start_a = create_all_shape(a, intersection_a);

    intersection_b.sort_by(sort_fn);

    let start_b = create_all_shape(b, intersection_b);
    (GreinerShape::new(start_a), GreinerShape::new(start_b))
}

fn create_all_shape(
    a: &Shape,
    intersection_a: Vec<Rc<RefCell<CoordOfIntersection>>>,
) -> Rc<RefCell<CoordOfIntersection>> {
    let start_a = Rc::new(RefCell::new(CoordOfIntersection::from_existing(&a.start)));
    let mut current = start_a.clone();
    let mut iter = intersection_a.iter();
    let mut current_intersection = iter.next();

    for curve_index in 0..a.curves.len() {
        let (a_p0, a_cp0, a_cp1, a_p1) = a.get_coords_of_curve(curve_index);
        let coord = *a_p0.borrow();
        let mut current_p0 = (coord, Some(a_p0));
        let coord = *a_cp0.borrow();
        let mut current_cp0 = (coord, Some(a_cp0));
        let coord = *a_cp1.borrow();
        let mut current_cp1 = (coord, Some(a_cp1));
        let coord = *a_p1.borrow();
        let current_p1 = (coord, a_p1);

        let mut last_t = 0.0;
        while current_intersection.is_some()
            && current_intersection.unwrap().borrow().curve_index == curve_index
        {
            let intersection = current_intersection.unwrap();

            let t = (intersection.borrow().t - last_t) / (1.0 - last_t);

            let (new_cp0, new_cp1l, new_p1, new_cp1r, new_cp2) = add_smooth_result(
                &current_p0.0,
                &current_cp0.0,
                &current_cp1.0,
                &current_p1.0,
                t,
            );

            last_t = intersection.borrow().t;

            let cp0 = Rc::new(RefCell::new(CoordOfIntersection::from_new(new_cp0)));
            current.borrow_mut().next = Some(cp0.clone());
            cp0.borrow_mut().prev = Some(Rc::downgrade(&current));
            current = cp0;

            let cp1l = Rc::new(RefCell::new(CoordOfIntersection::from_new(new_cp1l)));
            current.borrow_mut().next = Some(cp1l.clone());
            cp1l.borrow_mut().prev = Some(Rc::downgrade(&current));
            current = cp1l;

            current.borrow_mut().next = Some(intersection.clone());
            intersection.borrow_mut().prev = Some(Rc::downgrade(&current));
            current = intersection.clone();

            current_p0 = (new_p1, None);
            current_cp0 = (new_cp1r, None);
            current_cp1 = (new_cp2, None);

            current_intersection = iter.next();
        }

        match current_cp0.1 {
            Some(cp0) => {
                let cp0 = Rc::new(RefCell::new(CoordOfIntersection::from_existing(&cp0)));
                current.borrow_mut().next = Some(cp0.clone());
                cp0.borrow_mut().prev = Some(Rc::downgrade(&current));
                current = cp0;
            }
            None => {
                let cp0 = Rc::new(RefCell::new(CoordOfIntersection::from_new(current_cp0.0)));
                current.borrow_mut().next = Some(cp0.clone());
                cp0.borrow_mut().prev = Some(Rc::downgrade(&current));
                current = cp0;
            }
        }

        match current_cp1.1 {
            Some(cp1) => {
                let cp1 = Rc::new(RefCell::new(CoordOfIntersection::from_existing(&cp1)));
                current.borrow_mut().next = Some(cp1.clone());
                cp1.borrow_mut().prev = Some(Rc::downgrade(&current));
                current = cp1;
            }
            None => {
                let cp1 = Rc::new(RefCell::new(CoordOfIntersection::from_new(current_cp1.0)));
                current.borrow_mut().next = Some(cp1.clone());
                cp1.borrow_mut().prev = Some(Rc::downgrade(&current));
                current = cp1;
            }
        }

        let p1 = Rc::new(RefCell::new(CoordOfIntersection::from_existing(
            &current_p1.1,
        )));
        current.borrow_mut().next = Some(p1.clone());
        p1.borrow_mut().prev = Some(Rc::downgrade(&current));
        current = p1;
    }

    let last_cp = current.borrow().prev.as_ref().unwrap().upgrade().unwrap();
    last_cp.borrow_mut().next = Some(start_a.clone());
    start_a.borrow_mut().prev = Some(Rc::downgrade(&last_cp));

    start_a
}

fn mark_entry_exit_points(ag: &mut GreinerShape, a: &Shape, bg: &mut GreinerShape, b: &Shape) {
    let mut status_entry = true;
    let con = b.contains(&ag.start.borrow().coord);
    if con {
        status_entry = false;
    }

    let mut current = ag.start.clone();
    while current.borrow().next.is_some()
        && !Rc::ptr_eq(current.borrow().next.as_ref().unwrap(), &ag.start)
    {
        let next = {
            let borrow_current = current.borrow();
            borrow_current.next.as_ref().unwrap().clone()
        };
        if next.borrow().intersect {
            next.borrow_mut().entry = status_entry;
            status_entry = !status_entry;
        }
        current = next.clone();
    }

    status_entry = true;
    if a.contains(&bg.start.borrow().coord) {
        status_entry = false;
    }

    current = bg.start.clone();
    while current.borrow().next.is_some()
        && !Rc::ptr_eq(current.borrow().next.as_ref().unwrap(), &bg.start)
    {
        let next = {
            let borrow_current = current.borrow();
            borrow_current.next.as_ref().unwrap().clone()
        };
        if next.borrow().intersect {
            next.borrow_mut().entry = status_entry;
            status_entry = !status_entry;
        }
        current = next.clone();
    }
}

fn merge(ag: &GreinerShape, _bg: &GreinerShape, a: &Shape, _b: &Shape) -> Shape {
    let first_intersection = {
        let mut current = ag.start.clone();
        while !current.borrow().intersect {
            let next = current.borrow().next.as_ref().unwrap().clone();
            current = next;
        }
        current
    };

    let mut merged = Shape {
        start: first_intersection.borrow().coord_ptr(),
        curves: Vec::new(),
        color: a.color.clone(),
    };

    let mut current = first_intersection.clone();
    loop {
        if !current.borrow().entry { //Remove ! to make the algo A AND B
            loop {
                let next = current.borrow().next.as_ref().unwrap().clone();
                current = next;
                let cp0 = current.borrow().coord_ptr();
        
                let next = current.borrow().next.as_ref().unwrap().clone();
                current = next;
                let cp1 = current.borrow().coord_ptr();
        
                let next = current.borrow().next.as_ref().unwrap().clone();
                current = next;
                let p1 = current.borrow().coord_ptr();
        
                merged.curves.push(Curve::new(cp0, cp1, p1));
                
                if current.borrow().intersect {
                    break;
                }
            }
        } else{
            loop {
                let next = current.borrow().prev.as_ref().unwrap().upgrade().unwrap().clone();
                current = next;
                let cp0 = current.borrow().coord_ptr();
        
                let next = current.borrow().prev.as_ref().unwrap().upgrade().unwrap().clone();
                current = next;
                let cp1 = current.borrow().coord_ptr();
        
                let next = current.borrow().prev.as_ref().unwrap().upgrade().unwrap().clone();
                current = next;
                let p1 = current.borrow().coord_ptr();
        
                merged.curves.push(Curve::new(cp0, cp1, p1));
                
                if current.borrow().intersect {
                    break;
                }
            }
        }
        let next = current
            .borrow()
            .neighbor
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .clone();
        current = next;

        if Rc::ptr_eq(&current, &first_intersection)
            || Rc::ptr_eq(
                &current,
                &first_intersection
                    .borrow()
                    .neighbor
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap(),
            )
        {
            break;
        }
    }

    merged
}

#[allow(dead_code)] 
pub fn shape_union(a: &Shape, b: &Shape) -> Shape {
    let (mut ag, mut bg) = find_all_intersecion(a, b);

    mark_entry_exit_points(&mut ag, a, &mut bg, b);

    merge(&ag, &bg, a, b)
}

#[cfg(test)]
mod test {
    use common::{types::Coord, Rgba};

    use crate::{
        create_circle,
        shape_boolean::{find_all_intersecion, mark_entry_exit_points, shape_union},
        Vgc,
    };

    #[test]
    fn given_two_circle_when_union_then_valid() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.2, 0.0), 0.2, 0.2);

        let s1 = vgc.get_shape(0).expect("Shape should exist");
        let s2 = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_union(&s1, &s2);

        assert_eq!(merged.curves.len(), 8);
    }

    #[test]
    fn given_two_oval_with_no_valid_p_when_union_then_valid() {
        let mut shape1 = vec![
            Coord::new(0.0, 0.3),
            Coord::new(-0.8, 0.3),
            Coord::new(-0.8, -0.3),
            Coord::new(0.0, -0.3),
            Coord::new(0.8, -0.3),
            Coord::new(0.8, 0.3),
            Coord::new(0.0, 0.3),
        ];
        shape1.reverse();

        let shape2 = vec![
            Coord::new(0.3, 0.0),
            Coord::new(0.3, 0.8),
            Coord::new(-0.3, 0.8),
            Coord::new(-0.3, 0.0),
            Coord::new(-0.3, -0.8),
            Coord::new(0.3, -0.8),
            Coord::new(0.3, 0.0),
        ];

        let vgc = crate::generate_from_push(vec![shape1, shape2]);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let (mut ag, mut bg) = find_all_intersecion(a, b);

        mark_entry_exit_points(&mut ag, a, &mut bg, b);

        assert_eq!(ag.len(), 18);
        assert_eq!(bg.len(), 18);

        let merged = shape_union(&a, &b);

        assert_eq!(merged.curves.len(), 4);
    }
}
