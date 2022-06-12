use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::constants::WX as WX;
use crate::game::constants::WY as WY;
//use crate::game::constants as cnst;
//use crate::game::level as level;

//configuration of physics simulator
pub fn rap_conf() -> RapierConfiguration {
    bevy_rapier2d::plugin::RapierConfiguration {
    gravity: Vect::Y * -9.81 * 10.0,
    physics_pipeline_active: true,
    query_pipeline_active: true,
    timestep_mode: TimestepMode::Fixed {
        dt: 0.01,
        substeps: 1,
    },
    scaled_shape_subdivision: 10,
//Ball altitude: 89.78951
//Ball altitude: -10.104906
    }
}

//configuration of pop-up window
pub fn window_conf() -> WindowDescriptor {
    bevy::window::WindowDescriptor{
        width: WX,
        height: WY,
        title: "game".to_string(),
        resizable: false,
        
        ..Default::default()
    }
}

//Graphics
pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
   /*commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    });*/
}

//Placing rigid bodies within the world
