use std::cmp::{max, min};
use image::{Pixel, Rgba, RgbaImage};

use crate::vcg_struct::{Coord, Curve, File};

#[allow(dead_code)]
pub fn render_h(file: &File, h: u32) -> Result<RgbaImage, String> {
    let w = ((h as f64) * file.ratio) as u32;
    return render(&file, w, h);
}

#[allow(dead_code)]
pub fn render_w(file: &File, w: u32) -> Result<RgbaImage, String> {
    let h = ((w as f64)*(1.0/file.ratio)) as u32;
    return render(&file, w,h);
}



fn render(file: &File, w:u32, h:u32)->Result<RgbaImage, String>{
    let mut image = RgbaImage::new(w,h);

    for i_h in 0..h {
        for i_w in 0..w {
            for region in 0..file.regions.len() {
                if file.regions[region].start.w < (i_w as f32)/(w as f32) {
                    //for curve in 0..file.regions[region].curves.len() {
                    for curve in 0..file.regions[region].curves.len() {
                        let start_coord = match curve{
                            0 => &file.regions[region].start,
                            _ => &file.regions[region].curves[curve-1].c2
                        };

                        if right_of_curve(&file.regions[region].curves[curve], start_coord, (i_w as f32)/(w as f32), (i_h as f32)/(w as f32)) {
                            let pix = Rgba::from([file.regions[region].color.r, file.regions[region].color.g, file.regions[region].color.b, file.regions[region].color.a]);
                            image.put_pixel(i_w, i_h, pix);
                        }
                    }

                }
            }
            
        }
    }

    Ok(image)
}

fn float_loop(start: f64, threshold: f64, step_size: f64) -> impl Iterator<Item=f64> {
    std::iter::successors(Some(start), move |&prev| {
        let next = prev + step_size;
        (next < threshold).then_some(next)
    })
}

fn right_of_curve(cur: &Curve, st: &Coord, w: f32, h: f32) -> bool {
    let min_range = h-0.1;
    let max_range = h+0.1;
    for t in float_loop(0.0, 1.0, 0.001) {
        let find_coord = cur.evaluate(t as f32, st);

        if find_coord.h >= min_range && find_coord.h <= max_range {
            if find_coord.w <=w {
                return true;
            }
            else { return false; }

        }
    }
    return false;
}

