extern crate pnet;

use std::collections::{HashSet, HashMap};
use std::thread;
use std::sync::Mutex;
use std::time::Duration;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::util;

fn receive_packet(interface: &NetworkInterface) -> Result<(), String> {
    println!("name: {:?}", interface.name);

    let mut rx = datalink::channel(&interface, Default::default())
        .map(|chan| match chan {
            Ethernet(_, rx) => rx,
            _ => panic!("Unhandled channel type"),
        })
        .map_err(|e| {
            format!("An error occurred when creating the datalink channel: {}",
                    e.to_string()
            )
        })?;

    loop {
        let next_packet = rx.next()
            .map_err(|e| format!("An error occurred when read next packet: {}", e.to_string()))
            .and_then(|packet| {
                EthernetPacket::new(packet).ok_or("failed to parse ethernet packet".to_string())
            });

        match next_packet {
            Ok(packet) => {
                println!(
                    "{}: {} -> {}",
                    interface.name,
//                    packet.get_ethertype(),
                    packet.get_source(),
                    packet.get_destination()
                );
//                handle_packet(&interface, &packet);
            }
            Err(err) => {
                println!("failed to read next packet {}, ignore and continue.", err);
                continue;
            }
        }
    }

//    Ok(())
}

struct MacAddressRecord {
    device_no: i32,
    last_time: Duration,
}

fn main() {
    let interface_names: HashSet<&str> = vec!["lo0", "en0", "en1"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|iface: &NetworkInterface| interface_names.contains(iface.name.as_str()))
        .collect();

    let MacAddressTable: HashMap<util::MacAddr, Mutex<MacAddressRecord>> = HashMap::new();


    // 送信用のバッファを初期化
    // - どのI/Fのtxに送るのか


// TODO: 受信したらARPテーブル更新
    let handles: Vec<_> = interfaces
        .into_iter()
        .map(|interface| {
            thread::spawn(move || {
                match receive_packet(&interface) {
                    Ok(_chan) => println!("ok in main"),
                    Err(e) => panic!(e),
                };
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

// tx: バッファからパケットを取り出して送信？
//     これを実現しようとするとblockが起きる？

// 各テーブルのaging timerをいつ更新するか
// ⇛学習したタイミングでlastTimeを更新する
// テーブルを調べたときに now > lastTimeでもう一度ブロードキャストするか決める
}