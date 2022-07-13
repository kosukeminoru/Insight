//use crate::networks::webrtc;
use libp2p::core;
use libp2p::identity;
use libp2p::Transport;
use libp2p::{dns, mplex, noise, tcp, websocket, yamux, PeerId};

use anyhow::Result;
use async_trait::async_trait;
use futures::future::FutureExt;
use futures::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use futures::stream::StreamExt;
use libp2p::core::upgrade;
use libp2p::request_response::{
    ProtocolName, ProtocolSupport, RequestResponse, RequestResponseCodec, RequestResponseConfig,
    RequestResponseEvent, RequestResponseMessage,
};
use libp2p::swarm::{Swarm, SwarmBuilder, SwarmEvent};

use rand::RngCore;
use std::time::Instant;
use std::{io, iter};

pub async fn build_transport(
    keypair: identity::Keypair,
) -> std::io::Result<core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>> {
    let transport = {
        let dns_tcp = dns::DnsConfig::system(tcp::TcpTransport::new(
            tcp::GenTcpConfig::new().nodelay(true),
        ))
        .await?;
        let ws_dns_tcp = websocket::WsConfig::new(
            dns::DnsConfig::system(tcp::TcpTransport::new(
                tcp::GenTcpConfig::new().nodelay(true),
            ))
            .await?,
        );
        dns_tcp.or_transport(ws_dns_tcp)
    };

    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&keypair)
        .expect("Signing libp2p-noise static DH keypair failed.");

    Ok(transport
        .upgrade(core::upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(core::upgrade::SelectUpgrade::new(
            yamux::YamuxConfig::default(),
            mplex::MplexConfig::default(),
        ))
        .timeout(std::time::Duration::from_secs(20))
        .boxed())
}
