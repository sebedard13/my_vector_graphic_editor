use common::types::Coord;
use common::types::Length2d;
use common::types::Rect;
use common::types::ScreenCoord;
use common::types::ScreenLength2d;

use crate::generate_child_methods;
use crate::SceneClient;

generate_child_methods!(camera,
    (camera_get_zoom, get_zoom(), f32),
    (camera_set_pixel_region, set_pixel_region(width: f32, height: f32)),
    (camera_set_rotation, set_rotation(rotation: f32)),
    (camera_get_rotation, get_rotation(), f32),
    (camera_set_reflect_x, set_reflect_x(reflect_x: bool)),
    (camera_get_reflect_x, get_reflect_x(), bool),
    (camera_set_reflect_y, set_reflect_y(reflect_y: bool)),
    (camera_get_reflect_y, get_reflect_y(), bool),
    (camera_get_base_scale, get_base_scale(), ScreenLength2d),
    (camera_region, region(), Rect),
    (camera_project, project(position: ScreenCoord), Coord),
    (camera_unproject, unproject(position: Coord), ScreenCoord),
    (camera_unproject_to_canvas, unproject_to_canvas(position: Coord), ScreenCoord),
    (camera_transform_to_length2d, transform_to_length2d(movement: ScreenLength2d), Length2d),
    (camera_zoom_at, zoom_at(movement: f32, coord: ScreenCoord)),
    (camera_pan_by, pan_by(movement: ScreenLength2d)),
    (camera_home, home())
);
