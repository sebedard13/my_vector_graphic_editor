use common::{
    pures::Vec2,
    types::{Coord, Rect},
};
use polynomen::Poly;

use crate::curve::cubic_bezier;

pub fn bounding_box(p0: &Coord, cp0: &Coord, cp1: &Coord, p1: &Coord) -> Rect {
    let extremities = extremites(&p0.c, &cp0.c, &cp1.c, &p1.c);

    let mut min = Vec2::new(f32::MAX, f32::MAX);
    let mut max = Vec2::new(f32::MIN, f32::MIN);

    for t in extremities {
        let value = cubic_bezier(t, &p0.c, &cp0.c, &cp1.c, &p1.c);

        min = Vec2::min(&min, &value);
        max = Vec2::max(&max, &value);
    }

    Rect {
        top_left: Coord { c: min },
        bottom_right: Coord { c: max },
    }
}

/// Returns t extremities of the curve in the order of t smallest to t largest
pub fn extremites(p0: &Vec2, cp0: &Vec2, cp1: &Vec2, p1: &Vec2) -> Vec<f32> {
    let mut vec = Vec::new();
    vec.push(0.0);
    vec.push(1.0);

    // for first derivative
    let d1a = 3.0 * (-p0 + 3.0 * cp0 - 3.0 * cp1 + p1);
    let d1b = 6.0 * (p0 - 2.0 * cp0 + cp1);
    let d1c = 3.0 * (cp0 - p0);

    // for second derivative
    let d2a = 6.0 * (-p0 + 3.0 * cp0 - 3.0 * cp1 + p1);
    let d2b = 6.0 * (p0 - 2.0 * cp0 + cp1);

    Poly::new_from_coeffs(&vec![d1c.x / d1a.x, d1b.x / d1a.x, 1.0])
        .real_roots()
        .and_then(|roots| {
            for root in roots {
                if root > 0.0 && root < 1.0 {
                    vec.push(root);
                }
            }
            Some(())
        });

    Poly::new_from_coeffs(&vec![d1c.y / d1a.y, d1b.y / d1a.y, 1.0])
        .real_roots()
        .and_then(|roots| {
            for root in roots {
                if root > 0.0 && root < 1.0 {
                    vec.push(root);
                }
            }
            Some(())
        });

    Poly::new_from_coeffs(&vec![d2b.x / d2a.x, 1.0])
        .real_roots()
        .and_then(|roots| {
            for root in roots {
                if root > 0.0 && root < 1.0 {
                    vec.push(root);
                }
            }
            Some(())
        });

    Poly::new_from_coeffs(&vec![d2b.y / d2a.y, 1.0])
        .real_roots()
        .and_then(|roots| {
            for root in roots {
                if root > 0.0 && root < 1.0 {
                    vec.push(root);
                }
            }
            Some(())
        });

    vec.sort_by(|a, b| a.partial_cmp(b).expect("No Nan value possible"));
    vec
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;

   #[test]
   fn test_bounding_box(){
         let p0 = Coord::new(110.0, 150.0);
         let cp0 = Coord::new(25.0, 190.0);
         let cp1 = Coord::new(210.0, 250.0);
         let p1 = Coord::new(210.0, 30.0);
    
         let rect = bounding_box(&p0, &cp0, &cp1, &p1);
    
         approx_eq!(f32, rect.top_left.x(), 87.6645332689);
         approx_eq!(f32, rect.top_left.y(), 30.0);
         approx_eq!(f32, rect.bottom_right.x(), 210.0);
         approx_eq!(f32, rect.bottom_right.y(), 188.862345822);
    }

    #[test]
    fn test_extremites() {
        let p0 = Vec2::new(110.0, 150.0);
        let cp0 = Vec2::new(25.0, 190.0);
        let cp1 = Vec2::new(210.0, 250.0);
        let p1 = Vec2::new(210.0, 30.0);

        let vec = extremites(&p0, &cp0, &cp1, &p1);

        assert_eq!(vec.len(), 6);
        approx_eq!(f32, vec[0], 0.0);
        approx_eq!(f32, vec[1], 0.066666666667);
        approx_eq!(f32, vec[2], 0.186813186813);
        approx_eq!(f32, vec[3], 0.437850957522);
        approx_eq!(f32, vec[4], 0.593406593407);
        approx_eq!(f32, vec[5], 1.0);
    }
}
