use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::constants::WX as WX;
use crate::game::constants::WY as WY;

//configuration of physics simulator
//Adjust dt and substeps by same factor to retain determinism
pub fn rap_conf() -> RapierConfiguration {
    bevy_rapier2d::plugin::RapierConfiguration {
    gravity: Vect::Y * -9.81 * 10.0,
    physics_pipeline_active: true,
    query_pipeline_active: true,
    timestep_mode: TimestepMode::Fixed {
        dt: 0.1,
        substeps: 10,
    },
    scaled_shape_subdivision: 10,
    }
}

//configuration of simulation pop-up window
pub fn window_conf() -> WindowDescriptor {
    bevy::window::WindowDescriptor{
        width: WX,
        height: WY,
        title: "game".to_string(),
        resizable: false,
        
        ..Default::default()
    }
}

//validation window configuration
pub fn val_window_conf() -> WindowDescriptor {
    bevy::window::WindowDescriptor{
        width: 0.0,
        height: 0.0,
        title: "game".to_string(),
        resizable: false,
        
        ..Default::default()
    }
}

//Graphics
pub fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

