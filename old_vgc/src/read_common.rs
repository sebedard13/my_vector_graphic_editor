use crate::vcg_struct::{Coord, RGBA};

pub fn read_number(char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<u32, String> {
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

pub fn read_coord(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>)-> Result<Coord, String>{
    let mut w = read_number(&mut char_iterator)? as f32;
    let mut h = read_number(&mut char_iterator)? as f32;

    loop {
        match w >= 1.0 {
            true => {
                w = w / 10.0
            }
            false => { break }
        }
    }

    loop {
        match h >= 1.0 {
            true => {
                h = h / 10.0
            }
            false => { break }
        }
    }

    Ok(Coord {
        w: w as f32,
        h: h as f32,
    })
}


#[allow(dead_code)]
pub fn print_ite(char_iterator: &mut impl DoubleEndedIterator<Item=char>) {
   loop{
       match char_iterator.next(){
           None => {break}
           Some(c) => { print!("{}",c)}
       }
   }
}

pub fn read_color(char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<RGBA, String> {
    let mut string = String::new();

    let mut colors: [u8; 4] = [0; 4];
    for i in 0..4 {
        for _j in 0..2 {
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

#[macro_export] macro_rules! try_append {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(format!("{}\n{}", $msg, err)),
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::read_common::read_coord;

    #[test]
    pub fn coord_valid() {
        let mut iter = "50,20".chars();

        let result = read_coord(&mut iter).unwrap();

        assert_eq!(result.w, 0.50);
        assert_eq!(result.h, 0.20);
    }

    #[test]
    pub fn coord_long_valid(){
        let mut iter = "502000,1".chars();

        let result = read_coord(&mut iter).unwrap();

        assert_eq!(result.w, 0.502000);
        assert_eq!(result.h, 0.1);
    }

    #[test]
    pub fn coord_invalid(){
        let mut iter = "5020001".chars();

        let result = read_coord(&mut iter);

        assert!(result.is_err())
    }
}