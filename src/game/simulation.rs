use crate::game;
use bevy::app::App;
use bevy::app::AppLabel;
use bevy::ecs::event::Events;
use bevy::input::mouse::MouseMotion;
use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy_rapier2d::pipeline::ContactForceEvent;
use bevy_rapier2d::prelude::*;
use std::mem;
use std::sync::Once;
use std::time::Instant;

use crate::subapps;
use subapps::renderer::camera::pan_orbit;
use subapps::renderer::geometry::my_plane;

static START: Once = Once::new();
static STARTER: Once = Once::new();
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Bench {
    start_time: Instant,
    loops: u64,
    end_time: Instant,
}

pub fn run() {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
    pub struct SubGame;
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
    pub struct SubAppSwap;

    pub struct DaCount {
        boo: bool,
    };
    /* create app
       structs are added as resources which can be fetched later
       Rapier added as a plugin
       Events or functions are added as systems
       Level and graphics are created in start_up system
       Please see Bevy Documentation
    */

    let mut app = App::new();
    /*
    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    });*/
    /*
    let _myBench = Bench {
        start_time: Instant::now(),
        loops: 0,
        end_time: Instant::now(),
    };*/
    //Mouse + graphics
    app.add_plugins(MinimalPlugins);
    app.add_plugin(bevy::window::WindowPlugin::default());
    app.add_plugin(bevy::winit::WinitPlugin);
    app.insert_resource(MyApp);
    app.insert_resource(DaCount { boo: true });
    let subapp_swap = App::new();

    app.add_sub_app(SubAppSwap, subapp_swap, move |app_world, subapp_swap| {
        mem::swap(app_world, &mut subapp_swap.world);
        if subapp_swap.world.get_resource::<DaCount>().unwrap().boo {
            let subapp_game = App::new();
            subapp_swap.add_sub_app(SubGame, subapp_game, move |app_world, subapp_game| {
                mem::swap(app_world, &mut subapp_game.world);
                if subapp_game.world.get_resource::<DaCount>().unwrap().boo {
                    let mut count = subapp_game.world.get_resource_mut::<DaCount>().unwrap();
                    count.boo = false;
                    let playing = subapp_game.world.get_resource::<MyApp>().unwrap();
                    subapp_game.add_plugin(*playing);
                }
                subapp_game.update();
                mem::swap(app_world, &mut subapp_game.world);
            });
        }
        subapp_swap.update();
        mem::swap(app_world, &mut subapp_swap.world);
        //app_world = &mut subapp.world;
    });

    //app.insert_resource(myBench);
    //app.add_system(bmark);
    app.run();
}
/*
fn go() {
    println!("hello");
}
fn bmark(mut Res: ResMut<Bench>) {
    Res.loops = Res.loops + 1;
    Res.end_time = Instant::now();
    println!("{:?}", Res);
}*/

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MyApp;
impl Plugin for MyApp {
    fn build(&self, app: &mut App) {
        app.add_plugins_with(DefaultPlugins, |group| {
            group
                .disable::<bevy::log::LogPlugin>()
                .disable::<bevy::window::WindowPlugin>()
                .disable::<bevy::winit::WinitPlugin>()
        })
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default()) // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.
        .add_startup_system(my_plane::setup_plane)
        .add_startup_system(pan_orbit::spawn_camera)
        .add_system(pan_orbit::pan_orbit_camera)
        .add_system(my_plane::add_block);
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MyApp2;
impl Plugin for MyApp2 {
    fn build(&self, app: &mut App) {
        app.add_plugins_with(DefaultPlugins, |group| {
            group
                .disable::<bevy::log::LogPlugin>()
                .disable::<bevy::window::WindowPlugin>()
                .disable::<bevy::winit::WinitPlugin>()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(game::conf::setup_graphics)
        .add_startup_system(game::level::setup_level)
        .add_startup_system(add)
        .add_system(game::player::player)
        .add_system(game::collisions::win)
        .add_system(game::collisions::collisions);
    }
}

fn add(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.insert_resource(game::conf::rap_conf());
    commands.insert_resource(game::conf::window_conf());
    commands.insert_resource(game::player::Attempt {
        first: None,
        second: None,
        third: None,
        fourth: None,
        fifth: None,
    });
    commands.insert_resource(game::player::Donde { shot: 1 });
    commands.insert_resource(game::player::MouseResource {
        first: Vec2::new(0., 0.),
        second: Vec2::new(0., 0.),
    });
    commands.insert_resource(game::player::IsReleased {
        b: game::player::BoolReleased::Yes,
    });
}
