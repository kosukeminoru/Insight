use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::conf as conf;
use crate::game::level as level;
use crate::game::constants::WX as WX;
use crate::game::constants::WY as WY;

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
        .add_system(player)
        .run();
}

fn player(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        create_player(commands, window);
    }

}



fn create_player(mut commands: Commands, windows: &Window) {
    let a = windows.cursor_position().unwrap().to_array();
    let x = a[0]-WX/2.0;
    let y = a[1]-WY/2.0;

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform::from_xyz(x,y,0.0),
        ..default()
    });
    println!("{:?}", windows.physical_cursor_position().unwrap().extend(0.0));
}

/*
fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        
    }
}*/