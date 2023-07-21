use std::ops::Add;

pub struct CoordIndex {
    i: usize,
}

pub struct CoordDS {
    array: Vec<Option<Coord>>,
    is_normalize: bool,
}

impl Default for CoordDS {
    fn default() -> Self {
        CoordDS { is_normalize: true, array: Vec::default() }
    }
}

impl CoordDS {
    pub fn new() -> Self {
        CoordDS::default()
    }

    pub fn insert(&mut self, c: Coord) -> CoordIndex {
        self.array.push(Some(c));
        CoordIndex { i: self.array.len() - 1 }
    }

    pub fn get(&self, coord_index: &CoordIndex) -> &Coord {
        self.array[coord_index.i].as_ref().expect("Coord should be valid from CoordInde")
    }

    pub fn modify(&mut self, coord_index: &CoordIndex, c: Coord) {
        self.array[coord_index.i] = Some(c);
    }


    pub fn scale(&self, w: f32, h: f32) -> Self {
        let mut arr = self.array.clone();

        arr.iter_mut().for_each(|op_c| {
            match op_c {
                Some(c) => {
                    c.x =c.x * w;
                    c.y =c.y * h;
                }
                None => {}
            };
        });

        CoordDS { array: arr, is_normalize: false }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use crate::coord::{Coord, CoordDS};

    #[test]
    fn scale_CoordDS() {
       let mut cds = CoordDS::new();
        cds.insert(Coord { x: 0.5, y: 0.2 });

        let sc_cds = cds.scale(10.0,5.0);


        assert!( approx_eq!(f32,  sc_cds.array[0].as_ref().unwrap().x, 5.0, ulps = 2) );
        assert!( approx_eq!(f32,  sc_cds.array[0].as_ref().unwrap().y, 1.0, ulps = 2) );
    }
}

#[derive(Clone)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

impl Coord {
    fn scale_percent(self, w: u32, h: u32) -> Coord {
        let ws = self.x * w as f32;
        let hs = self.y * h as f32;
        Coord { x: ws, y: hs }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}