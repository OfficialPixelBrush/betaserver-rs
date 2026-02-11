use std::net::TcpStream;

fn receive<T>(mut stream: TcpStream) -> T {

}

fn send<T>(value: T, mut stream: TcpStream) {
    stream.write_all(value.serialize()).unwrap();
}