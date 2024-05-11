use std::io;

pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

impl Default for State {
    fn default() -> Self {
        // State::Closed;
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        eprintln!(
            "{}:{} -> {}:{}, {} bytes of protocol: TCP",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len()
        );
        match *self {
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcph.syn() {
                    // Only expected syn packet
                    return Ok(0);
                }
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    tcph.sequence_number(),
                    0,
                );
                syn_ack.syn = true;
                syn_ack.ack = true;
                let ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len().try_into().unwrap(),
                    64,
                    etherparse::IpNumber::TCP,
                    iph.destination(),
                    iph.source(),
                )
                .unwrap();
                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten)?;
                    syn_ack.write(&mut unwritten)?;
                    unwritten.len()
                };
                nic.send(&buf[..unwritten])
            }
            _ => todo!(),
        }
    }
}
