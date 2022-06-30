use crate::db;
use crate::game;
use bevy::prelude::*;
use bevy::winit;
use bevy_rapier2d::prelude::*;

pub fn run() -> bool {
    /* create app
       structs are added as resources which can be fetched later
       Rapier added as a plugin
       Events or functions are added as systems
       Level and graphics are created in start_up system
       Please see Bevy Documentation
    */
    db::db::delete(String::from("gwin"));
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(winit::WinitSettings {
            return_from_run: true,
            ..default()
        })
        .insert_resource(game::conf::rap_conf())
        .insert_resource(game::conf::window_conf())
        .insert_resource(game::player::Attempt {
            first: None,
            second: None,
            third: None,
            fourth: None,
            fifth: None,
        })
        .insert_resource(game::player::Donde { shot: 1 })
        .insert_resource(game::player::MouseResource {
            first: Vec2::new(0., 0.),
            second: Vec2::new(0., 0.),
        })
        .insert_resource(game::player::IsReleased {
            b: game::player::BoolReleased::Yes,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(game::conf::setup_graphics)
        .add_startup_system(game::level::setup_level)
        .add_system(game::player::player)
        .add_system(game::collisions::win)
        .add_system(game::collisions::collisions)
        .run();
    if db::db::get(String::from("gwin")) != String::from("1") {
        return false;
    } else {
        return true;
    }
}
