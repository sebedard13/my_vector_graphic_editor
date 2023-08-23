use iced::{
    widget::canvas::{Fill, Frame, Path, Stroke},
    Color, Point,
};

use crate::scene::{point_in_radius, Scene};

pub struct SelectedShape {
    index_selected_coord: usize,
}

impl Default for SelectedShape {
    fn default() -> Self {
        Self {
            index_selected_coord: 9999,
        }
    }
}

pub fn change_hover(scene: &mut Scene, cursor_position: Point) {
    let coords = scene.vgc_data.list_coord();
    for coord in coords {
        if point_in_radius(
            &scene.camera.project(cursor_position),
            &Point::new(coord.coord.x, coord.coord.y),
            scene.camera.fixed_length(12.0),
        ) {
            scene.selected_shape.index_selected_coord = coord.i;
            return;
        }
    }

    scene.selected_shape.index_selected_coord = 9999;
}

pub fn draw(scene: &Scene, frame: &mut Frame) {
    // Render points
    let coords = scene.vgc_data.list_coord();
    for coord in coords {
        let color = match scene.selected_shape.index_selected_coord == coord.i {
            true => Color::from_rgb8(0x0E, 0x90, 0xAA),
            false => Color::from_rgb8(0x3A, 0xD1, 0xEF),
        };

        let center = Point::new(
            coord.coord.x,
            coord.coord.y * 1.0 / scene.vgc_data.ratio as f32,
        );
        frame.fill(
            &Path::circle(center, scene.camera.fixed_length(5.0)),
            Fill::from(color),
        );
    }

    // Render lines between control points and points
    let selected_shape = 0;

    let p_coords = scene.vgc_data.get_p_of_shape(selected_shape);
    let cp_coords = scene.vgc_data.get_cp_of_shape(selected_shape);

    for cp in cp_coords {
        match cp {
            vgc::CoordType::Cp0(curve_index, coord) => {
                let from = Point::new(
                    p_coords[curve_index].x,
                    p_coords[curve_index].y * 1.0 / scene.vgc_data.ratio as f32,
                );

                let to = Point::new(coord.x, coord.y * 1.0 / scene.vgc_data.ratio as f32);

                let stroke = Stroke::default()
                    .with_width(2.0)
                    .with_color(Color::from_rgb8(0x3A, 0xD1, 0xEF));
                frame.stroke(&Path::line(from, to), stroke);
            }
            vgc::CoordType::Cp1(curve_index, coord) => {
                let from = Point::new(
                    p_coords[curve_index + 1].x,
                    p_coords[curve_index + 1].y * 1.0 / scene.vgc_data.ratio as f32,
                );

                let to = Point::new(coord.x, coord.y * 1.0 / scene.vgc_data.ratio as f32);
                let stroke = Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgba8(0x3A, 0xD1, 0xEF,0.5));
                frame.stroke(&Path::line(from, to), stroke);
            }
            _ => {}
        }
    }

    // Render path stroke overlay
    let coords = scene.vgc_data.get_coords_of_shape(selected_shape);
    let path = Path::new(|p| {
        p.move_to(Point::new(
            coords[0].x,
            coords[0].y * 1.0 / scene.vgc_data.ratio as f32,
        ));

        for i in 0..((coords.len() - 1) / 3) {
            let index = i*3+1;
            p.bezier_curve_to(
                Point::new(
                    coords[index].x,
                    coords[index].y * 1.0 / scene.vgc_data.ratio as f32,
                ),
                Point::new(
                    coords[index+1].x,
                    coords[index+1].y * 1.0 / scene.vgc_data.ratio as f32,
                )
                ,Point::new(
                    coords[index+2].x,
                    coords[index+2].y * 1.0 / scene.vgc_data.ratio as f32,
                )
            );
        }

    });

    let stroke = Stroke::default()
    .with_width(1.0)
    .with_color(Color::from_rgba8(0x3A, 0xD1, 0xEF,0.5));
    frame.stroke(&path, stroke);
}

pub fn draw_closest_pt(scene: &Scene, frame: &mut Frame, pos: Point)  {
    let shape_index = 0;
    let pos = scene.camera.project(pos);
    let coord = scene.vgc_data.get_closest_coord_on_shape(shape_index, pos.x, pos.y);

    let color = Color::from_rgb8(0x0E, 0x90, 0xAA);
   
    let center = Point::new(
        coord.x,
        coord.y * 1.0 / scene.vgc_data.ratio as f32,
    );
    frame.fill(
        &Path::circle(center, scene.camera.fixed_length(3.0)),
        Fill::from(color),
    );
}

