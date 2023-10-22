extern crate tun_tap;
use std::io;

fn main() -> io::Result<()>{
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to create tun_tap instance");
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        eprintln!("read flags: {:x?}", &buf[..2]);
        eprintln!("read proto: {:x?}", &buf[2..4]);
        eprintln!("read {} bytes: {:x?}", nbytes - 4, &buf[4..nbytes]);
    }
    Ok(())
}
