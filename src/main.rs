use std::fs;

mod vcg_struct;
mod read_file;
mod read_header;
mod read_common;
mod read_content;

fn main() {
    let bytes = fs::read("data/test1.vgcu").expect("Unable to read file");
    let a = read_file::read(bytes);

    match a {
        Ok(file) => { println!("{:#?}", file) }
        Err(e) => { eprintln!("{}", e) }
    }
    // Construct a new RGB ImageBuffer with the specified width and height.
    /*let mut img: RgbImage = ImageBuffer::new(100, 100);

    img.put_pixel(50, 50, Rgb([255, 0, 0]));

    img.save("data/test1.png").unwrap();*/
}
