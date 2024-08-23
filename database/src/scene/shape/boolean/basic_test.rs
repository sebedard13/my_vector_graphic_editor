use crate::{DbCoord, Shape};

use super::{ShapeDifference, ShapeIntersection, ShapeUnion};

pub fn verify_union(res: ShapeUnion, a: Shape, b: Shape) {
    let merged = {
        match res {
            ShapeUnion::New(merged) => vec![merged],
            ShapeUnion::A => vec![a.clone()],
            ShapeUnion::B => vec![b.clone()],
            ShapeUnion::None => vec![],
        }
    };

    let steps = 15;
    for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
        for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            let coord = &DbCoord::new(x + 0.001, y - 0.002).coord;

            let merged_contains = merged.iter().any(|shape| shape.contains(coord));

            assert_eq!(
                merged_contains,
                a.contains(coord) || b.contains(coord),
                "Contains failed at ({}, {})",
                x + 0.001,
                y - 0.002
            );
        }
    }
}

pub fn verify_intersection(res: ShapeIntersection, a: Shape, b: Shape) {
    let merged = {
        match res {
            ShapeIntersection::New(merged) => merged,
            ShapeIntersection::A => vec![a.clone()],
            ShapeIntersection::B => vec![b.clone()],
            ShapeIntersection::None => vec![],
        }
    };

    let steps = 15;
    for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
        for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            let coord = &DbCoord::new(x + 0.001, y - 0.002).coord;

            let merged_contains = merged.iter().any(|shape| shape.contains(coord));

            assert_eq!(
                merged_contains,
                a.contains(coord) && b.contains(coord),
                "Contains failed at ({}, {})",
                x + 0.001,
                y - 0.002
            );
        }
    }
}

pub fn verify_difference(res: ShapeDifference, a: Shape, b: Shape) {
    let merged = {
        match res {
            ShapeDifference::A => vec![a.clone()],
            ShapeDifference::EraseA => vec![],
            ShapeDifference::New(merged) => merged,
            ShapeDifference::AWithBHole => unimplemented!(),
        }
    };

    let steps = 15;
    for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
        for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            let coord = &DbCoord::new(x + 0.001, y - 0.002).coord;

            let merged_contains = merged.iter().any(|shape| shape.contains(coord));

            assert_eq!(
                merged_contains,
                a.contains(coord) && !b.contains(coord),
                "Contains failed at ({}, {})",
                x + 0.001,
                y - 0.002
            );
        }
    }
}

pub fn print_svg_scale(a: &Shape, b: &Shape, scale: f32) {
    /*<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
        <path d="M -0.7 -0.7 C -0.7 -0.7 0.7 -0.7 0.7 -0.7 C 0.7 -0.7 0.7 0.5 0.7 0.5 C 0.7 0.5 -0.7 0.5 -0.7 0.5 C -0.7 0.5 -0.7 -0.7 -0.7 -0.7 Z" fill="red" transform="scale(100)"/>
        <path d="M -0.7 -0.5 C -0.7 -0.5 0.7 -0.5 0.7 -0.5 C 0.7 -0.5 0.7 0.7 0.7 0.7 C 0.7 0.7 -0.7 0.7 -0.7 0.7 C -0.7 0.7 -0.7 -0.5 -0.7 -0.5 Z" fill="black" transform="scale(100)"/>
    </svg>*/

    let a_path = a.path();
    let b_path = b.path();

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
    <path d="{}" fill="red" transform="scale({})"/>
    <path d="{}" fill="black" transform="scale({})"/>
</svg>"#,
        a_path, scale, b_path, scale
    );
    println!("{}", svg);
}

fn print_svg(a: &Shape, b: &Shape) {
    print_svg_scale(a, b, 100.0);
}

mod overlapping_circles {
    use common::types::{Coord, Length2d};

    use crate::{
        scene::shape::boolean::{
            difference::shape_difference, intersection::shape_intersection, union::shape_union,
            ShapeDifference, ShapeIntersection, ShapeUnion,
        },
        Shape,
    };

    use super::{verify_difference, verify_intersection, verify_union};

    fn create() -> (Shape, Shape) {
        //M 0 0.5000276 C 0.27671343 0.49936792 0.49936792 0.27671343 0.5000276 0 C 0.49936792 -0.27671343 0.27671343 -0.49936792 0 -0.5000276 C -0.27671343 -0.49936792 -0.49936792 -0.27671343 -0.5000276 0 C -0.49936792 0.27671343 -0.27671343 0.49936792 0 0.5000276 Z
        //M 0.3 0.60003316 C 0.6320561 0.59924155 0.89924157 0.33205613 0.9000332 0 C 0.89924157 -0.33205613 0.6320561 -0.59924155 0.3 -0.60003316 C -0.032056123 -0.59924155 -0.29924154 -0.33205613 -0.30003315 0 C -0.29924154 0.33205613 -0.032056123 0.59924155 0.3 0.60003316 Z
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.5, 0.5));
        let b = Shape::new_circle(Coord::new(0.3, 0.0), Length2d::new(0.6, 0.6));
        (a, b)
    }

    #[test]
    fn union() {
        let (a, b) = create();

        let res = shape_union(&a, &b);
        match &res {
            ShapeUnion::New(_) => {}
            _ => panic!("Unexpected result"),
        }

        verify_union(res, a, b);
    }

    #[test]
    fn intersection() {
        let (a, b) = create();

        let res = shape_intersection(&a, &b);

        match &res {
            ShapeIntersection::New(merged) => {
                assert_eq!(merged.len(), 1);
            }
            _ => panic!("Unexpected result"),
        }

        verify_intersection(res, a, b);
    }

    #[test]
    fn difference() {
        let (a, b) = create();

        let res = shape_difference(&a, &b);
        match &res {
            ShapeDifference::New(merged) => {
                assert_eq!(merged.len(), 1);
            }
            _ => panic!("Unexpected result"),
        }

        verify_difference(res, a, b);
    }
}

mod disjoint_circles {
    use common::types::{Coord, Length2d};

    use crate::{
        scene::shape::boolean::{
            difference::shape_difference, intersection::shape_intersection, union::shape_union,
            ShapeDifference, ShapeIntersection, ShapeUnion,
        },
        Shape,
    };

    fn create() -> (Shape, Shape) {
        //M 0 0.30001658 C 0.16602807 0.29962078 0.29962078 0.16602807 0.30001658 0 C 0.29962078 -0.16602807 0.16602807 -0.29962078 0 -0.30001658 C -0.16602807 -0.29962078 -0.29962078 -0.16602807 -0.30001658 0 C -0.29962078 0.16602807 -0.16602807 0.29962078 0 0.30001658 Z
        //M 0.7 0.20001104 C 0.81068534 0.19974717 0.89974713 0.11068537 0.90001106 0 C 0.89974713 -0.11068537 0.81068534 -0.19974717 0.7 -0.20001104 C 0.58931464 -0.19974717 0.50025284 -0.11068537 0.49998894 0 C 0.50025284 0.11068537 0.58931464 0.19974717 0.7 0.20001104 Z
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.3, 0.3));
        let b = Shape::new_circle(Coord::new(0.7, 0.0), Length2d::new(0.2, 0.2));
        (a, b)
    }

    #[test]
    fn union() {
        let (a, b) = create();

        let res = shape_union(&a, &b);

        match &res {
            ShapeUnion::None => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn intersection() {
        let (a, b) = create();

        let res = shape_intersection(&a, &b);

        match &res {
            ShapeIntersection::None => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn difference() {
        let (a, b) = create();

        let res = shape_difference(&a, &b);

        match &res {
            ShapeDifference::A => {}
            _ => panic!("Unexpected result"),
        }
    }
}

mod ovelapping_ovals {

    use common::pures::Affine;

    use crate::{
        scene::shape::boolean::{
            difference::shape_difference, intersection::shape_intersection, union::shape_union,
            ShapeDifference, ShapeIntersection, ShapeUnion,
        },
        DbCoord, Shape,
    };

    use super::{verify_difference, verify_intersection, verify_union};

    fn create() -> (Shape, Shape) {
        let mut shape1 = vec![
            //horizontal oval
            DbCoord::new(0.0, 0.3),
            DbCoord::new(-0.8, 0.3),
            DbCoord::new(-0.8, -0.3),
            DbCoord::new(0.0, -0.3),
            DbCoord::new(0.8, -0.3),
            DbCoord::new(0.8, 0.3),
            DbCoord::new(0.0, 0.3),
        ];
        shape1.reverse();

        let shape2 = vec![
            //vertical oval
            DbCoord::new(0.3, 0.0),
            DbCoord::new(0.3, 0.8),
            DbCoord::new(-0.3, 0.8),
            DbCoord::new(-0.3, 0.0),
            DbCoord::new(-0.3, -0.8),
            DbCoord::new(0.3, -0.8),
            DbCoord::new(0.3, 0.0),
        ];

        let a = Shape::new_from_path(shape1, Affine::identity());
        let b = Shape::new_from_path(shape2, Affine::identity());
        (a, b)
    }

    #[test]
    fn union() {
        let (a, b) = create();
        println!("A: {:?}", a.path());
        println!("B: {:?}", b.path());

        let res = shape_union(&a, &b);

        match &res {
            ShapeUnion::New(merged) => {
                assert_eq!(merged.curves_len(), 4);
            }
            _ => panic!("Unexpected result"),
        }

        verify_union(res, a, b);
    }

    #[test]
    fn intersection() {
        let (a, b) = create();

        let res = shape_intersection(&a, &b);

        match &res {
            ShapeIntersection::New(merged) => {
                assert_eq!(merged.len(), 1);
                assert_eq!(merged[0].curves_len(), 8);
            }
            _ => panic!("Unexpected result"),
        }

        verify_intersection(res, a, b);
    }

    #[test]
    fn difference() {
        let (a, b) = create();

        let res = shape_difference(&a, &b);

        match &res {
            ShapeDifference::New(merged) => {
                assert_eq!(merged.len(), 2);
                assert_eq!(merged[0].curves_len(), 3);
                assert_eq!(merged[1].curves_len(), 3);
            }
            _ => panic!("Unexpected"),
        }

        verify_difference(res, a, b);
    }
}

mod enveloping_circles {
    use crate::{
        scene::shape::boolean::{
            difference::shape_difference, intersection::shape_intersection, union::shape_union,
            ShapeDifference, ShapeIntersection, ShapeUnion,
        },
        Shape,
    };
    use common::types::{Coord, Length2d};

    fn create() -> (Shape, Shape) {
        let a = Shape::new_circle(Coord::new(0.0, 0.0), Length2d::new(0.7, 0.7));
        let b = Shape::new_circle(Coord::new(0.1, 0.0), Length2d::new(0.4, 0.4));
        (a, b)
    }

    #[test]
    fn union() {
        let (a, b) = create();

        let res = shape_union(&a, &b);

        match &res {
            ShapeUnion::A => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn intersection() {
        let (a, b) = create();

        let res = shape_intersection(&a, &b);

        match &res {
            ShapeIntersection::B => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn difference() {
        let (a, b) = create();

        let res = shape_difference(&a, &b);

        match &res {
            ShapeDifference::AWithBHole => {}
            _ => panic!("Unexpected result"),
        }
    }
}

mod squares_with_a_common_side {
    use common::pures::Affine;

    use super::{verify_difference, verify_intersection, verify_union};
    use crate::{
        scene::shape::boolean::{
            difference::shape_difference, intersection::shape_intersection, union::shape_union,
            ShapeDifference, ShapeIntersection, ShapeUnion,
        },
        DbCoord, Shape,
    };

    fn create() -> (Shape, Shape) {
        let a = Shape::new_from_lines(
            vec![
                DbCoord::new(-0.7, 0.0),
                DbCoord::new(-0.7, 0.9),
                DbCoord::new(0.3, 0.9),
                DbCoord::new(0.3, 0.0),
            ],
            Affine::identity(),
        );
        let b = Shape::new_from_lines(
            vec![
                DbCoord::new(-0.7, -0.7),
                DbCoord::new(0.7, -0.7),
                DbCoord::new(0.7, 0.7),
                DbCoord::new(-0.7, 0.7),
            ],
            Affine::identity(),
        );
        (a, b)
    }

    #[test]
    fn union() {
        let (a, b) = create();

        let res = shape_union(&a, &b);
        match &res {
            ShapeUnion::New(_) => {}
            _ => panic!("Unexpected result"),
        }

        verify_union(res, a, b);
    }

    #[test]
    fn intersection() {
        let (a, b) = create();

        let res = shape_intersection(&a, &b);

        match &res {
            ShapeIntersection::New(merged) => {
                assert_eq!(merged.len(), 1);
            }
            _ => panic!("Unexpected result"),
        }

        verify_intersection(res, a, b);
    }

    #[test]
    fn difference() {
        let (a, b) = create();

        let res = shape_difference(&a, &b);
        match &res {
            ShapeDifference::New(merged) => {
                assert_eq!(merged.len(), 1);
            }
            _ => panic!("Unexpected result"),
        }

        verify_difference(res, a, b);
    }
}

mod squares_with_a_common_side_flip {
    use common::pures::Affine;

    use super::{verify_difference, verify_intersection, verify_union};
    use crate::{
        scene::shape::boolean::{
            difference::shape_difference, intersection::shape_intersection, union::shape_union,
            ShapeDifference, ShapeIntersection, ShapeUnion,
        },
        DbCoord, Shape,
    };

    fn create() -> (Shape, Shape) {
        let a = Shape::new_from_lines(
            vec![
                DbCoord::new(-0.6, -0.6),
                DbCoord::new(0.6, -0.6),
                DbCoord::new(0.6, 0.6),
                DbCoord::new(-0.6, 0.6),
            ],
            Affine::identity(),
        );
        let b = Shape::new_from_lines(
            vec![
                DbCoord::new(0.3, -0.6),
                DbCoord::new(0.3, -0.3),
                DbCoord::new(0.8, -0.3),
                DbCoord::new(0.8, -0.6),
            ],
            Affine::identity(),
        );
        (a, b)
    }

    #[test]
    fn union() {
        let (a, b) = create();

        let res = shape_union(&a, &b);
        match &res {
            ShapeUnion::New(_) => {}
            _ => panic!("Unexpected result"),
        }

        verify_union(res, a, b);
    }

    #[test]
    fn intersection() {
        let (a, b) = create();

        let res = shape_intersection(&a, &b);

        match &res {
            ShapeIntersection::New(merged) => {
                assert_eq!(merged.len(), 1);
            }
            _ => panic!("Unexpected result"),
        }

        verify_intersection(res, a, b);
    }

    #[test]
    fn difference() {
        let (a, b) = create();

        let res = shape_difference(&a, &b);
        match &res {
            ShapeDifference::New(merged) => {
                assert_eq!(merged.len(), 1);
            }
            _ => panic!("Unexpected result"),
        }

        verify_difference(res, a, b);
    }
}

mod squares_sliding {
    use common::pures::Affine;

    use super::{verify_difference, verify_intersection, verify_union};
    use crate::{
        scene::shape::boolean::{
            difference::shape_difference, intersection::shape_intersection, union::shape_union,
            ShapeDifference, ShapeIntersection, ShapeUnion,
        },
        DbCoord, Shape,
    };

    fn create() -> (Shape, Shape) {
        let a = Shape::new_from_lines(
            vec![
                DbCoord::new(-0.7, -0.7),
                DbCoord::new(0.7, -0.7),
                DbCoord::new(0.7, 0.5),
                DbCoord::new(-0.7, 0.5),
            ],
            Affine::identity(),
        );
        let b = Shape::new_from_lines(
            vec![
                DbCoord::new(-0.7, -0.5),
                DbCoord::new(0.7, -0.5),
                DbCoord::new(0.7, 0.7),
                DbCoord::new(-0.7, 0.7),
            ],
            Affine::identity(),
        );
        (a, b)
    }

    #[test]
    fn union() {
        let (a, b) = create();

        let res = shape_union(&a, &b);

        match &res {
            ShapeUnion::New(_) => {}
            _ => panic!("Unexpected result"),
        }

        verify_union(res, a, b);
    }

    #[test]
    fn intersection() {
        let (a, b) = create();

        let res = shape_intersection(&a, &b);
        match &res {
            ShapeIntersection::New(merged) => {
                assert_eq!(merged.len(), 1);
            }
            _ => panic!("Unexpected result"),
        }

        verify_intersection(res, a, b);
    }

    #[test]
    fn difference() {
        let (a, b) = create();

        let res = shape_difference(&a, &b);
        match &res {
            ShapeDifference::New(merged) => {
                assert_eq!(merged.len(), 1);
            }
            _ => panic!("Unexpected result"),
        }

        verify_difference(res, a, b);
    }
}

mod squares_touching_outside {
    use common::pures::Affine;

    use super::{print_svg, verify_union};
    use crate::{
        scene::shape::boolean::{
            difference::shape_difference, intersection::shape_intersection, union::shape_union,
            ShapeDifference, ShapeIntersection, ShapeUnion,
        },
        DbCoord, Shape,
    };

    fn create() -> (Shape, Shape) {
        let a = Shape::new_from_lines(
            vec![
                DbCoord::new(-0.8, -0.8),
                DbCoord::new(0.8, -0.8),
                DbCoord::new(0.8, 0.0),
                DbCoord::new(-0.8, 0.0),
            ],
            Affine::identity(),
        );
        let b = Shape::new_from_lines(
            vec![
                DbCoord::new(-0.6, 0.0),
                DbCoord::new(0.6, 0.0),
                DbCoord::new(0.6, 0.8),
                DbCoord::new(-0.6, 0.8),
            ],
            Affine::identity(),
        );
        (a, b)
    }

    #[test]
    fn union() {
        let (a, b) = create();
        print_svg(&a, &b);

        let res = shape_union(&a, &b);

        match &res {
            ShapeUnion::New(_) => {}
            _ => panic!("Unexpected result"),
        }

        verify_union(res, a, b);
    }

    #[test]
    fn intersection() {
        let (a, b) = create();

        let res = shape_intersection(&a, &b);
        match &res {
            ShapeIntersection::None => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn difference() {
        let (a, b) = create();

        let res = shape_difference(&a, &b);
        match &res {
            ShapeDifference::A => {}
            _ => panic!("Unexpected result"),
        }
    }
}
