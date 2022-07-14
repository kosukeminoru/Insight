use crate::db;
use crate::game;
use bevy::app::AppLabel;
use bevy::ecs::{
    event::Events,
    prelude::{FromWorld, IntoExclusiveSystem},
    schedule::{
        IntoSystemDescriptor, Schedule, ShouldRun, Stage, StageLabel, State, StateData, SystemSet,
        SystemStage,
    },
    system::Resource,
    world::World,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::mem;
use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

pub fn run() {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
    pub struct SubAppLabel;
    /* create app
       structs are added as resources which can be fetched later
       Rapier added as a plugin
       Events or functions are added as systems
       Level and graphics are created in start_up system
       Please see Bevy Documentation
    */
    //db::db::delete(String::from("gwin"));
    let mut app = App::new();
    let mut subapp = App::new();
    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    });
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    subapp
        .add_plugins_with(DefaultPlugins, |group| {
            group
                .disable::<bevy::log::LogPlugin>()
                .disable::<bevy::winit::WinitPlugin>()
        })
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(game::conf::setup_graphics)
        .add_startup_system(game::level::setup_level);
    //.add_system(game::player::player)

    /*
    app.add_plugin(bevy::hierarchy::HierarchyPlugin);
    app.add_plugin(bevy::asset::AssetPlugin);
    app.add_plugin(bevy::transform::TransformPlugin);
    app.add_plugin(bevy::render::RenderPlugin);
    app.add_plugin(bevy::sprite::SpritePlugin);
    app.add_plugins(MinimalPlugins);
    app.add_plugin(bevy::window::WindowPlugin::default());
    app.add_plugin(bevy::winit::WinitPlugin);
    app.add_plugin(bevy::input::InputPlugin);*/
    //app.init_resource::<bevy::reflect::TypeRegistryArc>();

    subapp.add_system(go);

    app.add_sub_app(SubAppLabel, subapp, move |app_world, subapp| {
        // temporarily add the app world to the render world as a resource
        mem::swap(app_world, &mut subapp.world);
        subapp.update();
        mem::swap(app_world, &mut subapp.world);
        // move the app world back, as if nothing happened.

        // Note: We apply buffers (read, Commands) after the `MainWorld` has been removed from the render app's world
        // so that in future, pipelining will be able to do this too without any code relying on it.
        // see <https://github.com/bevyengine/bevy/issues/5082>

        //app_world = &mut subapp.world;
    });
    //app.add_sub_app("string", default_subapp);

    app.run();
}

fn go() {
    println!("Hello from game");
}

/*
fn extract(app_world: &mut World, subapp: &mut App) {
    let extract = subapp
        .schedule
        .get_stage_mut::<SystemStage>(&RenderStage::Extract)
        .unwrap();

    // temporarily add the app world to the render world as a resource
    let scratch_world = app_world.remove_resource::<ScratchMainWorld>().unwrap();

    let running_world = &mut render_app.world;
    running_world.insert_resource(MainWorld(inserted_world));

    extract.run(running_world);
    // move the app world back, as if nothing happened.
    let inserted_world = running_world.remove_resource::<MainWorld>().unwrap();
    let scratch_world = std::mem::replace(app_world, inserted_world.0);
    app_world.insert_resource(ScratchMainWorld(scratch_world));

    // Note: We apply buffers (read, Commands) after the `MainWorld` has been removed from the render app's world
    // so that in future, pipelining will be able to do this too without any code relying on it.
    // see <https://github.com/bevyengine/bevy/issues/5082>
    extract.apply_buffers(running_world);
}*/

//from_world(&mut World::new());
//fn subapp_default() -> App{}

/*
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
/*
if db::db::get(String::from("gwin")) != String::from("1") {
    return false;
} else {
    return true;
}*/*/

/*    subapp.insert_resource(bevy::window::Window::new(
    bevy::window::WindowId::primary(),
    &bevy::window::WindowDescriptor {
        width: 1200.0,
        height: 600.0,
        title: "game".to_string(),
        resizable: false,

        ..Default::default()
    },
    600,
    600,
    2.0,
    None,
    ..default(),
)); */

/*
subapp.insert_non_send_resource(bevy::winit::WinitWindows::default());
    subapp.insert_resource(bevy::winit::WinitSettings { ..default() });
    subapp.insert_resource(bevy::winit::WinitSettings { ..default() });
    v */
