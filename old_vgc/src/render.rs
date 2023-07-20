use image::{Pixel, Rgba, RgbaImage};
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

    for i_h in 0..h {
        for i_w in 0..w {
            if file.regions[0].start.w < (i_w as f32)/(w as f32) {
                let pix =  Rgba::from([file.regions[0].color.r, file.regions[0].color.g, file.regions[0].color.b, file.regions[0].color.a]);
               image.put_pixel(i_w, i_h,pix);
            }
        }
    }

    Ok(image)
}