use std::io::{Read, Write};
use std::{net::TcpListener, net::TcpStream, error::Error};
use std::process;
use std::fs;
use rust_web_server::router::*;
fn main() {

    let listener = match TcpListener::bind("127.0.0.1:7878"){
        Ok(v) => v,
        Err(e) => {
            panic!("Error binding server! Error message: {:?}", e);
        }
    };


    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        handle_connection(&mut stream);
    }

}

fn handle_connection(stream: &mut TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let mut router =  Router::new("404.html");
    router.add_route("/", "index.html");

    let get = b"GET / HTTP/1.1\r\n";

    let file_path = if buffer.starts_with(get) {
        "/"
    }
    else{
        "someRandomShit"
    };

    let content = fs::read_to_string(router.get_route(file_path)).unwrap();

    let res = format!("HTTP/1.1 200 OK\r\nContent Length: {}\r\n\r\n{}", content.len(), content);
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
