use crate::{read_content, read_header};
use crate::vcg_struct::*;

pub fn read(content: Vec<u8>) -> Result<File, String> {
    let str_content = String::from_utf8(content).expect("Unable to convert file to string");

    if !str_content.is_ascii() { return Err(String::from("Not a valid ASCII charset")) };

    let char_iterator = str_content.chars();
    let mut char_iterator = char_iterator.filter(|c| !c.is_whitespace());


    match try_read_file(&mut char_iterator) {
        Ok(ok) => { Ok(ok) }
        Err(e) => {
            let mut str_file = String::new();
            loop {
                match char_iterator.next() {
                    None => { break }
                    Some(c) => { str_file = format!("{}{}", str_file, c) }
                }
            }
            str_file = format!("Could not read vcg content\n{}\n\n===> {:>5}", e, str_file);
            Err(str_file)
        }
    }
}

fn try_read_file(mut char_iterator: &mut impl DoubleEndedIterator<Item=char>) -> Result<File, String> {
    let version = read_header::read_version(&mut char_iterator)?;
    let background = read_header::read_background(&mut char_iterator)?;
    let ratio = read_header::read_ratio(&mut char_iterator)?;
    let regions: Vec<Region> = read_content::read_regions(&mut char_iterator)?;


    Ok(File {
        version,
        background,
        ratio,
        regions,
    })
}
