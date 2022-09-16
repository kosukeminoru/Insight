use crate::struc::NetworkEvent;
use crate::struc::NetworkInfo;
use crate::struc::PlayerID;
use crate::struc::Request;
//use crate::subapps;
use bevy::app::App;
//use bevy::app::AppLabel;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
//use subapps::renderer::camera::pan_orbit;
//use subapps::renderer::geometry::my_plane;

pub fn run(
    reciever: Receiver<NetworkInfo>,
    sender: Sender<Request>,
    remote_rec: Receiver<NetworkEvent>,
) {
    /* create app
       structs are added as resources which can be fetched later
       Rapier added as a plugin
       Events or functions are added as systems
       Level and graphics are created in start_up system
       Please see Bevy Documentation
    */

    let mut app = App::new();
    let info;
    match reciever.try_recv() {
        Ok(i) => info = i,
        Err(_) => info = NetworkInfo::default(),
    }
    app.insert_resource(info);

    app.add_plugins_with(DefaultPlugins, |group| {
        group.disable::<bevy::log::LogPlugin>()
    });
    // app.add_plugin(MyApp);
    app.insert_resource(reciever);
    app.insert_resource(sender);
    app.add_system(update_accounts);
    //app.add_system(network_events);

    app.run();
}

fn update_accounts(mut commands: Commands, r: Res<Receiver<NetworkInfo>>) {
    if let Result::Ok(accounts) = r.try_recv() {
        commands.insert_resource(accounts);
    }
}
fn network_events(
    mut commands: Commands,
    r: Res<Receiver<NetworkEvent>>,
    query: Query<(Entity, &PlayerID, &mut Transform)>,
) {
    let v: Vec<NetworkEvent> = r.iter().collect();
    for m in v.iter() {
        let mut ent;
        let mut trans;
        for (a, b, t) in query.iter() {
            if b.id == m.player {
                ent = a;
                trans = t;
                break;
            }
        }
        match m.input.key {
            /*
            1 => {
                commands
                    .spawn()
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::ball(5.0))
                    .insert(PlayerID { id: m.player })
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 0.25, 0.75),
                            custom_size: Some(Vec2::new(5.0 * 2.0, 5.0 * 2.0)),
                            ..default()
                        },
                        transform: Transform {
                            ..Default::default()
                        },
                        ..default()
                    })
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Restitution::coefficient(0.5))
                    .insert(Velocity { ..default() })
                    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
            }
            2 => {
                commands.entity(ent).despawn();
            }
            3 => trans.translation[0] += 1.0,
            4 => trans.translation[0] -= 1.0,
            5 => trans.translation[1] += 1.0,
            6 => trans.translation[1] -= 1.0,*/
            _ => (),
        }
    }
}
/*
pub struct MyApp;
impl Plugin for MyApp {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(game::conf::setup_graphics)
        //.add_startup_system(game::level::setup_level)
        //.add_startup_system(add)
        //.add_system(game::player::player)
        //.add_system(game::collisions::win)
        //.add_system(game::collisions::collisions);
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



        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
            .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default()) // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.
            .add_startup_system(my_plane::setup_plane)
            .add_startup_system(pan_orbit::spawn_camera)
            .add_system(pan_orbit::pan_orbit_camera)
            .add_system(my_plane::add_block);
    }
}*/


fn movement(
    mut commands: Commands,
    s: Res<Sender<NetworkEvent>>,
    input: Res<KeyboardInput>,
    query: Query<&Transform, With<LocalPlayer>>,
) {
    let t = query.get_single().expect("error");
    match input.key_code {
        Some(KeyCode::W) => trans.translation[0] += 1.0,
        Some(KeyCode::A) => trans.translation[0] -= 1.0,
        Some(KeyCode::S) => trans.translation[0] -= 1.0,
        Some(KeyCode::D) => trans.translation[0] += 1.0,
        Some(_) => (),
        None => (),
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
*/
