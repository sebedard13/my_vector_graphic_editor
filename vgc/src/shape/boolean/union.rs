use super::{
    create_shape, find_intersecions, mark_entry_exit_points, GreinerShape, IntersectionType,
};
use crate::{curve::Curve, shape::Shape};

pub enum ShapeUnion {
    /// A contains fully B
    A,
    /// B contains fully A
    B,
    /// A and B do not fully contain each other
    /// New shape is created    
    New(Shape),
    /// A and B do not intersect each other
    None,
}

#[allow(dead_code)]
pub fn shape_union(a: &Shape, b: &Shape) -> ShapeUnion {
    let (intersections_a, intersections_b) = find_intersecions(a, b);

    if intersections_a.is_empty() && intersections_b.is_empty() {
        if a.contains(&b.start.borrow()) {
            return ShapeUnion::A;
        } else if b.contains(&a.start.borrow()) {
            return ShapeUnion::B;
        } else {
            return ShapeUnion::None;
        }
    }

    let mut ag = create_shape(a, intersections_a);
    let mut bg = create_shape(b, intersections_b);

    mark_entry_exit_points(&mut ag, a, &mut bg, b);

    let merge_shape = do_union(&ag, &bg, a, b);

    ShapeUnion::New(merge_shape)
}

fn do_union(ag: &GreinerShape, bg: &GreinerShape, a: &Shape, _b: &Shape) -> Shape {
    let first_intersection = &ag.data[0];

    let mut merged = Shape {
        start: first_intersection.coord_ptr(),
        curves: Vec::new(),
        color: a.color.clone(),
    };

    let mut current = first_intersection;
    let mut current_shape = ag;
    loop {
        if !current.entry {
            //Remove ! to make the algo A AND B
            loop {
                let next = current.next.unwrap();
                current = &current_shape.data[next];
                let cp0 = current.coord_ptr();

                let next = current.next.unwrap();
                current = &current_shape.data[next];
                let cp1 = current.coord_ptr();

                let next = current.next.unwrap();
                current = &current_shape.data[next];
                let p1 = current.coord_ptr();

                merged.curves.push(Curve::new(cp0, cp1, p1));

                if current.intersect == IntersectionType::Intersection
                    || current.intersect == IntersectionType::CommonIntersection
                {
                    break;
                }
            }
        } else {
            loop {
                let next = current.prev.unwrap();
                current = &current_shape.data[next];
                let cp0 = current.coord_ptr();

                let next = current.prev.unwrap();
                current = &current_shape.data[next];
                let cp1 = current.coord_ptr();

                let next = current.prev.unwrap();
                current = &current_shape.data[next];
                let p1 = current.coord_ptr();

                merged.curves.push(Curve::new(cp0, cp1, p1));

                if current.intersect == IntersectionType::Intersection
                    || current.intersect == IntersectionType::CommonIntersection
                {
                    break;
                }
            }
        }
        let next = current.neighbor.unwrap();
        let eq = std::ptr::eq(current_shape, ag);
        if eq {
            current_shape = bg;
        } else {
            current_shape = ag;
        }
        current = &current_shape.data[next];

        // first interaction is from ag
        if std::ptr::eq(current, first_intersection)
            || std::ptr::eq(
                current,
                bg.data.get(first_intersection.neighbor.unwrap()).unwrap(),
            )
        {
            break;
        }
    }

    let len_last = merged.curves.len() - 1;
    merged.curves[len_last].p1 = merged.start.clone();

    merged
}

#[cfg(test)]
mod test {
    use super::{shape_union, ShapeUnion};
    use common::pures::Affine;
    use common::{types::Coord, Rgba};

    use crate::shape::Shape;
    use crate::{create_circle, Vgc};

    #[test]
    fn given_two_circle_when_union_then_new() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.2, 0.0), 0.2, 0.2);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_union(&a, &b);
        let merged = match merged {
            ShapeUnion::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.curves.len(), 8);

        let steps = 5;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged.contains(&coord),
                    a.contains(&coord) || b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_two_oval_with_no_valid_p_when_union_then_new() {
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

        let merged = shape_union(&a, &b);

        let merged = match merged {
            ShapeUnion::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };

        assert_eq!(merged.curves.len(), 4);

        let steps = 5;
        for x in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
            for y in (0..steps).map(|x| ((x as f32 * 2.0) / steps as f32) - 1.0) {
                let coord = &Coord::new(x, y);
                assert_eq!(
                    merged.contains(&coord),
                    a.contains(&coord) || b.contains(&coord),
                    "Contains failed at ({}, {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn given_two_circle_when_union_then_a() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.1, 0.1);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_union(&a, &b);

        assert!(matches!(merged, ShapeUnion::A), "Should be ShapeUnion::A");
    }

    #[test]
    fn given_two_circle_when_union_then_none() {
        let mut vgc = Vgc::new(Rgba::new(255, 255, 255, 255));

        create_circle(&mut vgc, Coord::new(0.0, 0.0), 0.2, 0.2);
        create_circle(&mut vgc, Coord::new(0.3, 0.3), 0.1, 0.1);

        let a = vgc.get_shape(0).expect("Shape should exist");
        let b = vgc.get_shape(1).expect("Shape should exist");

        let merged = shape_union(&a, &b);

        assert!(
            matches!(merged, ShapeUnion::None),
            "Should be ShapeUnion::None"
        );
    }

    ///These two test are more about the intersection precision to not give two intersections for the same point
    #[test]
    fn given_bug_intersection_not_even_when_union_then_new() {
        let a_coords = vec![
            Coord::new(-0.99999994, -0.71117526),
            Coord::new(-1.0, -0.23080525),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(0.9, 1.0),
            Coord::new(0.9, 1.0),
            Coord::new(0.9, 1.0),
            Coord::new(-0.33028302, -0.23028299),
            Coord::new(-0.8108693, -0.7108693),
            Coord::new(-0.78689057, -0.746981),
            Coord::new(-0.7720803, -0.7968017),
            Coord::new(-0.77199256, -0.852),
            Coord::new(-0.7721685, -0.96268535),
            Coord::new(-0.831543, -1.0517472),
            Coord::new(-0.9053333, -1.052011),
            Coord::new(-0.97912353, -1.0517472),
            Coord::new(-1.038498, -0.96268535),
            Coord::new(-1.038674, -0.852),
            Coord::new(-1.0385865, -0.79695743),
            Coord::new(-1.0238594, -0.74726224),
            Coord::new(-0.99999994, -0.71117526),
        ];

        let b_coords = vec![
            Coord::new(-0.7186666, -0.52398896),
            Coord::new(-0.64487636, -0.5242528),
            Coord::new(-0.5855018, -0.6133146),
            Coord::new(-0.5853259, -0.724),
            Coord::new(-0.5855018, -0.8346853),
            Coord::new(-0.64487636, -0.9237472),
            Coord::new(-0.7186666, -0.924011),
            Coord::new(-0.79245687, -0.9237472),
            Coord::new(-0.85183144, -0.8346853),
            Coord::new(-0.8520073, -0.724),
            Coord::new(-0.85183144, -0.6133146),
            Coord::new(-0.79245687, -0.5242528),
            Coord::new(-0.7186666, -0.52398896),
        ];

        let a = Shape::new_from_path(&a_coords, Affine::identity(), Rgba::new(255, 255, 255, 255));
        let b = Shape::new_from_path(&b_coords, Affine::identity(), Rgba::new(255, 255, 255, 255));

        let merged = shape_union(&a, &b);

        let _ = match merged {
            ShapeUnion::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };
    }

    #[test]
    fn given_bug_intersection_not_even_when_union_then_new2() {
        let a_coords = vec![
            Coord::new(0.20751375, 0.34723622),
            Coord::new(0.21654128, 0.44478416),
            Coord::new(0.27201384, 0.5194509),
            Coord::new(0.33956885, 0.5196924),
            Coord::new(0.38424692, 0.5195327),
            Coord::new(0.4236402, 0.48681968),
            Coord::new(0.44780847, 0.43646392),
            Coord::new(0.47138536, 0.46824136),
            Coord::new(0.5025848, 0.4875699),
            Coord::new(0.5369022, 0.48769262),
            Coord::new(0.57090247, 0.48757103),
            Coord::new(0.6018421, 0.468597),
            Coord::new(0.6253404, 0.43734184),
            Coord::new(0.8221004, 0.7221004),
            Coord::new(1.0, 0.8999999),
            Coord::new(1.0, 0.9),
            Coord::new(1.0, -1.0),
            Coord::new(-0.23018986, -1.0),
            Coord::new(-0.7108151, -1.0),
            Coord::new(-0.6673614, -0.9868111),
            Coord::new(-0.63168496, -0.94219863),
            Coord::new(-0.61470246, -0.8825373),
            Coord::new(-0.6094471, -0.8834914),
            Coord::new(-0.6040959, -0.88399154),
            Coord::new(-0.5986666, -0.8840109),
            Coord::new(-0.5690645, -0.8839051),
            Coord::new(-0.5417824, -0.8695086),
            Coord::new(-0.5197115, -0.84515834),
            Coord::new(-0.49585035, -0.87908435),
            Coord::new(-0.46360314, -0.8998836),
            Coord::new(-0.42799997, -0.90001094),
            Coord::new(-0.35420972, -0.89974713),
            Coord::new(-0.29483518, -0.8106853),
            Coord::new(-0.29465926, -0.6999999),
            Coord::new(-0.2946701, -0.69315886),
            Coord::new(-0.29490715, -0.6864005),
            Coord::new(-0.31415755, -0.5105602),
            Coord::new(-0.2603052, -0.80185175),
            Coord::new(-0.25727442, -0.69053465),
            Coord::new(-0.2588198, -0.53187996),
            Coord::new(-0.19420975, -0.6837472),
            Coord::new(-0.03705758, -0.9386854),
            Coord::new(-0.13110375, -0.5053334),
            Coord::new(-0.13475603, -0.4231357),
            Coord::new(-0.15275288, -0.36880988),
            Coord::new(-0.18120365, -0.3321915),
            Coord::new(-0.1577722, -0.30772406),
            Coord::new(-0.13985686, -0.27226898),
            Coord::new(-0.13072033, -0.2307203),
            Coord::new(-0.09856181, -0.28831297),
            Coord::new(-0.08709261, -0.29515216),
            Coord::new(-0.02626367, -0.41720873),
            Coord::new(-0.038059883, -0.34654507),
            Coord::new(-0.09697605, -0.23089579),
            Coord::new(-0.10139105, -0.1580722),
            Coord::new(-0.07567976, -0.18505096),
            Coord::new(-0.056251522, -0.19848329),
            Coord::new(-0.037594244, -0.20499676),
            Coord::new(-0.07972273, -0.17144103),
            Coord::new(-0.10831648, -0.10507981),
            Coord::new(-0.10843848, -0.028318644),
            Coord::new(-0.108289905, 0.065165184),
            Coord::new(-0.06591298, 0.14322421),
            Coord::new(-0.008288192, 0.16538757),
            Coord::new(0.008089226, 0.2048078),
            Coord::new(0.033348244, 0.23499021),
            Coord::new(0.06335316, 0.24973136),
            Coord::new(0.086181894, 0.31055903),
            Coord::new(0.12956555, 0.35151365),
            Coord::new(0.17956889, 0.35169247),
            Coord::new(0.18916018, 0.35165817),
            Coord::new(0.19850793, 0.35012364),
            Coord::new(0.20751375, 0.34723622),
        ];

        let b_coords = vec![
            Coord::new(0.21956885, 0.39569262),
            Coord::new(0.2933591, 0.39542875),
            Coord::new(0.35273364, 0.30636695),
            Coord::new(0.35290956, 0.19568157),
            Coord::new(0.35273364, 0.0849962),
            Coord::new(0.2933591, -0.004065603),
            Coord::new(0.21956885, -0.004329473),
            Coord::new(0.1457786, -0.004065603),
            Coord::new(0.086404055, 0.0849962),
            Coord::new(0.08622815, 0.19568157),
            Coord::new(0.086404055, 0.30636695),
            Coord::new(0.1457786, 0.39542875),
            Coord::new(0.21956885, 0.39569262),
        ];

        let a = Shape::new_from_path(&a_coords, Affine::identity(), Rgba::new(255, 255, 255, 255));
        let b = Shape::new_from_path(&b_coords, Affine::identity(), Rgba::new(255, 255, 255, 255));

        let merged = shape_union(&a, &b);

        let _ = match merged {
            ShapeUnion::New(merged) => merged,
            _ => panic!("Should be a new shape"),
        };
    }

    #[test]
    fn union_line_triangle() {
        // A: M -1 1 C -1 1 -1 -1 -1 -1 C -1 -1 0 0 0 0 C 0 0 1 1 1 1 C 1 1 -1 1 -1 1 Z
        /*B: M -0.72533333 -0.4059889
        C -0.6515431 -0.40625277 -0.59216857 -0.49531457 -0.5919926 -0.60599995
        C -0.59216857 -0.7166853 -0.6515431 -0.80574715 -0.72533333 -0.80601096
        C -0.7991236 -0.80574715 -0.8584981 -0.7166853 -0.85867405 -0.60599995
        C -0.8584981 -0.49531457 -0.7991236 -0.40625277 -0.72533333 -0.4059889 Z*/

        let a_coords = vec![
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(-1.0, -1.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(-1.0, 1.0),
            Coord::new(-1.0, 1.0),
        ];

        let b_coords = vec![
            Coord::new(-0.72533333, -0.4059889),
            Coord::new(-0.6515431, -0.40625277),
            Coord::new(-0.59216857, -0.49531457),
            Coord::new(-0.5919926, -0.60599995),
            Coord::new(-0.59216857, -0.7166853),
            Coord::new(-0.6515431, -0.80574715),
            Coord::new(-0.72533333, -0.80601096),
            Coord::new(-0.7991236, -0.80574715),
            Coord::new(-0.8584981, -0.7166853),
            Coord::new(-0.85867405, -0.60599995),
            Coord::new(-0.8584981, -0.49531457),
            Coord::new(-0.7991236, -0.40625277),
            Coord::new(-0.72533333, -0.4059889),
        ];

        let a = Shape::new_from_path(&a_coords, Affine::identity(), Rgba::new(255, 255, 255, 255));
        let b = Shape::new_from_path(&b_coords, Affine::identity(), Rgba::new(255, 255, 255, 255));

        let intersections = super::find_intersecions(&a, &b);
        assert_eq!(intersections.0.len(), 2);

        assert_eq!(intersections.0[1].t, 0.299266011);

        let mut ag = super::create_shape(&a, intersections.0);
        let mut bg = super::create_shape(&b, intersections.1);

        super::mark_entry_exit_points(&mut ag, &a, &mut bg, &b);

        let valid_values = vec![-0.5923179, -0.7849242, -1.00, 1.00, 0.00];
        for i in 0..ag.data.len() {
            let coord = ag.data[i].coord;
            assert!(
                valid_values.iter().any(|v| (v - coord.x()).abs() < 0.0001)
                    && valid_values.iter().any(|v| (v - coord.y()).abs() < 0.0001),
                "Invalid value ({}, {})",
                coord.x(),
                coord.y()
            );
        }
    }
}
