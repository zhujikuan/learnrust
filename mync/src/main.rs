use std::io::prelude::*;

fn handle_conn(mut stream: std::net::TcpStream) {
    println!("new connection from {}", stream.peer_addr().unwrap());
    // 计时
    use std::time::Instant;
    let start = Instant::now();

    let mut buf = [0; 1024];
    loop {
        let n = stream.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        let s = std::str::from_utf8(&buf[..n]).unwrap();
        println!("recv: {}", s);
        stream.write(&buf[..n]).unwrap();
    }
    let elapsed = start.elapsed();
    println!("connection closed Time elapsed: {:?}", elapsed);
}

fn main() {
    let port = std::env::args().nth(1).unwrap();
    let port = port.parse::<u16>().unwrap();
    let addr = format!("0.0.0.0:{}", port);
    println!("listenon: {}", addr);

    let listener = std::net::TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        std::thread::spawn(|| {
            handle_conn(stream);
        });
    }
}
