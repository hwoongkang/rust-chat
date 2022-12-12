fn main() {
    // socket connect to 127.0.0.1:8000 all at once
    let stream = std::net::TcpStream::connect("127.0.0.1:8000").unwrap();
    println!("{:?}", stream);
}
