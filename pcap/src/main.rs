extern crate pnet;

use std::collections::{HashSet};
use std::thread;
use std::sync::mpsc;
use std::ops::Deref;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::{Packet, arp, tcp, udp};

#[derive(Clone, Debug)]
struct PacketWithInterface {
    interface: NetworkInterface,
    packet: Vec<u8>,
}

fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    println!(
        "{}: {} > {}",
        interface.name,
        ethernet.get_source(),
        ethernet.get_destination(),
    );

    print!("  {}: ", ethernet.get_ethertype());
    match ethernet.get_ethertype() {
        EtherTypes::Arp => {
            let arp = arp::ArpPacket::new(ethernet.payload()).unwrap();
            match arp.get_operation() {
                arp::ArpOperations::Reply => {
                    println!(
                        "ARP reply({}): {} -> {}",
                        arp.get_sender_proto_addr(),
                        arp.get_sender_hw_addr(),
                        arp.get_target_hw_addr()
                    );
                }
                arp::ArpOperations::Request => {
                    println!(
                        "ARP request({}): {} -> {}",
                        arp.get_target_proto_addr(),
                        arp.get_sender_hw_addr(),
                        arp.get_target_hw_addr()
                    );
                }
                _ => (),
            }
        }
        EtherTypes::Ipv4 => {
            let ip = Ipv4Packet::new(ethernet.payload()).unwrap();
            println!("{} -> {}", ip.get_source(), ip.get_destination());
            handle_ip_packet(&interface, &ip)
        }
        _ => (),
    }
}

fn handle_ip_packet(interface: &NetworkInterface, ip: &Ipv4Packet) {
    print!("    {}: ", ip.get_next_level_protocol());
    match ip.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => {
            let tcp = tcp::TcpPacket::new(ip.payload()).unwrap();
            handle_tcp_packet(&interface, &tcp);
        }
        IpNextHeaderProtocols::Udp => {
            let udp = udp::UdpPacket::new(ip.payload()).unwrap();
            handle_udp_packet(&interface, &udp);
        }
        _ => (),
    }
}

fn handle_tcp_packet(_interface: &NetworkInterface, tcp: &tcp::TcpPacket) {
    println!("{} -> {}", tcp.get_source(), tcp.get_destination());
}

fn handle_udp_packet(_interface: &NetworkInterface, udp: &udp::UdpPacket) {
    println!("{} -> {}", udp.get_source(), udp.get_destination());
}

fn main() {
    let interface_names: HashSet<&str> = vec!["lo0", "en0"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|interface: &NetworkInterface| interface_names.contains(interface.name.as_str()))
        .collect();

    let (sender, receiver) = mpsc::channel();

    let mut handles: Vec<_> = interfaces.into_iter()
        .map(|interface| {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut rx = datalink::channel(&interface, Default::default())
                    .map(|chan| match chan {
                        Ethernet(_, rx) => rx,
                        _ => panic!("could not initialize datalink channel {:?}", interface.name),
                    }).unwrap();

                loop {
                    match rx.next() {
                        Ok(src) => {
                            sender.send(PacketWithInterface {
                                interface: interface.clone(),
                                packet: src.to_owned(),
                            }).unwrap();

                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
            })
        }
        )
        .collect();

    handles.push(thread::spawn(move || {
        loop {
            match receiver.recv() {
                Ok(packet_with_interface) => {
                    let _packet = packet_with_interface.packet.deref();
                    match EthernetPacket::new(_packet) {
                        Some(packet) => {
                            handle_ethernet_frame(&packet_with_interface.interface, &packet);
                        },
                        _ => {
                            continue;
                        }
                    }
                },
                _ => {
                    continue;
                }
            }
        }
    }));


    for h in handles {
        h.join().unwrap();
    }
}