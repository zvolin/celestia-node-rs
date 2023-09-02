use std::env;

use anyhow::{Context, Result};
use futures::StreamExt;
use libp2p::{
    core::upgrade::Version,
    identify,
    identity::{self, PublicKey},
    noise, request_response,
    swarm::{keep_alive, NetworkBehaviour, SwarmBuilder, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Transport,
};
use tendermint_proto::Protobuf;

use celestia_proto::p2p::pb::{header_request, HeaderRequest};
use celestia_rpc::prelude::*;
use celestia_types::ExtendedHeader;

mod exchange;

const NETWORK: &str = "private";
const WS_URL: &str = "ws://localhost:26658";

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();

    // Create identity
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("local peer id: {local_peer_id:?}");

    // Setup swarm
    let transport = tcp::tokio::Transport::default()
        .upgrade(Version::V1Lazy)
        .authenticate(noise::Config::new(&local_key)?)
        .multiplex(yamux::Config::default())
        .boxed();
    let mut swarm = SwarmBuilder::with_tokio_executor(
        transport,
        Behaviour::new(local_key.public()),
        local_peer_id,
    )
    .build();

    // Get the address of the local bridge node
    let auth_token = env::var("CELESTIA_NODE_AUTH_TOKEN_ADMIN")?;
    let client = celestia_rpc::client::new_websocket(WS_URL, Some(&auth_token)).await?;
    let bridge_info = client.p2p_info().await?;
    let bridge_maddrs: Vec<Multiaddr> = bridge_info
        .addrs
        .into_iter()
        .map(|addr| addr.parse().context("Parsing addr failed"))
        .collect::<Result<_>>()?;
    println!("bridge id: {:?}", bridge_info.id);
    println!("bridge listens on: {bridge_maddrs:?}");

    // Listen for incoming connections
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let bridge_ma = bridge_maddrs
        .into_iter()
        .find(|ma| ma.protocol_stack().any(|protocol| protocol == "tcp"))
        .context("Bridge doesn't listen on tcp")?;

    // Dial the bridge node
    println!("dialing bridge at: {bridge_ma:?}");
    swarm.dial(bridge_ma)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => match event {
                BehaviourEvent::Identify(event) => match event {
                    identify::Event::Received { peer_id, .. } => {
                        println!("Identify event: {event:?}");
                        let req_id = swarm.behaviour_mut().header_ex.send_request(
                            &peer_id,
                            HeaderRequest {
                                amount: 1,
                                data: Some(header_request::Data::Origin(1)),
                            },
                        );
                        println!("Requested header 1 with req_id: {req_id}");
                    }
                    _ => println!("Unhandled identify event: {event:?}"),
                },
                BehaviourEvent::HeaderEx(event) => match event {
                    request_response::Event::Message {
                        peer,
                        message:
                            request_response::Message::Response {
                                request_id,
                                response,
                            },
                    } => {
                        println!(
                            "Response for request: {request_id}, from peer: {peer}, status: {:?}",
                            response.status_code()
                        );
                        let header = ExtendedHeader::decode(&response.body[..])?;
                        println!("Header: {header:?}");
                    }
                    _ => println!("Unhandled header_ex event: {event:?}"),
                },
                BehaviourEvent::KeepAlive(event) => println!("KeepAlive event: {event:?}"),
            },
            e => println!("other: {e:?}"),
        }
    }
}

/// Our network behaviour.
#[derive(NetworkBehaviour)]
struct Behaviour {
    identify: identify::Behaviour,
    header_ex: exchange::ExchangeBehaviour,
    keep_alive: keep_alive::Behaviour,
}

impl Behaviour {
    fn new(pubkey: PublicKey) -> Self {
        let identify = identify::Behaviour::new(identify::Config::new("".to_owned(), pubkey));
        Self {
            identify,
            header_ex: exchange::exchange_behaviour(NETWORK),
            keep_alive: keep_alive::Behaviour,
        }
    }
}
