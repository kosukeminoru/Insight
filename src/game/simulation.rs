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

use crate::subapps;
// use subapps::renderer::camera::pan_orbit;
// use subapps::renderer::geometry::my_plane;

use subapps::default_plane::bevy_blender::camera;
use subapps::default_plane::cubemap::camera_controller;
use subapps::default_plane::cubemap::cubemap_materials;
use subapps::default_plane::cubemap::cubemap_setup;
use subapps::default_plane::gltf::bevy_gltf_setup;

static START: Once = Once::new();
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

    let mut app = App::new();
    /*
    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    });*/

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
    });
    app.run();
}

pub struct MyApp;
impl Plugin for MyApp {
    fn build(&self, app: &mut App) {
        app.add_plugins_with(DefaultPlugins, |group| {
            group
                .disable::<bevy::log::LogPlugin>()
                .disable::<bevy::log::LogPlugin>()
                .disable::<bevy::window::WindowPlugin>()
                .disable::<bevy::winit::WinitPlugin>()
                .disable::<bevy::core::CorePlugin>()
        })
        .add_plugin(MaterialPlugin::<CubemapMaterial>::default())
        .add_plugin(MaterialPlugin::<CubemapArrayMaterial>::default())
        
        //blender
        .add_plugin(BlenderPlugin)
        .add_startup_system(setup_blender_camera)
        //.add_system(camera::pan_orbit_camera)

        //cubemap
        .add_startup_system(setup_cubemap)
        .add_system(cycle_cubemap_asset)
        .add_system(asset_loaded.after(cycle_cubemap_asset))
        .add_system(camera_controller)
        .add_system(animate_cubebox_light_direction)

        //fox
        .add_startup_system(setup_fox)
        .add_system(setup_scene_once_loaded)
        .add_system(keyboard_animation_control)

        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        //helmet
        .add_startup_system(setup_helmet)
        .add_system(animate_helmet_light_direction)
    }
}

// impl Plugin for MyApp {
//     fn build(&self, app: &mut App) {
//         app.add_plugins_with(DefaultPlugins, |group| {
//             group
//                 .disable::<bevy::log::LogPlugin>()
//                 .disable::<bevy::log::LogPlugin>()
//                 .disable::<bevy::window::WindowPlugin>()
//                 .disable::<bevy::winit::WinitPlugin>()
//                 .disable::<bevy::core::CorePlugin>()
//         })
//         .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
//         .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default()) // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.
//         .add_startup_system(my_plane::setup_plane)
//         .add_startup_system(pan_orbit::spawn_camera)
//         .add_system(pan_orbit::pan_orbit_camera)
//         .add_system(my_plane::add_block);
//     }
// }
