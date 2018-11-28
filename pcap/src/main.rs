extern crate pnet;

use std::collections::{HashSet, VecDeque};
use std::thread;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::{Packet, arp, tcp, udp};

// TODO: 並列処理しているので標準出力の整合性がとれていない
fn receive_packet<'a>(queue: &mut VecDeque<PacketWithInterface<'a>>, interface: &NetworkInterface, rx:  &'a mut Box<datalink::DataLinkReceiver + 'a>) -> Result<(), String> {
    loop {
        let next_packet = rx.next()
            .map_err(|e| format!("An error occurred when read next packet: {}", e.to_string()))
            .and_then(|packet| {
                EthernetPacket::new(packet).ok_or("failed to parse ethernet packet".to_string())
            });

        match next_packet {
            Ok(ethernet) => {
                queue.push_back(
                    PacketWithInterface {
                        interface: interface.clone(),
                        packet: ethernet,
                    }
                );
            }
            Err(err) => {
                println!("failed to read next packet {}, ignore and continue.", err);
                continue;
            }
        }
    }
}

struct PacketWithInterface<'p> {
    interface: NetworkInterface,
    packet: EthernetPacket<'p>,
}

// queueから取り出したpacketを処理する
fn handle_packet(queue: &mut VecDeque<PacketWithInterface>, packet: &PacketWithInterface) {
    let packet = queue.pop_front();
    // TODO: mapにしたい
    match packet {
        Some(packet) => handle_ethernet_packet(&packet.interface, &packet.packet),
        None => (),
    }
}

fn handle_ethernet_packet(interface: &NetworkInterface, ethernet: &EthernetPacket) {
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
    let interface_names: HashSet<&str> = vec!["lo0", "en0", "en1"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|interface: &NetworkInterface| interface_names.contains(interface.name.as_str()))
        .collect();

    let mut queue: VecDeque<PacketWithInterface> = VecDeque::new();

    let mut handles: Vec<_> = interfaces.into_iter()
        .map(|interface|
            thread::spawn(move || {
                let rx = datalink::channel(&interface, Default::default())
                    .map(|chan| match chan {
                        Ethernet(_, rx) => rx,
                        _ => panic!("could not initialize datalink channel {:?}", interface.name),
                    });
                // TODO: get locking the queue
                match receive_packet(&mut queue, &interface, &mut rx.unwrap()) {
                    Ok(_) => (),
                    Err(e) => panic!("{}", e.to_string()),
                };
            })
        )
        .collect();

    handles.push(
        thread::spawn(move || {
            loop {
                // TODO: get locking the queue
                // let _queue = // scopeを抜けたらunlockさせる
                queue.pop_front()
                    .map(|packet| handle_packet(&mut queue, &packet));
            }
        })
    );

    for h in handles {
        h.join().unwrap();
    }
}
