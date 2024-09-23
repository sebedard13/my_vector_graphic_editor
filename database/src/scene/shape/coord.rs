use super::Shape;
use crate::scene::id::CoordId;
use common::pures::Affine;
use common::types::Coord;
use serde::{Deserialize, Serialize};

pub enum CoordType {
    P0,
    CP0,
    CP1,
    P1,
}

impl Shape {
    pub fn coord_select(&self, index: CoordId) -> Option<&DbCoord> {
        self.path.iter().find(|c| c.id == index)
    }

    pub fn coord_select_mut(&mut self, index: CoordId) -> Option<&mut DbCoord> {
        self.path.iter_mut().find(|c| c.id == index)
    }

    pub fn coord_set(&mut self, index: CoordId, coord: Coord) {
        self.path
            .iter_mut()
            .filter(|c| c.id == index)
            .for_each(|c| c.coord = coord);
    }

    pub(crate) fn coord_index_select(&self, index: CoordId) -> Option<(usize, CoordType)> {
        for (i, c) in self.path.iter().enumerate() {
            if c.id == index {
                if i == 0 {
                    return Some((i, CoordType::P0));
                } else if i % 3 == 1 {
                    return Some((i, CoordType::CP0));
                } else if i % 3 == 2 {
                    if self.path[i + 1].id == index {
                        return Some((i + 1, CoordType::P1));
                    }
                    return Some((i, CoordType::CP1));
                } else {
                    return Some((i, CoordType::P1));
                }
            }
        }
        None
    }

    pub fn coord_delete(&mut self, index: CoordId) -> Result<(), String> {
        let (index, coord_type) = self.coord_index_select(index).ok_or("Coord not found")?;
        match coord_type {
            CoordType::P0 => {
                assert_eq!(index, 0);
                if self.is_closed() {
                    let len = self.path.len();
                    if len == 4 {
                        self.path.clear();
                        return Ok(());
                    }
                    self.path.swap(len - 2, 2); // keep CPl of P1

                    self.path.remove(index); // P0
                    self.path.remove(index); // CPr
                    self.path.remove(index); // CPl of P0
                } else {
                    self.path.remove(index); //P0
                    self.path.remove(index); //CPr
                    self.path.remove(index); //CPl of P1
                }
            }
            CoordType::CP0 => {
                self.path[index] = self.path[index + 1];
            }
            CoordType::CP1 => {
                self.path[index] = self.path[index - 1];
            }
            CoordType::P1 => {
                if self.path.len() - 1 == index {
                    if !self.is_closed() {
                        self.path.remove(index - 2); //cp0
                        self.path.remove(index - 2); //cp1
                        self.path.remove(index - 2); //p1
                    } else {
                        let len = self.path.len();
                        if len == 4 {
                            self.path.clear();
                            return Ok(());
                        }
                        let index = if index == 0 { len - 1 } else { index };
                        self.path.swap(len - 1, index); // keep CPl of P1

                        self.path.remove(index); // P0
                        self.path.remove(index); // CPr
                        self.path.remove(index); // CPl of P0
                    }
                } else {
                    self.path.remove(index); //P1
                    self.path.remove(index); //CPl
                    self.path.remove(index - 1); //CPr
                }
            }
        }

        Ok(())
    }
}

impl Shape {
    pub fn curves_path_update_id(&mut self) {
        for coord in &mut self.path {
            if coord.id == CoordId::null() {
                coord.id.update();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DbCoord {
    pub id: CoordId,
    pub(in super::super) coord: Coord,
}

impl DbCoord {
    pub fn new(x: f32, y: f32) -> Self {
        DbCoord {
            id: CoordId::new(),
            coord: Coord::new(x, y),
        }
    }

    pub fn transform(&self, transform: &Affine) -> Self {
        DbCoord {
            id: self.id,
            coord: *transform * self.coord,
        }
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }
}

impl From<Coord> for DbCoord {
    fn from(coord: Coord) -> Self {
        DbCoord {
            id: CoordId::new(),
            coord,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_a_shape_not_closed_when_delete_coord_p0_then_1_elment() {
        let mut shape = Shape::new();
        shape.path = vec![
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
        ];

        let id3 = shape.path[3].id;

        shape.coord_delete(shape.path[0].id).unwrap();

        assert_eq!(shape.path.len(), 1);
        assert_eq!(shape.path[0].id, id3);
    }

    #[test]
    fn given_a_shape_not_closed_when_delete_coord_p1_then_1_elment() {
        let mut shape = Shape::new();
        shape.path = vec![
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
        ];
        let id0 = shape.path[0].id;

        shape.coord_delete(shape.path[3].id).unwrap();

        assert_eq!(shape.path.len(), 1);
        assert_eq!(shape.path[0].id, id0);
    }

    #[test]
    fn given_a_shape_closed_when_delete_coord_p0_then_0_elment() {
        let mut shape = Shape::new();
        shape.path = vec![
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
        ];
        shape.path[3].id = shape.path[0].id;

        shape.coord_delete(shape.path[0].id).unwrap();

        assert_eq!(shape.path.len(), 0);
    }

    #[test]
    fn given_a_shape_line_closed_when_delete_coord_p0_then_0_elment() {
        let mut shape = Shape::new_from_lines(
            vec![
                DbCoord::new(0.0, 0.0),
                DbCoord::new(0.0, 0.0),
                DbCoord::new(0.0, 0.0),
            ],
            Affine::identity(),
        );
        shape.path[3].id = shape.path[0].id;

        shape.coord_delete(shape.path[0].id).unwrap();

        assert_eq!(shape.path.len(), 7);
    }

    #[test]
    fn given_a_shape_closed_when_delete_coord_p1_then_0_elment() {
        let mut shape = Shape::new();
        shape.path = vec![
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
        ];
        shape.path[3].id = shape.path[0].id;

        shape.coord_delete(shape.path[3].id).unwrap();

        assert_eq!(shape.path.len(), 0);
    }

    #[test]
    fn given_a_shape_closed_when_delete_coord_p1_in_middle_then_4_elment() {
        let mut shape = Shape::new();
        shape.path = vec![
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
            DbCoord::new(0.0, 0.0),
        ];
        shape.path[6].id = shape.path[0].id;
        let id2 = shape.path[2].id;
        let id3 = shape.path[3].id;
        let id4 = shape.path[4].id;

        shape.coord_delete(shape.path[3].id).unwrap();

        assert_eq!(shape.path.len(), 4);
        assert!(shape.path.iter().all(|c| c.id != id3));
        assert!(shape.path.iter().all(|c| c.id != id2));
        assert!(shape.path.iter().all(|c| c.id != id4));
    }
}
