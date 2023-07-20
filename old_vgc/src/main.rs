use std::fs;
use crate::render::render_w;

mod vcg_struct;
mod read_file;
mod read_header;
mod read_common;
mod read_content;
mod render;

fn main() {
    let bytes = fs::read("old_vgc/data/test1.vgcu").expect("Unable to read file");
    let a = read_file::read(bytes);

    match a {
        Err(e) => { eprintln!("{}", e) }
        Ok(file) => {
          match render_w(&file, 512){
              Ok(img) => { img.save("old_vgc/data/test1.png").expect("Able to save image");}
              Err(e) => {eprintln!("{}", e)}
          }
        }
    }
}
