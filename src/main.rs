use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        // Skip non-ipv4 packets
        if eth_proto != 0x0800 {
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let payload_len = p.payload_len().unwrap();
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                // skip non TCP
                if proto != etherparse::IpNumber::TCP {
                    continue;
                }
                eprintln!(
                    "{:?} -> {:?}, {:?} bytes of protocol: {:?}",
                    src, dst, payload_len, proto
                );
            }
            Err(e) => eprintln!("Ignoring malformed packet: {:?}", e),
        }
    }
}
