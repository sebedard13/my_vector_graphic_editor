use image::{RgbaImage};
use crate::vcg_struct::File;

#[allow(dead_code)]
pub fn render_h(file: &File, h:u32)->Result<RgbaImage, String>{

    let w = ((h as f64)*file.ratio) as u32;
    return render(&file, w,h);
}

#[allow(dead_code)]
pub fn render_w(file: &File, w:u32)->Result<RgbaImage, String>{
    let h = ((w as f64)*(1.0/file.ratio)) as u32;
    return render(&file, w,h);
}



fn render(file: &File, w:u32, h:u32)->Result<RgbaImage, String>{
    let mut image = RgbaImage::new(w,h);




    Err(format!("Not able to generate image"))
}