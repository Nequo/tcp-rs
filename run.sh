#!/bin/bash

cargo b --release
ext=$?
if [[ $ext -ne 0 ]]; then
    exit $ext
fi
sudo setcap  'cap_net_admin=eip' ./target/release/tcp-rs
./target/release/tcp-rs &
pid=$!
sudo ip addr add 10.0.0.1/24 dev tun0
sudo ip link set up dev tun0
trap 'kill $pid' INT TERM
wait $pid
