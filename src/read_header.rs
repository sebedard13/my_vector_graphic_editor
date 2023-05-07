use crate::read_common;
use crate::vcg_struct::RGBA;

pub fn read_background(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<RGBA, String> {
    match char_iterator.next() {
        None => {
            return Err(String::from("Not valid background color"))
        }
        Some(c) => {
            match c {
                '#' => {
                    read_common::read_color(&mut char_iterator)
                }
                _ => {
                    return Err(String::from("Not valid background color"))
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
                    let num = read_common::read_number(&mut char_iterator)?;
                    let denum = read_common::read_number(&mut char_iterator)?;

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
    let error = Err(String::from("Not a valid VGC header"));

    match char_iterator.next() {
        None => { return Err(format!("No content")) }
        Some(c) => {
            match c {
                'V' => {}
                _ => {}
            }
        }
    }
    match char_iterator.next() {
        None => {
            return error
        }
        Some(c) => {
            match c {
                'G' => {}
                _ => { return error; }
            }
        }
    }
    match char_iterator.next() {
        None => { return error; }
        Some(c) => {
            match c {
                'C' => { version = read_common::read_number(&mut char_iterator)?; }
                _ => { return error; }
            }
        }
    }

    return Ok(version);
}
