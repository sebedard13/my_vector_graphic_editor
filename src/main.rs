use std::{fs, str};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::Add;
use std::slice::Iter;
use std::str::Chars;

use image::{ImageBuffer, Rgb, RgbImage};

fn uncompress() {

}

fn read_number(char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<u32, String> {
    let mut string = String::new();
    loop {
        match char_iterator.next() {
            None => {
                break
            }
            Some(c) => {
                match c.is_ascii_digit() {
                    true => {
                        string = format!("{}{}", string, c);
                    }
                    false => {
                        break
                    }
                }
            }
        }
    }

    match u32::from_str_radix(string.as_str(), 10) {
        Ok(number) => { Ok(number) }
        Err(_) => { Err(format!("Not a valid number {} with radix {}", string, 10)) }
    }
}

fn unformat(content: Vec<u8>) -> Result<File, String> {
    let str_content = String::from_utf8(content).expect("Unable to convert file to string");

    if !str_content.is_ascii() { return Err(String::from("Not a valid ASCII charset")) };

    let mut char_iterator = str_content.chars().filter(|c| !c.is_whitespace());
    let version = read_version(&mut char_iterator)?;
    let background= read_background(&mut char_iterator)?;
    let ratio = read_ratio(&mut char_iterator)?;
    let regions: Vec<Region> = Vec::new();


    Ok(File{
        version,
        background,
        ratio,
        regions
    })
}

fn print_ite(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) {
   loop{
       match char_iterator.next(){
           None => {break}
           Some(c) => { print!("{}",c)}
       }
   }
}

fn read_background(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<RGBA, String> {
    match char_iterator.next() {
        None => {
            return Err(String::from("Not valid background color"))
        }
        Some(c) => {
            match c {
                '#' => {
                    read_color(&mut char_iterator)
                }
                _ => {
                    return Err(String::from("Not valid background color"))
                }
            }
        }
    }
}

fn read_ratio(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<f64, String> {
    match char_iterator.next() {
        None => {
            return Err(String::from("Invalid ratio"))
        }
        Some(c) => {
            match c {
                'r' => {
                    let num = read_number(&mut char_iterator)?;
                    let denum = read_number(&mut char_iterator)?;

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

fn read_version(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<u32, String> {
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
                'C' => { version = read_number(&mut char_iterator)?; }
                _ => { return error; }
            }
        }
    }

    return Ok(version);
}

fn read_color(char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<RGBA, String> {
    let mut string = String::new();

    let mut colors: [u8; 4] = [0; 4];
    for i in 0..4 {
        for j in 0..2 {
            match char_iterator.next() {
                None => {
                    println!("End of file");
                    break;
                }
                Some(c) => {
                    match c.is_ascii_hexdigit() {
                        true => {
                            string = format!("{}{}", string, c);
                        }
                        false => {
                            char_iterator.next_back().expect("To go backward, number is not the first char of file");
                            break;
                        }
                    }
                }
            }
        }
        colors[i] = match u8::from_str_radix(string.as_str(), 16) {
            Ok(number) => {
                number
            }
            Err(_) => { return Err(format!("Not a valid u8 {} with radix 16", string)) }
        };
        string.clear();
    }

    Ok(RGBA {
        r: colors[0],
        g: colors[1],
        b: colors[2],
        a: colors[3],
    })
}

#[derive(Debug)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug)]
struct Coord {
    w: u32,
    h: u32,
}

#[derive(Debug)]
struct Curve {
    c1: Coord,
    c2: Coord,
    p: Coord,
}

#[derive(Debug)]
struct Region {
    id: u32,
    start: Coord,
    curves: Vec<Curve>,
    color: RGBA,
}

#[derive(Debug)]
struct File {
    version: u32,
    background: RGBA,
    ratio: f64,
    regions: Vec<Region>,
}

fn main() {
    let mut bytes = fs::read("data/test1.vgcu").expect("Unable to read file");
    let a = unformat(bytes);

    match a {
        Ok(_) => {}
        Err(e) => { eprintln!("{}", e) }
    }
    // Construct a new RGB ImageBuffer with the specified width and height.
    /*let mut img: RgbImage = ImageBuffer::new(100, 100);

    img.put_pixel(50, 50, Rgb([255, 0, 0]));

    img.save("data/test1.png").unwrap();*/
}
