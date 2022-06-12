use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::conf as conf;
use crate::game::level as level;

pub fn run(){

/* create app
    add physics config
    add window config
    default Bevy settings
    Add physics engine
    Add debugger (Shows what physics engine sees)
    add graphics
    add level 
*/
    App::new()
        .insert_resource(conf::rap_conf())
        .insert_resource(conf::window_conf())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())

        .add_startup_system(conf::setup_graphics)
        .add_startup_system(level::setup_physics)
        .add_system(print_ball_altitude)
        .run();
}





fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}