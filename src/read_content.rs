use crate::read_common::{read_color, read_coord};
use crate::vcg_struct::{Curve, Region};

pub fn read_regions(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<Vec<Region>, String> {
    let mut regions = Vec::new();
    loop {
        match char_iterator.next() {
            None => { break; }
            Some(c) => {
                match c {
                    'E' => {
                        let region = read_region(&mut char_iterator)?;
                        regions.push(region);
                    }
                    _ => {}
                }
            }
        }
    }

    return Ok(regions);
}

fn read_region(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<Region, String> {
    let start = match char_iterator.next() {
        None => { return Err(format!("No valid start region")); }
        Some(c) => {
            match c {
                'P' => {
                    read_coord(char_iterator)?
                }
                _ => { return Err(format!("No valid start region")); }
            }
        }
    };

    let mut curves = Vec::new();
    loop {
        match char_iterator.next() {
            None => { break; }
            Some(c) => {
                match c {
                    '{' => {}
                    'C' => {
                        let c1 = read_coord(&mut char_iterator)?;
                        let c2 = read_coord(&mut char_iterator)?;
                        let p = read_coord(&mut char_iterator)?;
                        curves.push(Curve { c1, c2, p })
                    }
                    '}' => { break; }
                    _ => { break; }
                }
            }
        }
    }
    let color = match char_iterator.next() {
        None => { return Err(format!("No valid region color")); }
        Some(c) => {
            match c {
                '#' => {
                    read_color(char_iterator)?
                }
                _ => {
                    return Err(format!("No valid region color"));
                }
            }
        }
    };


    return Ok(Region { start, curves, color });
}