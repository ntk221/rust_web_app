use std::net::TcpStream;

fn main() {
    let _sttream = TcpStream::connect("localhost:3000").unwrap();
}
