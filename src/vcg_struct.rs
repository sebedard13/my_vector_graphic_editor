
#[derive(Debug)]
pub struct File {
    pub version: u32,
    pub background: RGBA,
    pub ratio: f64,
    pub regions: Vec<Region>,
}

#[derive(Debug)]
pub struct Region {
    pub id: u32,
    pub start: Coord,
    pub curves: Vec<Curve>,
    pub color: RGBA,
}

#[derive(Debug)]
pub struct Curve {
    pub c1: Coord,
    pub c2: Coord,
    pub p: Coord,
}

#[derive(Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug)]
pub struct Coord {
    pub w: u32,
    pub h: u32,
}






