# rust-l2sw

## 検証用のvagrantメモ

```sh
vagrant init ubuntu/trusty64
vagrant up
```

```sh
$ uname -a
Linux vagrant-ubuntu-trusty-64 3.13.0-161-generic #211-Ubuntu SMP Wed Oct 3 14:52:35 UTC 2018 x86_64 x86_64 x86_64 GNU/Linux
```

```sh
$ sudo bash ./init.sh
net.ipv4.ip_forward = 1
net.ipv4.ip_forward = 1
net.ipv4.ip_forward = 1
```

## 試験時の様子

```sh
# RT1
$ sudo ip netns exec RT1 bash
root@vagrant-ubuntu-trusty-64:~# nc -l -p 1234
```

```sh
# host
vagrant@vagrant-ubuntu-trusty-64:~$ sudo ip netns exec host bash
root@vagrant-ubuntu-trusty-64:~# ping 192.168.1.254 -c 1
PING 192.168.1.254 (192.168.1.254) 56(84) bytes of data.
64 bytes from 192.168.1.254: icmp_seq=1 ttl=63 time=0.706 ms

--- 192.168.1.254 ping statistics ---
1 packets transmitted, 1 received, 0% packet loss, time 0ms
rtt min/avg/max/mdev = 0.706/0.706/0.706/0.000 ms
root@vagrant-ubuntu-trusty-64:~# nc 192.168.1.254 1234
```


```sh
# RT2
vagrant@vagrant-ubuntu-trusty-64:~$ sudo ip netns exec RT2 bash
root@vagrant-ubuntu-trusty-64:~# cd pcap
root@vagrant-ubuntu-trusty-64:~/pcap# sudo ./target/debug/pcap
RT2_veth1: d6:c7:d0:99:8e:bc > ff:ff:ff:ff:ff:ff
  Arp: ARP request(192.168.1.254): d6:c7:d0:99:8e:bc -> 00:00:00:00:00:00
RT2_veth1: f6:63:73:99:cc:cd > d6:c7:d0:99:8e:bc
  Arp: ARP reply(192.168.1.254): f6:63:73:99:cc:cd -> d6:c7:d0:99:8e:bc
RT2_veth1: d6:c7:d0:99:8e:bc > f6:63:73:99:cc:cd
  Ipv4: 192.168.0.1 -> 192.168.1.254
    Icmp: RT2_veth1: f6:63:73:99:cc:cd > d6:c7:d0:99:8e:bc
  Ipv4: 192.168.1.254 -> 192.168.0.1
    Icmp: RT2_veth1: f6:63:73:99:cc:cd > d6:c7:d0:99:8e:bc
  Arp: ARP request(192.168.1.1): f6:63:73:99:cc:cd -> 00:00:00:00:00:00
RT2_veth1: d6:c7:d0:99:8e:bc > f6:63:73:99:cc:cd
  Arp: ARP reply(192.168.1.1): d6:c7:d0:99:8e:bc -> f6:63:73:99:cc:cd
RT2_veth1: d6:c7:d0:99:8e:bc > f6:63:73:99:cc:cd
  Ipv4: 192.168.0.1 -> 192.168.1.254
    Tcp: 45693 -> 1234
RT2_veth1: f6:63:73:99:cc:cd > d6:c7:d0:99:8e:bc
  Ipv4: 192.168.1.254 -> 192.168.0.1
    Tcp: 1234 -> 45693
RT2_veth1: d6:c7:d0:99:8e:bc > f6:63:73:99:cc:cd
  Ipv4: 192.168.0.1 -> 192.168.1.254
    Tcp: 45693 -> 1234
```
