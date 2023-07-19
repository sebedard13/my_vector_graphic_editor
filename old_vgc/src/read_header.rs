use crate::{read_common::*, try_append};
use crate::vcg_struct::RGBA;

pub fn read_background(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<RGBA, String> {
    let error = "Not valid background color".into();

    match char_iterator.next() {
        None => {

            return Err(error)
        }
        Some(c) => {
            match c {
                '#' => {
                    Ok(try_append!(read_color(&mut char_iterator), error))
                }
                _ => {
                    return Err(error)
                }
            }
        }
    }
}

pub fn read_ratio(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<f64, String> {
    match char_iterator.next() {
        None => {
            return Err(String::from("Invalid ratio"))
        }
        Some(c) => {
            match c {
                'r' => {
                    let num = try_append!(read_number(&mut char_iterator), "Invalid ratio's numerator");
                    let denum = try_append!(read_number(&mut char_iterator), "Invalid ratio's denominator");

                    if denum == 0{
                        return Err(String::from("Invalid Ratio because division by 0"));
                    }
                    let x : f64 = (num)as f64 / (denum) as f64;
                    Ok(x)
                }
                _ => {
                    return Err(String::from("Invalid ratio"))
                }
            }
        }
    }
}

pub fn read_version(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<u32, String> {
    let version: u32;
    let error = "Not a valid VGC header".into();

    match char_iterator.next() {
        None => { return Err("No content".into()) }
        Some(c) => {
            match c {
                'V' => {}
                _ => {}
            }
        }
    }
    match char_iterator.next() {
        None => {
            return Err(error)
        }
        Some(c) => {
            match c {
                'G' => {}
                _ => { return Err(error); }
            }
        }
    }
    match char_iterator.next() {
        None => { return Err(error); }
        Some(c) => {
            match c {
                'C' => { version = try_append!(read_number(&mut char_iterator), error); }
                _ => { return Err(error); }
            }
        }
    }

    return Ok(version);
}
