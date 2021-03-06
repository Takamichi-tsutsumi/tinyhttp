extern crate tokio;
extern crate http;

use tokio::io;
use tokio::prelude::*;
use tokio::net::{TcpStream, TcpListener};
use http::{Response, StatusCode};

fn main() {
    let addr = "127.0.0.1:3030".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
                      process(socket);
                      Ok(())
                  })
        .map_err(|err| {
                     println!("IO error: {}", err);
                 });

    tokio::run(server);
}


fn process(socket: TcpStream) {
    let (tx, rx) = socket.split();
    let res = Response::builder()
        .status(StatusCode::OK)
        .body(())
        .unwrap();

    let connection = io::write_all(encode(res)).then(|response| {
                                                         println!("wrote message; success={:?}",
                                                                  response.is_ok());
                                                         Ok(())
                                                     });

    tokio::spawn(connection);
}

fn encode(res: Response<()>) -> Vec<u8> {
    format!("HTTP/1.1 {}\r\n\n<html><body><h1>Hello from Rust</h1></body></html>",
            res.status())
            .into_bytes()
}
