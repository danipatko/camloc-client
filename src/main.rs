#![allow(unused)]

use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 1111);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);

    let (mut tcp_stream, addr) = listener.accept()?; // block  until requested
    println!("Connection received! {:?} is sending data.", addr);

    loop {
        tcp_stream.write("augh ".as_bytes())?;
    }

    // let mut input = String::new();
    // let _ = tcp_stream.read_to_string(&mut input)?;
    // println!("{:?} says {}", addr, input);
    // Ok(())
}
