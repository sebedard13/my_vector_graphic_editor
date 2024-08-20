use common::types::Coord;

use super::curve2::{IntersectionPoint, IntersectionResult};

pub fn line_intersection(
    c1_p0: &Coord,
    c2_p0: &Coord,
    c2_p1: &Coord,
    c1_p1: &Coord,
) -> IntersectionResult {
    let div = c1_p0.x() * (c2_p0.y() - c2_p1.y())
        - c1_p0.y() * (c2_p0.x() - c2_p1.x())
        - c1_p1.x() * (c2_p0.y() - c2_p1.y())
        + c1_p1.y() * (c2_p0.x() - c2_p1.x());

    if div == 0.0 {
        // The lines are parallel or coincident
        // Check any point extremeties are equal
        if c1_p0 == c2_p0 {
            return IntersectionResult::Pts(vec![IntersectionPoint {
                coord: *c1_p0,
                t1: 0.0,
                t2: 0.0,
            }]);
        } else if c1_p0 == c2_p1 {
            return IntersectionResult::Pts(vec![IntersectionPoint {
                coord: *c1_p0,
                t1: 0.0,
                t2: 1.0,
            }]);
        } else if c1_p1 == c2_p0 {
            return IntersectionResult::Pts(vec![IntersectionPoint {
                coord: *c1_p1,
                t1: 1.0,
                t2: 0.0,
            }]);
        } else if c1_p1 == c2_p1 {
            return IntersectionResult::Pts(vec![IntersectionPoint {
                coord: *c1_p1,
                t1: 1.0,
                t2: 1.0,
            }]);
        }

        return IntersectionResult::None;
    }

    let mut t1 = (c1_p0.x() * (c2_p0.y() - c2_p1.y()) - c1_p0.y() * (c2_p0.x() - c2_p1.x())
        + c2_p0.x() * c2_p1.y()
        - c2_p0.y() * c2_p1.x())
        / div;
    let mut t2 = -(c1_p0.x() * (c1_p1.y() - c2_p0.y()) - c1_p0.y() * (c1_p1.x() - c2_p0.x())
        + c1_p1.x() * c2_p0.y()
        - c1_p1.y() * c2_p0.x())
        / div;

    if 0.0 > t1 || t1 > 1.0 || 0.0 > t2 || t2 > 1.0 {
        return IntersectionResult::None;
    }

    // Precision problem see longer_line_bug02()
    // if line_eval(c1_p0, c1_p1, t1) != line_eval(c2_p0, c2_p1, t2) {
    //     println!("eval diff {:?} {:?}", line_eval(c1_p0, c1_p1, t1), line_eval(c2_p0, c2_p1, t2));
    //     println!("({:?}, {:?}) and ({:?}, {:?})", c1_p0, c1_p1, c2_p0, c2_p1);
    //     return IntersectionResult::None;
    // }

    if line_eval(c1_p0, c1_p1, t1) == *c1_p0 {
        t1 = 0.0;
    } else if line_eval(c1_p0, c1_p1, t1) == *c1_p1 {
        t1 = 1.0;
    }

    if line_eval(c2_p0, c2_p1, t2) == *c2_p0 {
        t2 = 0.0;
    } else if line_eval(c2_p0, c2_p1, t2) == *c2_p1 {
        t2 = 1.0;
    }

    return IntersectionResult::Pts(vec![IntersectionPoint {
        coord: line_eval(c1_p0, c1_p1, t1),
        t1,
        t2,
    }]);
}

fn line_eval(c0: &Coord, c1: &Coord, t: f32) -> Coord {
    return (1.0 - t) * c0 + t * c1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_line_intersection_then_in_middle() {
        let c1_p0 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(1.0, 1.0);

        let c2_p0 = Coord::new(0.0, 1.0);
        let c2_p1 = Coord::new(1.0, 0.0);

        let result = line_intersection(&c1_p0, &c2_p0, &c2_p1, &c1_p1);
        match result {
            IntersectionResult::Pts(pts) => {
                assert_eq!(pts.len(), 1);
                assert_eq!(pts[0].coord, Coord::new(0.5, 0.5));
                assert_eq!(pts[0].t1, 0.5);
                assert_eq!(pts[0].t2, 0.5);
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn when_line_intersection_then_not_in_middle() {
        let c1_p0 = Coord::new(0.25, 0.25);
        let c1_p1 = Coord::new(1.0, 1.0);

        let c2_p0 = Coord::new(0.4, 0.0);
        let c2_p1 = Coord::new(0.4, 5.0);

        let result = line_intersection(&c1_p0, &c2_p0, &c2_p1, &c1_p1);
        match result {
            IntersectionResult::Pts(pts) => {
                assert_eq!(pts.len(), 1);
                assert_eq!(pts[0].coord, Coord::new(0.4, 0.4));
                assert_eq!(pts[0].t1, 0.2);
                assert_eq!(pts[0].t2, 0.080000006);
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_line_extremeties() {
        //[2024-08-02T18:25:05Z INFO  database::math::curve2] c1: Coord { c: Vec2 { x: -0.45359507, y: -0.45359507 } } Coord { c: Vec2 { x: -0.45359507, y: -0.45359507 } } Coord { c: Vec2 { x: 0.0, y: 0.0 } } Coord { c: Vec2 { x: 0.0, y: 0.0 } }
        //[2024-08-02T18:25:05Z INFO  database::math::curve2] c2: Coord { c: Vec2 { x: 1.0, y: 1.0 } } Coord { c: Vec2 { x: 1.0, y: 1.0 } } Coord { c: Vec2 { x: 0.0, y: 0.0 } } Coord { c: Vec2 { x: 0.0, y: 0.0 } }

        let c1_p0 = Coord::new(-0.45359507, -0.45359507);
        let c1_p1 = Coord::new(0.0, 0.0);

        let c2_p0 = Coord::new(1.0, 1.0);
        let c2_p1 = Coord::new(0.0, 0.0);

        let result = line_intersection(&c1_p0, &c2_p0, &c2_p1, &c1_p1);
        match result {
            IntersectionResult::Pts(pts) => {
                assert_eq!(pts.len(), 1);
                assert_eq!(pts[0].coord, Coord::new(0.0, 0.0));
                assert_eq!(pts[0].t1, 1.0);
                assert_eq!(pts[0].t2, 1.0);
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_other_extremeties() {
        //[2024-08-02T18:34:09Z ERROR database::math::curve2] Different result for line intersection and bezier intersection
        //line: Pts([IntersectionPoint { coord: Coord { c: Vec2 { x: 0.0, y: 0.0 } }, t1: 1.0, t2: 1.0 }])
        //bezier: Pts([IntersectionPoint { coord: Coord { c: Vec2 { x: 1.0, y: 1.0 } }, t1: 1.0, t2: 1.0 }])
        //[2024-08-02T18:34:09Z INFO  database::math::curve2] c1: Coord { c: Vec2 { x: 0.0, y: 0.0 } } Coord { c: Vec2 { x: 0.0, y: 0.0 } } Coord { c: Vec2 { x: 1.0, y: 1.0 } } Coord { c: Vec2 { x: 1.0, y: 1.0 } }
        //[2024-08-02T18:34:09Z INFO  database::math::curve2] c2: Coord { c: Vec2 { x: -1.0, y: 1.0 } } Coord { c: Vec2 { x: -1.0, y: 1.0 } } Coord { c: Vec2 { x: 1.0, y: 1.0 } } Coord { c: Vec2 { x: 1.0, y: 1.0 } }

        let c1_p0 = Coord::new(0.0, 0.0);
        let c1_p1 = Coord::new(1.0, 1.0);

        let c2_p0 = Coord::new(-1.0, 1.0);
        let c2_p1 = Coord::new(1.0, 1.0);

        let result = line_intersection(&c1_p0, &c2_p0, &c2_p1, &c1_p1);
        match result {
            IntersectionResult::Pts(pts) => {
                assert_eq!(pts.len(), 1);
                assert_eq!(pts[0].coord, Coord::new(1.0, 1.0));
                assert_eq!(pts[0].t1, 1.0);
                assert_eq!(pts[0].t2, 1.0);
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn line_intersection_bug01() {
        //(Vec2 { x: 0.7, y: 0.5 }, Vec2 { x: 0.7, y: 0.5 }, Vec2 { x: -0.7, y: 0.5 }, Vec2 { x: -0.7, y: 0.5 }) and (Vec2 { x: 0.7, y: -0.5 }, Vec2 { x: 0.7, y: -0.5 }, Vec2 { x: 0.7, y: 0.7 }, Vec2 { x: 0.7, y: 0.7 })
        let c1_p0 = Coord::new(0.7, 0.5);
        let c1_p1 = Coord::new(-0.7, 0.5);

        let c2_p0 = Coord::new(0.7, -0.5);
        let c2_p1 = Coord::new(0.7, 0.7);

        let result = line_intersection(&c1_p0, &c2_p0, &c2_p1, &c1_p1);
        match result {
            IntersectionResult::Pts(pts) => {
                assert_eq!(pts.len(), 1);
                assert_eq!(pts[0].coord, Coord::new(0.7, 0.5));
                assert_eq!(pts[0].t1, 0.0);
                assert_eq!(pts[0].t2, 0.833333313);
            }
            _ => panic!("Unexpected result"),
        }
    }


    #[test]
    fn longer_line_bug02(){
        //(Coord { c: Vec2 { x: -45.0, y: -45.0 } }, Coord { c: Vec2 { x: -45.0, y: -90.0 } }) and (Coord { c: Vec2 { x: 654.5, y: -58.5 } }, Coord { c: Vec2 { x: -95.5, y: -58.5 } })
        let c1_p0 = Coord::new(-45.0, -45.0);
        let c1_p1 = Coord::new(-45.0, -90.0);

        let c2_p0 = Coord::new(654.5, -58.5);
        let c2_p1 = Coord::new(-95.5, -58.5);

        let result = line_intersection(&c1_p0, &c2_p0, &c2_p1, &c1_p1);
        match result {
            IntersectionResult::Pts(pts) => {
                assert_eq!(pts.len(), 1);
                assert_eq!(pts[0].coord, Coord::new(-45.0, -58.5));
                assert_eq!(pts[0].t1, 0.3);
                assert_eq!(pts[0].t2, 0.93266666);
            }
            _ => panic!("Unexpected result"),
        }
    }
}
