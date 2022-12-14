use std::{
    io::Read,
    net::{Ipv4Addr, SocketAddrV4},
    thread,
};

fn main() {
    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8000);
    // listen
    let listener = std::net::TcpListener::bind(socket).unwrap();
    // accept

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    // connection succeeded
                    loop {
                        let mut buf = [0; 1024];
                        match stream.read(&mut buf) {
                            Ok(_) => {
                                let s = std::str::from_utf8(&buf).unwrap();
                                println!("we are here, {}", s);
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

    // read
    loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let mut buf = [0; 1024];
                match stream.read(&mut buf) {
                    Ok(_) => {
                        let s = std::str::from_utf8(&buf).unwrap();
                        println!("we are here, {}", s);
                    }
                    Err(e) => println!("failed to read from socket; err = {:?}", e),
                }
            }
            Err(e) => println!("failed to accept socket; err = {:?}", e),
        }
    }
}
