use crate::insight;
use crate::insight::constants::WX;
use crate::insight::constants::WY;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

//which # shot we are on
pub struct Donde {
    pub shot: u8,
}
// Win attempt
#[derive(Serialize, Deserialize, Debug)]
pub struct Attempt {
    pub first: Option<MouseResource>,
    pub second: Option<MouseResource>,
    pub third: Option<MouseResource>,
    pub fourth: Option<MouseResource>,
    pub fifth: Option<MouseResource>,
}
//Whether Player released or dragging yes- released
pub enum BoolReleased {
    Yes,
    No,
}
//So can be inserted as a resource into app. Probably a better way of doing this
pub struct IsReleased {
    pub b: BoolReleased,
}

//First click and second release locations to spawn a player object
//derive used for serializing and deserializing struct
#[derive(Serialize, Deserialize, Debug)]
pub struct MouseResource {
    pub first: Vec2,
    pub second: Vec2,
}

// tracks mouse location on first click shoots from location on release
pub fn player(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    mut resource: ResMut<MouseResource>,
    mut release: ResMut<IsReleased>,
    velocity: Query<&Velocity>,
    mut number: ResMut<Donde>,
    mut attempt: ResMut<Attempt>,
    mut exit: EventWriter<AppExit>,
) {
    //Proceed is to ensure no objects are moving
    let mut proceed: bool = true;
    //Iterates through velocities if moving, proceed = false
    for vel in velocity.iter() {
        if insight::collisions::moving(vel) == true {
            proceed = false;
        }
    }
    if proceed {
        match number.shot {
            6 => exit.send(AppExit),
            _ => (),
        }
        let window = windows.get_primary_mut().unwrap();
        //if mouse is released and mouse left clicks, store click into MouseResource and change value of BoolReleased
        if let BoolReleased::Yes = release.b {
            if btn.pressed(MouseButton::Left) {
                resource.first = window.cursor_position().unwrap();
                release.b = BoolReleased::No;
            }
        }
        //If released from dragging, shoot the player store location and change BoolReleased
        else {
            release.b = BoolReleased::Yes;
            //if cursor is detectable store location and shoot
            if let Some(a) = window.cursor_position() {
                resource.second = a;
                // passing MouseResource instead of ResMut
                let shot: MouseResource = MouseResource {
                    first: resource.first,
                    second: resource.second,
                };
                shoot_player(&mut commands, &shot);
                //store shot in db
                match number.shot {
                    1 => attempt.first = Some(shot),
                    2 => attempt.second = Some(shot),
                    3 => attempt.third = Some(shot),
                    4 => attempt.fourth = Some(shot),
                    5 => attempt.fifth = Some(shot),
                    //6 => restart,
                    _ => println!("what fuck there is error"),
                };
                number.shot += 1;
                shoot_player_store(&attempt);
            }
        }
    }
}

pub fn shoot_player(commands: &mut Commands, resource: &MouseResource) {
    //Calculate distance between two points
    println!("enter4");
    let x = resource.second.to_array()[0] - WX / 2.0;
    let y = resource.second.to_array()[1] - WY / 2.0;
    let mut distx = resource.first.to_array()[0] - resource.second[0];
    let mut disty = resource.first.to_array()[1] - resource.second[1];
    //if one of the distances is greater than 350.0 then it is 0. (change later)
    let max = distx.abs().max(disty.abs());
    if max > 350.0 {
        distx = 0.0;
        disty = 0.0;
    }
    //Create a player object with the parameters (Change later)
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform {
                ..Default::default()
            },
            ..default()
        })
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            linvel: Vec2::new(distx, disty),
            angvel: 0.0,
        })
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(x, y, 0.0)));
}

// stores Mouse Resource (Change later needs key value )
fn shoot_player_store(_attempt: &Attempt) {

    //let serialized = serde_json::to_string(attempt).unwrap();
    //db::db::put(String::from("tempattempt"), serialized);
}
