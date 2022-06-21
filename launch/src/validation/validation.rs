use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_rapier2d::prelude::*;
use bevy::winit;
use crate::game::player::Attempt;
use crate::game;
use crate::db;

//App for Validation
pub fn run(attempt: game::player::Attempt) -> bool{
    db::db::delete(String::from("vwin"));
    App::new()
        .insert_resource(winit::WinitSettings {return_from_run: true, ..default()})
        .insert_resource(attempt)
        .insert_resource(game::conf::rap_conf())
        .insert_resource(game::conf::window_conf())
        .insert_resource(game::player::Donde{shot: 1})
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_startup_system(game::conf::setup_graphics)
        .add_startup_system(game::level::setup_level)
        .add_system(insert)
        .add_system(vwin)
        .add_system(game::collisions::collisions)
        .run();
    if db::db::get(String::from("vwin")) != String::from("1"){
    println!("lose");
        return false;
    }
    else{
        return true;
    }
}

pub fn insert(
    mut commands: Commands,
    velocity: Query<&Velocity>, 
    mut number: ResMut<game::player::Donde>, 
    attempt: Res<Attempt>,
    mut exit: EventWriter<AppExit>
) {
    let mut proceed: bool = true;
    for vel in velocity.iter(){
        if game::collisions::moving(vel) == true {
            proceed = false;
        }
    }
    if proceed == true{ 
        match number.shot{                
            1 => match attempt.first{
              
               Some(game::player::MouseResource{first: _, second: _}) => game::player::shoot_player(&mut commands, &attempt.first.as_ref().unwrap()),
               None => println!("none")
            },
            2 => match attempt.second{
                
               Some(game::player::MouseResource{first: _, second: _}) =>  game::player::shoot_player(&mut commands, &attempt.second.as_ref().unwrap()),
               None => println!("none")
            },
            3 => match attempt.third{
               
               Some(game::player::MouseResource{first: _, second: _}) =>  game::player::shoot_player(&mut commands, &attempt.third.as_ref().unwrap()),
               None => println!("none")
            },
            4 => match attempt.fourth{
               
               Some(game::player::MouseResource{first: _, second: _}) =>  game::player::shoot_player(&mut commands, &attempt.fourth.as_ref().unwrap()),
               None => println!("none")
            },
            5 => match attempt.fifth{
           
               Some(game::player::MouseResource{first: _, second: _}) =>  game::player::shoot_player(&mut commands, &attempt.fifth.as_ref().unwrap()),
               None => println!("none")
            },
            6 => exit.send(AppExit),
            _ => ()
        }
        number.shot += 1;
    }      
    
}

pub fn vwin(
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
        db::db::put(String::from("vwin"), String::from("1"));
        exit.send(AppExit);
    }
}
