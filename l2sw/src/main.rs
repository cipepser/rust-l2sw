extern crate pnet;

use std::collections::HashSet;
use std::thread;
use pnet::datalink::{self, NetworkInterface};

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
    // TODO: まずはここでパケットキャプチャを並列にできるようにする
    // datalink::channelで(rx, tx)のパケットキャプチャ？
    // 各I/Fをどうやって扱うか
    // let (mut tx, mut rx) = datalink::channel()
    // Resultを返すようにして、関数化したほうがいいかも。
    interfaces
        .into_iter()
        .map(|iface: &NetworkInterface|
            let (mut tx, mut rx) = datalink::channel(iface, Default::default());

        );


    // tx: バッファからパケットを取り出して送信？
    //     これを実現しようとするとblockが起きる？
    //     MACアドレステーブルに


    // 各テーブルのaging timerをいつ更新するか
    // ⇛学習したタイミングでlastTimeを更新する
    // テーブルを調べたときに now > lastTimeでもう一度ブロードキャストするか決める

}