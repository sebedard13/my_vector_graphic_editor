use std::fs;
use image::{ImageBuffer, Rgb, RgbImage};


fn uncompress(){

}

fn unformat(content:  Vec<u8>) -> Vec<u8>{
    let mut str_content = String::from_utf8(content).expect("Unable to convert file to string");
    str_content.retain(|c| !c.is_whitespace());

    return str_content.as_bytes().to_vec();
}

fn main() {
    let mut bytes = fs::read("data/test1.vgcu").expect("Unable to read file");
    bytes = unformat(bytes);


    let str = String::from_utf8( bytes).expect("Unable to convert file to string");

    println!("{}", str.as_str());

    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new(100, 100);

    img.put_pixel(50, 50, Rgb([255, 0, 0]));

    img.save("data/test1.png").unwrap();
}
