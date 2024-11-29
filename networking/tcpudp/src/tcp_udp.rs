

/*
    all the networking.
    std::net

    read and write traits  ( also being used for sockets. )  
    std::io

    IpAddr -> generic ip address

    SocketAddr ->  ip:host

    TcpListener and TcpStrem for communication over TCP.

    UdpSocket for UDP

    the standard library does not provide any APIs to deal with the
        network stack at a lower level


    libpnet which provides a set of APIs for lower-level networking
    net2
    socket2

    tokio ecosystem of crates for writing high-performance networking applications that do not require fine-grained control of socket semantics
 */

 use std::net::{TcpListener, TcpStream};
 use std::thread;
 
 use std::io::{Read, Write, Error};
 
 fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
     println!("Incoming connection from: {}", stream.peer_addr()?);
     let mut buf = [0; 512];
     loop {
         let bytes_read = stream.read(&mut buf)?;
         if bytes_read == 0 {return Ok(()); }
         stream.write(&buf[..bytes_read])?;
     }
 }
 

 pub fn _main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could no bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
                });
            }
        }
    }
 }