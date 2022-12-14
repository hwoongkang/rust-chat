use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    thread,
};

fn main() {
    // socket connect to 127.0.0.1:8000 all at once
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8000").unwrap();

    let mut writer = BufWriter::new(stream.try_clone().unwrap());
    let mut reader = BufReader::new(stream);
    thread::spawn(move || loop {
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        println!("{}", buf);
    });

    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        writer.write(buf.trim().as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
