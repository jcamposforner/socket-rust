use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    println!("Client {:?}", stream);
                });
            }
        }
    }
}
