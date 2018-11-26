extern crate pnet;

use std::collections::HashSet;
use std::thread;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;

fn receive_packet(interface: &NetworkInterface, rx: &mut Box<datalink::DataLinkReceiver>) -> Result<(), String> {
    loop {
        let next_packet = rx.next()
            .map_err(|e| format!("An error occurred when read next packet: {}", e.to_string()))
            .and_then(|packet| {
                EthernetPacket::new(packet).ok_or("failed to parse ethernet packet".to_string())
            });

        match next_packet {
            Ok(packet) => {
                println!(
                    "{}: {}, {} > {}",
                    interface.name,
                    packet.get_ethertype(),
                    packet.get_source(),
                    packet.get_destination(),
                );
            }
            Err(err) => {
                println!("failed to read next packet {}, ignore and continue.", err);
                continue;
            }
        }
    }
}

fn main() {
    let interface_names: HashSet<&str> = vec!["lo0", "en0", "en1"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|interface: &NetworkInterface| interface_names.contains(interface.name.as_str()))
        .collect();

    let handles: Vec<_> = interfaces.into_iter()
        .map(|interface|
            thread::spawn(move || {
                let rx = datalink::channel(&interface, Default::default())
                    .map(|chan| match chan {
                        Ethernet(_, rx) => rx,
                        _ => panic!("could not initialize datalink channel {:?}", interface.name),
                    });
                match receive_packet(&interface, &mut rx.unwrap()) {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e.to_string()),
                };
            })
        )
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
