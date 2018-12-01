#!/usr/bin/bash

sudo ip netns add host
sudo ip netns add RT2
sudo ip netns add RT1

sudo ip link add host_veth1 type veth peer name RT2_veth0
sudo ip link add RT2_veth1 type veth peer name RT1_veth0

sudo ip link set host_veth1 netns host
sudo ip link set RT2_veth0 netns RT2
sudo ip link set RT2_veth1 netns RT2
sudo ip link set RT1_veth0 netns RT1

sudo ip netns exec host ip addr add 192.168.0.1/24 dev host_veth1
sudo ip netns exec RT2 ip addr add 192.168.0.254/24 dev RT2_veth0
sudo ip netns exec RT2 ip addr add 192.168.1.1/24 dev RT2_veth1
sudo ip netns exec RT1 ip addr add 192.168.1.254/24 dev RT1_veth0

sudo ip netns exec host ip link set lo up
sudo ip netns exec RT2 ip link set lo up
sudo ip netns exec RT1 ip link set lo up

sudo ip netns exec host ip link set host_veth1 up
sudo ip netns exec RT2 ip link set RT2_veth0 up
sudo ip netns exec RT2 ip link set RT2_veth1 up
sudo ip netns exec RT1 ip link set RT1_veth0 up

sudo ip netns exec host ip route add default via 192.168.0.254
sudo ip netns exec RT2 ip route add default via 192.168.1.254
sudo ip netns exec RT1 ip route add 192.168.0.0/24 via 192.168.1.1

sudo ip netns exec host sysctl -w net.ipv4.ip_forward=1
sudo ip netns exec RT2 sysctl -w net.ipv4.ip_forward=1
sudo ip netns exec RT1 sysctl -w net.ipv4.ip_forward=1