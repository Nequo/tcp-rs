# TCP-rs

Rust implementation of the TCP protocol following [Jon Gjengset's videos](https://www.youtube.com/watch?v=bzja9fQWzdA).

## Running the environment

I am running this on an M2 mac in a lima VM. By default, lima currently mounts the home directory as sshfs/fuse. This does not support extended filesystem attributes required by setcap to delegate CAP_NET_ADMIN to the generated binary.

```bash
$ sudo strace setcap  'cap_net_admin=ep' target/release/tcp-rs
[...]
setxattr("target/release/tcp-rs", "security.capability", "\1\0\0\2\0\20\0\0\0\0\0\0\0\0\0\0\0\0\0", 20, 0) = -1 EOPNOTSUPP (Operation not supported)
```

We can use the virtiofs mount type to make setcap work:

```bash
limactl start --mount-writable --vm-type=vz --mount-type=virtiofs template://fedora
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
./run.sh
```
