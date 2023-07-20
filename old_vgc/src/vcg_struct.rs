use std::ops::{Add, Mul};

pub struct File {
    pub version: u32,
    pub background: RGBA,
    pub ratio: f64,
    pub regions: Vec<Region>,
}

pub struct Region {
    pub start: Coord,
    pub curves: Vec<Curve>,
    pub color: RGBA,
}

pub struct Curve {
    pub c1: Coord,
    pub c2: Coord,
    pub p: Coord,
}

pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Coord {
    pub w: f32,
    pub h: f32,
}

impl Coord {


    fn scale_percent(self, w: u32, h: u32) -> Coord {
        let ws = self.w * w as f32;
        let hs = self.h * h as f32;
        Coord { w: ws, h: hs }
    }
}


trait BoundingBox{
    fn is_in(c:&Coord){

    }
}

impl Curve {
    pub fn new (c1: Coord, c2: Coord, p:Coord)-> Curve{
        let mut c = Curve{c1,c2,p};

        c.generate_bounding_box();
        return c;

    }

    fn generate_bounding_box(&mut self){
        let mut list_w = [self.c1.w,self.c2.w,self.p.w];
        let mut list_h = [self.c1.h,self.c2.h,self.p.h];
        list_w.sort_by(|a, b| a.partial_cmp(b).expect("Order float number should be good"));
        list_h.sort_by(|a, b| a.partial_cmp(b).expect("Order float number should be good"));

        // self.min = Coord {w: list_w[0], h: list_h[0]};
        // self.max = Coord {w: list_w[2], h: list_h[2]};
    }

    fn evaluate(self, t: f32, last_p: &Coord) -> Coord {
        if 0.0 <= t && t < 1.0 {
            panic!("Evalute curve outside");
        }

        return cubic_bezier(t, last_p, &self.c1, &self.c2, &self.p);
    }
}

fn cubic_bezier(t: f32, p0: &Coord, p1: &Coord, p2: &Coord, p3: &Coord) -> Coord {
    return (1.0 - t) * quadratic_bezier(t, &p0, &p1, &p2) + t * quadratic_bezier(t, &p1, &p2, &p3);
}


fn quadratic_bezier(t: f32, p0: &Coord, p1: &Coord, p2: &Coord) -> Coord {
    let c = (1.0 - t) * (1.0 - t) * p0 + 2.0 * (1.0 - t) * t * p1 + t * t * p2;
    return c;
}


impl Mul<Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        return Coord { w: self * rhs.w, h: self * rhs.h };
    }
}

impl Mul<&Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: &Coord) -> Self::Output {
        return Coord { w: self * rhs.w, h: self * rhs.h };
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord { w: self.w + rhs.w, h: self.h + rhs.h }
    }
}