use crate::read_common::{read_color, read_coord};
use crate::try_append;
use crate::vcg_struct::{Curve, Region};

pub fn read_regions(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<Vec<Region>, String> {
    let mut regions = Vec::new();
    loop {
        match char_iterator.next() {
            None => { break; }
            Some(c) => {
                match c {
                    'E' => {
                        let region = try_append!(read_region(&mut char_iterator),"Invalid Region");
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
        None => { return Err(format!("No valid start coord")); }
        Some(c) => {
            match c {
                'P' => {
                    read_coord(char_iterator)?
                }
                _ => { return Err(format!("No valid start coord")); }
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
        None => { return Err(format!("No region color")); }
        Some(c) => {
            match c {
                '#' => {
                    read_color(char_iterator)?
                }
                _ => {
                    return Err(format!("No region color"));
                }
            }
        }
    };


    return Ok(Region { start, curves, color });
}