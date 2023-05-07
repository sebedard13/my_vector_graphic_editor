use crate::vcg_struct::*;
use crate::read_header;

pub fn read(content: Vec<u8>) -> Result<File, String> {
    let str_content = String::from_utf8(content).expect("Unable to convert file to string");

    if !str_content.is_ascii() { return Err(String::from("Not a valid ASCII charset")) };

    let mut char_iterator = str_content.chars().filter(|c| !c.is_whitespace());
    let version = read_header::read_version(&mut char_iterator)?;
    let background= read_header::read_background(&mut char_iterator)?;
    let ratio = read_header::read_ratio(&mut char_iterator)?;
    let regions: Vec<Region> = Vec::new();


    Ok(File{
        version,
        background,
        ratio,
        regions
    })
}
