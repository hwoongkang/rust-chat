use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4},
    sync::{mpsc, Arc},
    thread,
};

fn main() {
    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000);
    // listen
    let listener = std::net::TcpListener::bind(socket).unwrap();

    let (tx, rx) = mpsc::channel::<(String, String)>();

    thread::spawn(move || {
        for received in rx {
            println!("New message: {} from {}", received.1, received.0);
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let t = tx.clone();
                thread::spawn(move || {
                    println!("New connection from : {}", stream.peer_addr().unwrap());
                    // connection succeeded
                    loop {
                        let mut buf = [0; 1024];
                        match stream.read(&mut buf) {
                            Ok(_) => {
                                let s = std::str::from_utf8(&buf).unwrap();

                                t.send((
                                    stream.peer_addr().unwrap().to_string(),
                                    s.trim().to_string(),
                                ))
                                .expect("Could not send");
                            }
                            Err(e) => println!("failed to read from socket; err = {:?}", e),
                        }
                    }
                });
            }
            Err(sth) => {
                println!("Error! {:?}", sth);
            }
        }
    }
}
