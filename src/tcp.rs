use std::io;
use std::io::prelude::*;

pub enum State {
    Closed,
    Listen,
    // SynRcvd,
    // Estab,
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: RecvSequenceSpace,
}

struct SendSequenceSpace {
    // send unack
    una: usize,
    // send next
    nxt: usize,
    // send window
    wnd: usize,
    // send urgent pointer
    up: bool,
    // segment sequence number used for last window update
    wl1: usize,
    // segment ack number used for last window update
    wl2: usize,
    // initial send sequence number
    iss: usize,
}

struct RecvSequenceSpace {
    // receive next
    nxt: usize,
    // receive window
    wnd: usize,
    // receive urgent pointer
    up: bool,
    // initial receive sequence number
    irs: usize,
}


impl Default for Connection {
    fn default() -> Self {
        // State::Closed
        Connection {
            state: State::Listen,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        // State::Closed
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a> (
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        match *self {
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcph.syn() {
                    // got unexpected SYN packet
                    return Ok(0);
                }
                 
                // need to establish a new connection
                let mut syn_ack = etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), 0, 0);
                syn_ack.syn = true;
                syn_ack.ack = true;
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    etherparse::IpTrafficClass::Tcp,
                    [
                        iph.destination()[0],
                        iph.destination()[1],
                        iph.destination()[2],
                        iph.destination()[3],
                    ],
                    [
                        iph.source()[0],
                        iph.source()[1],
                        iph.source()[2],
                        iph.source()[3],
                    ],
                );
                // write out the headers 
                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten);
                    syn_ack.write(&mut unwritten);
                    unwritten.len()
                };
                nic.send(&buf[..unwritten])
            }
        }
    }
}
