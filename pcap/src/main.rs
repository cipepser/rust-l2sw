extern crate pnet;

use std::collections::HashSet;
use std::thread;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::

fn reveive_packet(rx: &mut Box<DataLinkReceiver>) {

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
                let mut rx = datalink::channel(&interface, Default::default())
                    .map(|chan|
                        Ethernet(_, rx) => rx,
                        _ => panic!("failed to initialize datalink channel {:?}", interface.name),
                    );
                reveive_packet(&mut rx);
            })
        )
        .collect();

    let _ = handles.into_iter().map(|h| h.join());
}