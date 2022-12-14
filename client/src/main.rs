use std::io::Write;

fn main() {
    // socket connect to 127.0.0.1:8000 all at once
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8000").unwrap();

    loop {
        // write
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        stream.write(input.trim().as_bytes()).unwrap();
    }
}
