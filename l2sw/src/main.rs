extern crate pnet;

use std::collections::{HashSet, HashMap};
use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use std::io;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;
//use pnet::util;

// receive_packetの責務が大きすぎる？
// - interfaceからrxを取得 <- これはテーブルのlockを取る必要がない
//   ⇛ このあとのことを考えるとchannel(tx, rx)のどちらも持っておきたい
// - rxを監視して、パケットがあればそれを処理する
fn receive_packet(interface: &NetworkInterface, mac_address_table: &mut HashMap<pnet::datalink::MacAddr, MacAddressRecord>) -> Result<(), String> {
//fn receive_packet(interface: &NetworkInterface, ) -> Result<(), String> {
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
                    packet.get_destination(),
                );
                // TODO: ここで初めてRecordに対するlockを取りたい
                // ⇛ receive_packetの引数にmac_address_tableがあることが設計が間違っている？
                // MacAddressTable型に対して`update`とかを呼び出すべきでは？
                // でもこの関数内ではMacAddressTableのことを知らないので、どうにかして知る必要がある
//                let _record = mac_address_table[&packet.get_source()].lock(); // .unwrap()が例にはあるけど、Resultで返したい
//                update_MacAddressTable(&mut mac_address_table[&packet.get_source()]);
//                handle_packet(&interface, &packet);
//                println!("{} updated", packet.get_source());
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
//    last_time: Duration,
}

impl MacAddressRecord {
    fn update_divice_no(&mut self, device_no: u32) {
        self.device_no = device_no;
    }
}

fn main() {
    let interface_names: HashSet<&str> = vec!["lo0", "en0", "en1"]
//    let interface_names: HashSet<&str> = vec!["en0"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|interface: &NetworkInterface| interface_names.contains(interface.name.as_str()))
        .collect();

    // TODO: Arc(Mutex())するのをMacAddressRecordごとにしたい
    let mac_address_table: Arc<Mutex<HashMap<pnet::datalink::MacAddr, MacAddressRecord>>> =
        Arc::new(Mutex::new(HashMap::new()));

//    let mut channels: Vec<Result<Channel, String>> = interfaces
//        .into_iter()
//        .map(|interface|
//            datalink::channel(&interface, Default::default())
//                .map_err(|e| {
//                    format!("An error occurred when creating the datalink channel: {}",
//                            e.to_string()
//                    )
//                })
//        )
//        .collect();

    let mut channels: Vec<Channel> = interfaces
        .into_iter()
        .map(|interface|
    let chan = match datalink::channel(&interface,
                                       Default::default())
        {
            Ok(chan) => chan,
            Err(err) => println!("An error occured when creating channel but ignore: {}", err),
        };
    )
    .collect();

    // 送信用のバッファを初期化
    // - どのI/Fのtxに送るのか


//    // TODO: 受信したらMACアドレステーブルを更新
//    // 最初はブロードキャストするだけでいい？
//    // ARP requestを投げる？
//    let handles: Vec<_> = interfaces
//        .into_iter()
//        .map(|interface| {
//            let mut mac_address_table = mac_address_table.clone();
//
//            thread::spawn(move || {
//                // TODO: 今のままだと、loopの外でlockを取っているのでunlockされず、blockしてしまう
//                // receive_packet()の返り値をResult<MacAddressRecord, String>にすることでlockを取らなくてよくできないか？
//                // rxのloopとの切り離しが設計できていないからちょっと厳しそう
//                // rxの取得自体はマルチスレッドにする必要はない
//                let mut mac_address_table = mac_address_table.lock().unwrap();
//                match receive_packet(&interface, &mut mac_address_table) {
////                match receive_packet(&interface) {
//                    Ok(_record) => println!("ok in main"),
//                    Err(e) => panic!(e),
//                };
//            })
//        })
//        .collect();
//
////    let mac_address_table = mac_address_table.clone();
//    for h in handles {
////        let mac_address_table = mac_address_table.clone();
//        h.join().unwrap();
//    }
//
//    // tx: バッファからパケットを取り出して送信？
//    //     これを実現しようとするとblockが起きる？
//
//    // 各テーブルのaging timerをいつ更新するか
//    // ⇛学習したタイミングでlastTimeを更新する
//    // テーブルを調べたときに now > lastTimeでもう一度ブロードキャストするか決める
}