use crate::game;
use bevy::app::App;
use bevy::app::AppLabel;
use bevy::ecs::event::Events;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier2d::pipeline::ContactForceEvent;
use bevy_rapier2d::prelude::*;
use std::mem;
use std::sync::Once;

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
    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    });

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
        app.add_plugins(MinimalPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            //.add_plugin(RapierDebugRenderPlugin::default())
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

/*
#[derive(Component)]
struct Camera;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.0, 0.90, 1.0),
        brightness: 0.05,
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1450.0,
            color: Color::ORANGE_RED,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.0, 0.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(Camera);
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera = query.single_mut();
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);

    for ev in motion_evr.iter() {
        let yaw = (ev.delta.x * (-0.2)).to_radians();
        let pitch = (ev.delta.y * (-0.2)).to_radians();

        let rot = camera.rotation;
        camera.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * rot * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

fn camera_control(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::W) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::A) {
            direction.z -= 1.0;
        }
        if input.pressed(KeyCode::S) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction.z += 1.0;
        }
        if input.pressed(KeyCode::R) {
            println!("{:?}", transform.rotation.to_axis_angle());
            println!("{:?}", transform.forward());
        }

        transform.translation += time.delta_seconds() * 2.0 * direction;
    }
}

/*.insert_resource(ClearColor(Color::rgb(0.75, 0.90, 1.0)))
   .add_startup_system(setup)
   .add_system(bevy::input::system::exit_on_esc_system)
   .add_system(camera_control)
   .add_system(mouse_motion);
*/
*/

/*    subapp
.add_plugins_with(DefaultPlugins, |group| {
    group
        .disable::<bevy::log::LogPlugin>()
        .disable::<bevy::winit::WinitPlugin>()
})
//.add_plugin(RapierDebugRenderPlugin::default())
.add_startup_system(game::conf::setup_graphics)
.add_startup_system(game::level::setup_level)
.add_startup_system(add)
.add_system(game::player::player)
.add_system(game::collisions::win)
.add_system(game::collisions::collisions); */
