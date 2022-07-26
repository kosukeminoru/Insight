use super::constants::WX;
use super::constants::WY;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

//Vector of objects which is an enum for different objects
pub struct Level {
    pub objects: Vec<Object>,
}
//Default level
impl Default for Level {
    fn default() -> Level {
        Level {
            objects: vec![
                Object::Segment(Segment::new(
                    [-1.0 * WX / 2.0, -1.0 * WY / 2.0],
                    [WX / 2.0, -1.0 * WY / 2.0],
                    "ground".to_string(),
                )),
                Object::Rectangle(Rectangle::new(
                    WX / 4.0,
                    -1.0 * WY / 2.0 + 40.0,
                    5.0,
                    20.0,
                    "wood".to_string(),
                )),
                Object::Rectangle(Rectangle::new(
                    WX / 4.0 + 15.0,
                    -1.0 * WY / 2.0 + 45.0,
                    20.0,
                    5.0,
                    "wood".to_string(),
                )),
                Object::Rectangle(Rectangle::new(
                    WX / 4.0 + 30.0,
                    -1.0 * WY / 2.0 + 20.0,
                    5.0,
                    20.0,
                    "wood".to_string(),
                )),
                Object::Circle(Circle::new(
                    WX / 4.0 + 15.0,
                    -1.0 * WY / 2.0 + 5.0,
                    5.0,
                    "enemy".to_string(),
                )),
                Object::Rectangle(Rectangle::new(
                    WX / 4.0 + 41.0,
                    -1.0 * WY / 2.0 + 20.0,
                    5.0,
                    20.0,
                    "wood".to_string(),
                )),
                Object::Rectangle(Rectangle::new(
                    WX / 4.0 + 56.0,
                    -1.0 * WY / 2.0 + 45.0,
                    20.0,
                    5.0,
                    "wood".to_string(),
                )),
                Object::Rectangle(Rectangle::new(
                    WX / 4.0 + 71.0,
                    -1.0 * WY / 2.0 + 20.0,
                    5.0,
                    20.0,
                    "wood".to_string(),
                )),
                Object::Circle(Circle::new(
                    WX / 4.0 + 56.0,
                    -1.0 * WY / 2.0 + 5.0,
                    5.0,
                    "hello".to_string(),
                )),
            ],
        }
    }
}

//Types of objects in a level
pub enum Object {
    Circle(Circle),
    Rectangle(Rectangle),
    Segment(Segment),
}

//Circle Object
pub struct Circle {
    pub x_coor: f32,
    pub y_coor: f32,
    pub radius: f32,
    pub kind: String,
}
// probably a better way of doing this rip
impl Circle {
    fn new(x: f32, y: f32, r: f32, k: String) -> Circle {
        Circle {
            x_coor: x,
            y_coor: y,
            radius: r,
            kind: k,
        }
    }
}
//Rectangle struct
pub struct Rectangle {
    pub x_coor: f32,
    pub y_coor: f32,
    pub height: f32,
    pub width: f32,
    pub kind: String,
}
impl Rectangle {
    fn new(x: f32, y: f32, w: f32, h: f32, k: String) -> Rectangle {
        Rectangle {
            x_coor: x,
            y_coor: y,
            height: h,
            width: w,
            kind: k,
        }
    }
}
// Segment Struct
pub struct Segment {
    pub p1: Vec2,
    pub p2: Vec2,
    pub kind: String,
}
impl Segment {
    fn new(pone: [f32; 2], ptwo: [f32; 2], k: String) -> Segment {
        Segment {
            p1: Vec2::new(pone[0], pone[1]),
            p2: Vec2::new(ptwo[0], ptwo[1]),
            kind: k,
        }
    }
}

pub fn setup_level(mut commands: Commands) {
    let new_level = Level {
        ..Default::default()
    };
    //iterates through level struct
    for i in new_level.objects {
        //if object is circle spawn circle with dimensions
        //insert a velocity and active event so it can be queried later
        if let Object::Circle(ccl) = i {
            commands
                .spawn()
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(ccl.radius))
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.25, 0.75),
                        custom_size: Some(Vec2::new(ccl.radius * 2.0, ccl.radius * 2.0)),
                        ..default()
                    },
                    transform: Transform {
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Restitution::coefficient(0.5))
                .insert(Velocity { ..default() })
                .insert_bundle(TransformBundle::from(Transform::from_xyz(
                    ccl.x_coor, ccl.y_coor, 0.0,
                )));
        }
        //if object is rectangle spawn rect with dimensions
        //insert a velocity and active event so it can be queried later
        else if let Object::Rectangle(rct) = i {
            commands
                .spawn()
                .insert(RigidBody::Dynamic)
                //created from midpoint height = 2x height width = 2x width
                // |---|
                // | . | Dot is midpoint 1.5 dashes = width, 1.5 | = height
                // |---|
                .insert(Collider::cuboid(rct.width, rct.height))
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.5, 0.5, 0.5),
                        custom_size: Some(Vec2::new(2.0 * rct.width, 2.0 * rct.height)),
                        ..default()
                    },
                    transform: Transform {
                        ..Default::default()
                    },
                    ..default()
                })
                .insert(Restitution::coefficient(0.7))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Velocity { ..default() })
                .insert_bundle(TransformBundle::from(Transform::from_xyz(
                    rct.x_coor, rct.y_coor, 0.0,
                )));
        }
        //if object is Segment spawn segment with dimensions
        //insert a velocity and active event so it can be queried later
        else if let Object::Segment(sg) = i {
            commands
                .spawn()
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Velocity { ..default() })
                .insert(Collider::segment(sg.p1, sg.p2));
        }
    }
}
