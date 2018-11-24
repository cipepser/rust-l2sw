extern crate pnet;

use std::collections::HashSet;
use std::thread;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;

// この関数の責務が明確化されていないから詰まっている気がする
// interfaceを受け取って、channelでrxを作る
//fn receive_packet(interface: &NetworkInterface) {
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
                println!("{}", err);
//                error!("failed to read next packet {}, ignore and continue.", err);
                continue;
            }
        }
    }

    // ?演算子はResult型に適用されてOk(T)ならunwrapした値を返す
    // Err(E)なら関数からErr(e)を返して抜ける
    Ok(())
}

fn main() {
    let interface_names: HashSet<&str> = vec!["lo0", "en0", "en1"]
        .into_iter()
        .collect();

    let interfaces: Vec<NetworkInterface> = datalink::interfaces()
        .into_iter()
        .filter(|iface: &NetworkInterface| interface_names.contains(iface.name.as_str()))
        .collect();

// MACアドレステーブル作成
// key: MACアドレス
// device No.
// lastTime

// 送信用のバッファを初期化
// - どのI/Fのtxに送るのか

// rx: パケットの受信を行う。
//     受信したらARPテーブル更新？
//     各I/Fは独立してrxで受信するパケットを監視する
//     MACアドレステーブルから、OptionでMACアドレスを取得して、
//     Some()とNoneで挙動を変える
//     Some(packet) =>
//     None => {
//         packet
//     }
// TODO: まずはここでパケットキャプチャを~並列に~できるようにする
// datalink::channelで(rx, tx)のパケットキャプチャ？
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
//     MACアドレステーブルに


// 各テーブルのaging timerをいつ更新するか
// ⇛学習したタイミングでlastTimeを更新する
// テーブルを調べたときに now > lastTimeでもう一度ブロードキャストするか決める
}