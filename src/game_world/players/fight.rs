#![allow(dead_code)]

use super::info;
use crate::game_world::ggrs_rollback::network::{self, GGRSConfig};
use bevy::prelude::*;
use components::struc::Request;
use crossbeam_channel::Sender;
use ggrs::{
    Config, P2PSession, PlayerType, SessionBuilder, SpectatorSession, SyncTestSession,
    UdpNonBlockingSocket,
};
// Damage player by pressing Return key if in a radius of 2.50.
pub fn fight(
    mut players: Query<(Entity, &Transform, &mut info::Player)>,
    mut me: Query<(Entity, &Transform, &network::Me)>,
    world: Res<World>,
    p2p_session: Option<Res<P2PSession<GGRSConfig>>>,
    keyboard_input: Res<Input<KeyCode>>,
    send: Res<Sender<Request>>,
) {
    let (me_entity, me_transform, me_Me) = me.single_mut();
    for (entity, transform, mut player) in players.iter_mut() {
        if me_transform.translation.distance(transform.translation) < 2.5
            && entity.id() != me_entity.id()
            && keyboard_input.pressed(KeyCode::Return)
        {
            if player.health >= 20 {
                player.health -= 20;
                println!("Damage");
            }
            if player.health == 0 {
                if let vec = p2p_session.unwrap() {
                    let inputs = vec.into_inner().confirmed_inputs;
                    send.send(Request::CreateBlock(inputs, world));
                }
                println!("Killed");
            }
            println!("{}", player.health);
        }
    }
}
