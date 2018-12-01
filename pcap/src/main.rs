extern crate pnet;

use std::collections::{HashSet, VecDeque};
use std::thread;
use std::sync::{Mutex, Arc};
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::{Packet, arp, tcp, udp};

#[derive(Clone, Debug)]
struct Queue<T: Send + Copy> {
    inner: Arc<Mutex<VecDeque<T>>>,
}

impl <T: Send + Copy>Queue<T> {
    fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(VecDeque::new())) }
    }

    fn get(&self) -> Option<T> {
        let _queue = self.inner.lock();
        if let Ok(mut queue) = _queue {
            queue.pop_front()
        } else {
            None
        }
    }

    fn add(&self, obj: T) -> usize {
        let _queue = self.inner.lock();
        if let Ok(mut queue) = _queue {
            queue.push_back(obj);
            queue.len()
        } else {
            0
        }
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

    let queue: Queue<i32> = Queue::new(); // i32はこの時点でコンパイル通すために暫定的にplaceしている


    // interfacesをiterateしてrxを監視する（マルチスレッド）
    // packet_handlerもここで一緒にマルチスレッドにしたい
}