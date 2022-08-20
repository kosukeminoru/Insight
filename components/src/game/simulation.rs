use crate::game;
use crate::game::network;
use crate::game::player;
use crate::struc::NetworkInfo;
use crate::struc::Request;
//use crate::subapps;
use bevy::app::App;
use bevy_ggrs::{GGRSPlugin, SessionType};
//use bevy::app::AppLabel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
//use subapps::renderer::camera::pan_orbit;
//use subapps::renderer::geometry::my_plane;

const FPS: usize = 60;
const ROLLBACK_DEFAULT: &str = "rollback_default";

pub fn run(reciever: Receiver<NetworkInfo>, sender: Sender<Request>) {
    /* create app
       structs are added as resources which can be fetched later
       Rapier added as a plugin
       Events or functions are added as systems
       Level and graphics are created in start_up system
       Please see Bevy Documentation
    */
    let sess_build = network::create_ggrs_session().unwrap();
    let sess = network::start_ggrs_session(sess_build).unwrap();
    let mut app = App::new();
    GGRSPlugin::<network::GGRSConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS)
        .with_input_system(network::input)
        .register_rollback_type::<Transform>()
        .register_rollback_type::<Velocity>()
        .register_rollback_type::<network::FrameCount>()
        .with_rollback_schedule(
            Schedule::default().with_stage(
                ROLLBACK_DEFAULT,
                SystemStage::parallel()
                    .with_system(network::move_player)
                    //.with_system(movement::move_cube_system)
                    .with_system(network::increase_frame_system),
                //.with_system(pcg_city::buildings::spawn_buildings), //i think spawning can't be done in rollback
            ),
        )
        // make it happen in the bevy app
        .build(&mut app);

    let info;
    match reciever.try_recv() {
        Ok(i) => info = i,
        Err(_) => info = NetworkInfo::default(),
    }
    app.insert_resource(info);

    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    });
    app.add_plugin(MyApp)
        .add_startup_system(network::setup_system)
        // add your GGRS session
        .insert_resource(sess)
        .insert_resource(SessionType::P2PSession)
        // register a resource that will be rolled back
        .insert_resource(network::FrameCount { frame: 0 });
    app.insert_resource(reciever);
    app.insert_resource(sender);
    app.add_system(update_accounts);

    app.run();
}

pub struct MyApp;
impl Plugin for MyApp {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(game::conf::setup_graphics)
            .add_startup_system(game::level::setup_level)
            .add_startup_system(add)
            //.add_system(game::player::player)
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
    commands.insert_resource(player::Donde { shot: 1 });
    commands.insert_resource(player::MouseResource {
        first: Vec2::new(0., 0.),
        second: Vec2::new(0., 0.),
    });
    commands.insert_resource(player::IsReleased {
        b: game::player::BoolReleased::Yes,
    });
}

/*
pub struct MyApp;
impl Plugin for MyApp {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_mod_picking::DefaultPickingPlugins)
            .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default()) // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.
            .add_startup_system(my_plane::setup_plane)
            .add_startup_system(pan_orbit::spawn_camera)
            .add_system(pan_orbit::pan_orbit_camera)
            .add_system(my_plane::add_block);
    }
}*/

fn update_accounts(mut commands: Commands, r: Res<Receiver<NetworkInfo>>) {
    if let Result::Ok(accounts) = r.try_recv() {
        commands.insert_resource(accounts);
    }
}

/* #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
pub struct SubAppLabel; */

/*
//Mouse + graphics
app.add_plugins(MinimalPlugins);
app.add_plugin(bevy::window::WindowPlugin::default());
app.add_plugin(bevy::winit::WinitPlugin);

let subapp = App::new();

app.add_sub_app(SubAppLabel, subapp, move |app_world, subapp| {
    mem::swap(app_world, &mut subapp.world);
    START.call_once(|| {
        subapp.add_plugin(MyApp);
    });
    subapp.update();
    mem::swap(app_world, &mut subapp.world);
    //app_world = &mut subapp.world;
});*/

/*
app.add_plugins_with(DefaultPlugins, |group| {
    group
        .disable::<bevy::log::LogPlugin>()
        .disable::<bevy::log::LogPlugin>()
        .disable::<bevy::window::WindowPlugin>()
        .disable::<bevy::winit::WinitPlugin>()
        .disable::<bevy::core::CorePlugin>()
})*/
