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


fn main() {
    let interface_names: HashSet<&str> = vec!["lo0", "en0", "en1"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|interface: &NetworkInterface| interface_names.contains(interface.name.as_str()))
        .collect();

    // queueをnewする

    let queue = Queue::new();
    queue.add(1);
    println!("{:?}", queue.get());


    // interfacesをiterateしてrxを監視する（マルチスレッド）
    // packet_handlerもここで一緒にマルチスレッドにしたい
}