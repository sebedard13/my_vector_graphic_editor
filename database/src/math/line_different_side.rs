use common::{
    pures::Vec2,
    types::{Coord, Vector},
    PRECISION,
};

pub fn points_are_different_side(
    point: &Coord,
    point2: &Coord,
    l1: &Coord,
    l1v: &Vector,
    l2: &Coord,
    l2v: &Vector,
) -> bool {
    let vertices = create_shape_from_infinite_lines(point, point2, l1, l1v, l2, l2v);
    contains_point(&vertices, point) != contains_point(&vertices, point2)
}

fn create_shape_from_infinite_lines(
    range1: &Coord,
    range2: &Coord,
    l1: &Coord,
    l1v: &Vector,
    l2: &Coord,
    l2v: &Vector,
) -> [Coord; 8] {
    let distance_max = ((range1.approx_distance(range2))
        .max(l1.approx_distance(l2))
        .max(range1.approx_distance(l1))
        .max(range2.approx_distance(l2)))
    .sqrt();

    let l1v_coord = Coord::from(*l1v) * distance_max * 4.0 + l1;
    let l2v_coord = Coord::from(*l2v) * distance_max * 4.0 + l2;

    let min_inf = Coord::min(&Coord::min(&Coord::min(&l1v_coord, &l2v_coord), l1), l2)
        + Coord::new(-distance_max, -distance_max) * 2.0;
    let max_inf = Coord::max(&Coord::max(&Coord::max(&l1v_coord, &l2v_coord), l1), l2)
        + Coord::new(distance_max, distance_max) * 2.0;

    let (b0, b1, b2, b4) = {
        if l1v_coord.x() > l2v_coord.x() {
            (
                max_inf,
                Coord::new(max_inf.x(), min_inf.y()),
                min_inf,
                Coord::new(min_inf.x(), max_inf.y()),
            )
        } else {
            (
                Coord::new(min_inf.x(), max_inf.y()),
                min_inf,
                Coord::new(max_inf.x(), min_inf.y()),
                max_inf,
            )
        }
    };

    
    [l2v_coord, *l2, *l1, l1v_coord, b0, b1, b2, b4]
}

fn contains_point(vertices: &[Coord; 8], point: &Coord) -> bool {
    let mut count = 0;
    let mut point = *point;
    for i in 0..vertices.len() {
        let v1 = &vertices[i];
        let v2 = &vertices[(i + 1) % vertices.len()];

        if f32::abs(point.y() - v2.y()) <= PRECISION && f32::abs(point.y() - v1.y()) <= PRECISION && v1.x().min(v2.x()) <= point.x() && point.x() <= v1.x().max(v2.x()) {
            count += 1;
            continue;
        }

        if f32::abs(point.y() - v2.y()) <= PRECISION || f32::abs(point.y() - v1.y()) <= PRECISION {
            let change = {
                let max = f32::max(point.y().abs(), point.x().abs());
                if max <= 1.0 {
                    PRECISION * 2.0
                } else {
                    //TODO: the coord value should be close to -1.0 or 1.0
                    max.log10().ceil() * PRECISION * 2.0
                }
            };

            point = point + Coord::new(0.0, change);
        }

        if point.y() + PRECISION < v1.y().min(v2.y()) || point.y() - PRECISION > v1.y().max(v2.y())
        {
            continue;
        }

        let x =
            ((v1.x() - v2.x()) * point.y() - v1.x() * v2.y() + v2.x() * v1.y()) / (v1.y() - v2.y());

        if x < point.x() {
            count += 1;
        }
    }
    count % 2 == 1
}

#[cfg(test)]
mod test {

    use super::*;

    fn print_vertice_for_desmos(coords: [Coord; 8]) {
        let string = coords
            .iter()
            .map(|c| format!("({}, {})", c.x, c.y))
            .collect::<Vec<String>>()
            .join(", ");
        println!("{}", string);
    }

    #[test]
    fn test_points_are_different_side() {
        let point = Coord::new(-1.0, 1.0);
        let point2 = Coord::new(1.0, -1.0);
        let l1 = Coord::new(0.0, 0.0);
        let l1v = Vector::new(0.5, 0.25).normal();
        let l2 = Coord::new(0.0, 0.0);
        let l2v = Vector::new(-0.25, -0.5).normal();

        assert_eq!(
            points_are_different_side(&point, &point2, &l1, &l1v, &l2, &l2v),
            true
        );
    }

    #[test]
    fn test_points_are_different_side2() {
        let point = Coord::new(-1.0, 1.0);
        let point2 = Coord::new(1.0, -1.0);
        let l1 = Coord::new(0.2, 0.2);
        let l1v = Vector::new(1.0, 0.0).normal();
        let l2 = Coord::new(-0.2, -0.2);
        let l2v = Vector::new(100.0 + 0.2, -1000.0 + 0.2).normal();

        assert_eq!(
            points_are_different_side(&point, &point2, &l1, &l1v, &l2, &l2v),
            true
        );
    }

    #[test]
    fn create_shape_from_infinite_lines_convex() {
        let point = Coord::new(-1.0, 1.0);
        let point2 = Coord::new(1.0, -1.0);
        let l1 = Coord::new(0.2, 0.2);
        let l1v = Vector::new(1.0, 0.0).normal();
        let l2 = Coord::new(-0.2, -0.2);
        let l2v = Vector::new(100.0 + 0.2, -1000.0 + 0.2).normal();

        let vertices = create_shape_from_infinite_lines(&point, &point2, &l1, &l1v, &l2, &l2v);
        let left = contains_point(&vertices, &point);
        let right = contains_point(&vertices, &point2);
        assert_ne!(left, right);

        assert_eq!(left, contains_point(&vertices, &Coord::new(-0.13, 0.08)));
        assert_eq!(left, contains_point(&vertices, &Coord::new(-0.193, -0.902)));
        assert_eq!(left, contains_point(&vertices, &Coord::new(0.79, 0.614)));
        assert_eq!(left, contains_point(&vertices, &Coord::new(-0.083, 0.02)));
        assert_eq!(left, contains_point(&vertices, &Coord::new(-0.230066, 0.1)));

        assert_eq!(right, contains_point(&vertices, &Coord::new(0.064, -0.072)));
        assert_eq!(right, contains_point(&vertices, &Coord::new(0.667, -0.68)));
    }

    #[test]
    fn create_shape_from_infinite_lines_concav() {
        let point = Coord::new(-1.0, 1.0);
        let point2 = Coord::new(1.0, -1.0);
        let l1 = Coord::new(-0.2, 0.2);
        let l1v = Vector::new(-100.0, -1000.0).normal();
        let l2 = Coord::new(0.2, -0.2);
        let l2v = Vector::new(100.0, 100.0).normal();

        let vertices = create_shape_from_infinite_lines(&point, &point2, &l1, &l1v, &l2, &l2v);
        let left = contains_point(&vertices, &point);
        let right = contains_point(&vertices, &point2);
        assert_ne!(left, right);

        assert_eq!(left, contains_point(&vertices, &Coord::new(1.0, 1.0)));
        assert_eq!(left, contains_point(&vertices, &Coord::new(-1.0, -1.0)));
        assert_eq!(left, contains_point(&vertices, &Coord::new(-0.417, -0.033)));
        assert_eq!(left, contains_point(&vertices, &Coord::new(0.186, -0.09)));
        assert_eq!(left, contains_point(&vertices, &Coord::new(-0.337, -0.53)));

        assert_eq!(right, contains_point(&vertices, &Coord::new(-0.153, 0.05)));
        assert_eq!(right, contains_point(&vertices, &Coord::new(0.607, -0.112)));
        assert_eq!(right, contains_point(&vertices, &Coord::new(-0.2, -0.527)));
    }

    #[test]
    fn create_shape_from_infinite_lines_bug01() {
        //Coord { c: Vector { x: 0.69999987, y: -0.5055999 } }, Coord { c: Vector { x: 0.6607999, y: 0.49999994 } }, Coord { c: Vector { x: 0.7, y: -0.49999997 } }, Vector { x: -1.0, y: -6.386212e-8 }, Coord { c: Vector { x: 0.7, y: 0.5 } }, Vector { x: 0.0, y: 1.0 }
        let point = Coord::new(0.69999987, -0.5055999);
        let point2 = Coord::new(0.6607999, 0.49999994);
        let l1 = Coord::new(0.7, -0.49999997);
        let l1v = Vector::new(-1.0, -6.386212e-8).normal();
        let l2 = Coord::new(0.7, 0.5);
        let l2v = Vector::new(0.0, 1.0).normal();

        let vertices = create_shape_from_infinite_lines(&point, &point2, &l1, &l1v, &l2, &l2v);

        let left = contains_point(&vertices, &point);
        let right = contains_point(&vertices, &point2);
        assert_ne!(left, right);
    }

    #[test]
    fn create_shape_from_infinite_lines_bug02() {
        //Coord { c: Vector { x: -0.67199993, y: 0.0 } }, Coord { c: Vector { x: -0.69999987, y: 0.7055998 } }, Coord { c: Vector { x: -0.7, y: 0.6999999 } }, Vector { x: 1.0, y: 5.9604645e-8 }, Coord { c: Vector { x: -0.7, y: 0.0 } }, Vector { x: 0.0, y: -1.0 }
        let point = Coord::new(-0.67199993, 0.0);
        let point2 = Coord::new(-0.69999987, 0.7055998);

        let l1 = Coord::new(-0.7, 0.6999999);
        let l1v = Vector::new(1.0, 5.9604645e-8).normal();
        let l2 = Coord::new(-0.7, 0.0);
        let l2v = Vector::new(0.0, -1.0).normal();

        let vertices = create_shape_from_infinite_lines(&point, &point2, &l1, &l1v, &l2, &l2v);

        let left = contains_point(&vertices, &point);
        let right = contains_point(&vertices, &point2);
        assert_ne!(left, right);
    }

    #[test]
    fn create_shape_from_infinite_lines_bug03() {
        //Coord { c: Vector { x: -0.6607999, y: 0.49999994 } }, Coord { c: Vector { x: -0.69999987, y: -0.5056 } }, Coord { c: Vector { x: -0.7, y: 0.5 } }, Vector { x: 0.0, y: 1.0 }, Coord { c: Vector { x: -0.7, y: -0.50000006 } }, Vector { x: 1.0, y: 6.386212e-8 }
        let point = Coord::new(-0.6607999, 0.49999994);
        let point2 = Coord::new(-0.69999987, -0.5056);

        let l1 = Coord::new(-0.7, 0.5);
        let l1v = Vector::new(0.0, 1.0).normal();
        let l2 = Coord::new(-0.7, -0.50000006);
        let l2v = Vector::new(1.0, 6.386212e-8).normal();

        let vertices = create_shape_from_infinite_lines(&point, &point2, &l1, &l1v, &l2, &l2v);

        let left = contains_point(&vertices, &point);
        let right = contains_point(&vertices, &point2);
        assert_ne!(left, right);
    }

    #[test]
    fn create_shape_from_infinite_lines_bug04() {
        //Coord { c: Vector { x: -0.9439999, y: -0.9999999 } }, Coord { c: Vector { x: -0.8402806, y: -0.8402806 } }, Coord { c: Vector { x: -0.8437499, y: -0.8437499 } }, Vector { x: 0.6754437, y: -0.7374116 }, Coord { c: Vector { x: -1.0, y: -1.0 } }, Vector { x: 0.0, y: 1.0 }
        let point = Coord::new(-0.9439999, -0.9999999);
        let point2 = Coord::new(-0.8402806, -0.8402806);

        let l1 = Coord::new(-0.8437499, -0.8437499);
        let l1v = Vector::new(0.6754437, -0.7374116).normal();
        let l2 = Coord::new(-1.0, -1.0);
        let l2v = Vector::new(0.0, 1.0).normal();

        let vertices = create_shape_from_infinite_lines(&point, &point2, &l1, &l1v, &l2, &l2v);

        let left = contains_point(&vertices, &point);
        let right = contains_point(&vertices, &point2);
        assert_ne!(left, right);
    }

    #[test]
    fn create_shape_from_infinite_lines_bug05() {
        //Coord { c: Vector { x: -0.83700687, y: -0.8506483 } }-false, Coord { c: Vector { x: -0.9999999, y: -0.9439999 } }-false, Coord { c: Vector { x: -1.0, y: -1.0 } }, Vector { x: 1.0, y: 0.0 }, Coord { c: Vector { x: -0.8437499, y: -0.8437499 } }, Vector { x: 0.70710677, y: 0.70710677 }
        let point = Coord::new(-0.83700687, -0.8506483);
        let point2 = Coord::new(-0.9999999, -0.9439999);

        let l1 = Coord::new(-1.0, -1.0);
        let l1v = Vector::new(1.0, 0.0).normal();
        let l2 = Coord::new(-0.8437499, -0.8437499);
        let l2v = Vector::new(0.70710677, 0.70710677).normal();

        let vertices = create_shape_from_infinite_lines(&point, &point2, &l1, &l1v, &l2, &l2v);

        let left = contains_point(&vertices, &point);
        let right = contains_point(&vertices, &point2);
        assert_ne!(left, right);
    }

    #[test]
    fn create_shape_from_infinite_lines_bug06() {
        // Coord { c: Vector { x: 1.2599999, y: 630.0 } } Coord { c: Vector { x: 0.0, y: 672.5699 } } Coord { c: Vector { x: 0.0, y: 630.0 } } Vector { x: -2.5014407e-8, y: -1.0 } Coord { c: Vector { x: 0.0, y: 672.5 } } Vector { x: 1.0, y: 0.0 }

        let point = Coord::new(1.2599999, 630.0);
        let point2 = Coord::new(0.0, 672.5699);

        let l1 = Coord::new(0.0, 630.0);
        let l1v = Vector::new(-2.5014407e-8, -1.0).normal();
        let l2 = Coord::new(0.0, 672.5);
        let l2v = Vector::new(1.0, 0.0).normal();

        let vertices = create_shape_from_infinite_lines(&point, &point2, &l1, &l1v, &l2, &l2v);
        print_vertice_for_desmos(vertices);

        let left = contains_point(&vertices, &point);
        let right = contains_point(&vertices, &point2);
        assert_ne!(left, right);
    }
}
