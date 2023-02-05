use futures::executor::block_on;
use futures::prelude::*;
use libp2p::{identity, PeerId};
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::Swarm;
use std::error::Error;
use std::task::Poll;

pub fn main(peers: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(key.public());
    println!("New local peer ID {:?}", local_peer_id);

    let transport = block_on(libp2p::development_transport(key))?;

    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;


    for peer in peers {
        let remote = peer.parse()?;
        swarm.dial_addr(remote)?;
        println!("Dialed {}", peer)
    }

    let mut listening = false;
    block_on(future::poll_fn(move |cx| loop {
        match swarm.poll_next_unpin(cx) {
            Poll::Ready(Some(event)) => println!("Got event {:?}", event),
            Poll::Ready(None) => return Poll::Ready(()),
            Poll::Pending => {
                if !listening {
                    for addr in Swarm::listeners(&swarm) {
                        println!("Listening on {}", addr);
                        listening = true;
                    }
                }
                return Poll::Pending;
            }
        }
    }));

    Ok(())
}