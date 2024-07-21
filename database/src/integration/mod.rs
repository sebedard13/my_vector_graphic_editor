use common::{
    types::{Coord, Length2d},
    Rgba,
};

use crate::scene::{shape::Shape, Scene};

#[test]
fn test_shape_insert() {
    let mut db = Scene::new();
    let shape = Shape::new();
    let inserted_id = db.shape_insert(shape);
    let shape = db.shape_select(inserted_id).unwrap();
    assert_eq!(shape.id, inserted_id);
    assert_eq!(shape.color, Rgba::new(0, 0, 0, 0));
}

#[test]
fn test_shape_select() {
    let mut db = Scene::new();

    let selected_id = db.shape_insert(Shape::new_circle(
        Coord::new(0.0, 0.0),
        Length2d::new(1.0, 1.0),
    ));
    db.shape_insert(Shape::new_circle(
        Coord::new(0.2, 0.2),
        Length2d::new(0.7, 0.7),
    ));

    let result = db.shape_select_contains_mut(&Coord::new(-0.3, -0.3));
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().id, selected_id);
}

#[test]
fn test_shape_select_with_move() {
    let mut db = Scene::new();

    let postition_id = db.shape_insert(Shape::new_circle(
        Coord::new(0.0, 0.0),
        Length2d::new(1.0, 1.0),
    ));
    for _ in 0..10 {
        db.shape_insert(Shape::new_circle(
            Coord::new(0.2, 0.2),
            Length2d::new(0.7, 0.7),
        ));
    }
    let object_id = db.shape_insert(Shape::new_circle(
        Coord::new(0.0, 0.0),
        Length2d::new(1.0, 1.0),
    ));

    let result = db.shape_select_contains_mut(&Coord::new(-0.3, -0.3));
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().id, postition_id);

    db.layer_move_at(object_id, postition_id);

    let result = db.shape_select_contains_mut(&Coord::new(-0.3, -0.3));
    assert_eq!(result.is_some(), true);
    assert_eq!(result.unwrap().id, object_id);
}
