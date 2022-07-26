use crate::struc::NetworkInfo;
use crate::subapps;
use bevy::app::App;
//use bevy::app::AppLabel;
use bevy::prelude::*;
use crossbeam_channel::Receiver;
use subapps::renderer::camera::pan_orbit;
use subapps::renderer::geometry::my_plane;

pub fn run(reciever: Receiver<NetworkInfo>) {
    /* create app
       structs are added as resources which can be fetched later
       Rapier added as a plugin
       Events or functions are added as systems
       Level and graphics are created in start_up system
       Please see Bevy Documentation
    */

    let mut app = App::new();

    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    });
    app.add_plugin(MyApp);
    app.insert_resource(reciever);
    app.add_system(update_accounts);

    app.run();
}

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
}

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
