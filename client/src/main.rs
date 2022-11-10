use std::io::prelude::*;
use std::io::Error;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Error> {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8080;
    let socket = SocketAddrV4::new(ip, port);
    let mut stream = TcpStream::connect(socket)?;
    let _bytes_written = stream.write(b"test\n");
    sleep(Duration::new(8, 0));
    let _bytes_written = stream.write(b"test\n");
    Ok(())
}
