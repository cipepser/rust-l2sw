extern crate pnet;

use std::collections::{HashSet, HashMap};
use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;
//use pnet::util;

fn receive_packet(interface: &NetworkInterface, mac_address_table: &mut HashMap<pnet::datalink::MacAddr, MacAddressRecord>) -> Result<(), String> {
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
//                println!(
//                    "{}: {} -> {}",
//                    interface.name,
//                    packet.get_ethertype(),
//                    packet.get_source(),
//                    packet.get_destination()
//                );
//                let _record = mac_address_table[&packet.get_source()].lock(); // .unwrap()が例にはあるけど、Resultで返したい
                println!("hoge");
//                update_MacAddressTable(&mut mac_address_table[&packet.get_source()]);
//                handle_packet(&interface, &packet);
                println!("{} updated", packet.get_source());
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
    device_no: u32,
    last_time: Duration,
}

//fn update_MacAddressTable(_record: &mut Mutex<MacAddressRecord>) {
//    let record.device_no = 1;
//}

fn main() {
    let interface_names: HashSet<&str> = vec!["lo0", "en0", "en1"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|interface: &NetworkInterface| interface_names.contains(interface.name.as_str()))
        .collect();

    let mac_address_table: Arc<Mutex<HashMap<pnet::datalink::MacAddr, MacAddressRecord>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // 送信用のバッファを初期化
    // - どのI/Fのtxに送るのか

    // TODO: 受信したらMACアドレステーブルを更新
    // 最初はブロードキャストするだけでいい？
    // ARP requestを投げる？
    let handles: Vec<_> = interfaces
        .into_iter()
        .map(|interface| {
            let mut mac_address_table = mac_address_table.clone();

            thread::spawn(move || {
                // TODO: 今のままだとblockしてしまう
                let mut mac_address_table = mac_address_table.lock().unwrap();
                match receive_packet(&interface, &mut mac_address_table) {
                    Ok(_chan) => println!("ok in main"),
                    Err(e) => panic!(e),
                };
            })
        })
        .collect();

//    let mac_address_table = mac_address_table.clone();
    for h in handles {
//        let mac_address_table = mac_address_table.clone();
        h.join().unwrap();
    }

    // tx: バッファからパケットを取り出して送信？
    //     これを実現しようとするとblockが起きる？

    // 各テーブルのaging timerをいつ更新するか
    // ⇛学習したタイミングでlastTimeを更新する
    // テーブルを調べたときに now > lastTimeでもう一度ブロードキャストするか決める
}