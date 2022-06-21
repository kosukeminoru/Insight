use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_rapier2d::prelude::*;
use crate::db;

//Function inserted into app
pub fn collisions(
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<&Velocity>,
    mut commands: Commands
) {
    for collision_event in collision_events.iter() {
        //col1, col2 are collision entities
        if let CollisionEvent::Started(col1, col2, _flag) = collision_event{
            //if has velocity
            if let Ok(Velocity{linvel: lin,angvel: _}) = query.get(*col1){
                //See if moving fast enough to be destroyed
                if calc_vel(lin) {
                  commands.entity(*col1).despawn();
                }
            }
            // if has velocity
            if let Ok(Velocity{linvel: lin,angvel: _}) = query.get(*col2){
                //See if moving fast enough to be destroyed
                if calc_vel(lin) {
                  commands.entity(*col2).despawn();
                }
            }
        }
    }
}

//Takes in a Vec2(x,y velocity) and returns whether its moving fast enough to destroy
fn calc_vel(vel: &Vec2)-> bool{
    let velabs = vel.abs();
    if velabs[0]+velabs[1] > 100.0 {
        return true;
    }
    return false;
}

// Determines whether object is moving from Velocity 
pub fn moving(vel: &Velocity) -> bool{
    if vel.linvel.abs()[0]+vel.linvel.abs()[1]>0.02{
        return true;
    }
    else {
        return false;
    }
}

pub fn win(
    query: Query<&Restitution>,
    mut exit: EventWriter<AppExit>
){
    let mut win: bool = true;
    for object in query.iter() {
        if object.coefficient == 0.5 {
            win = false;
        }
    }
    if win {
        db::db::put(String::from("gwin"), String::from("1"));
        exit.send(AppExit);
    }
}