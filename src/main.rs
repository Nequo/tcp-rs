use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::State> = Default::default();
    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
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
            Ok(iph) => {
                let src = iph.source_addr();
                let dst = iph.destination_addr();
                let proto = iph.protocol();
                // skip non TCP
                if proto != etherparse::IpNumber::TCP {
                    continue;
                }
                let ip_hdr_size = iph.slice().len();
                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + iph.slice().len()..nbytes]) {
                    Ok(tcph) => {
                        let tcp_hdr_size = tcph.slice().len();
                        let datai = 4 + ip_hdr_size + tcp_hdr_size;
                        connections
                            .entry(Quad {
                                src: (src, tcph.source_port()),
                                dst: (dst, tcph.destination_port()),
                            })
                            .or_default()
                            .on_packet(&mut nic, iph, tcph, &buf[datai..nbytes])?;
                        // Inserts only if quad doesn't already exist
                    }
                    Err(e) => eprintln!("Ignoring malformed TCP packet: {:?}", e),
                }
            }
            Err(e) => eprintln!("Ignoring malformed packet: {:?}", e),
        }
    }
}
