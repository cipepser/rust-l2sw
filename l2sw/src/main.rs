extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use std::collections::HashSet;

fn main() {
    let interface_names: HashSet<&str> = vec!["lo0", "en0", "en1"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|iface: &NetworkInterface| interface_names.contains(iface.name.as_str()))
        .collect();
    
    // datalink::channelで(rx, tx)のパケットキャプチャ
}