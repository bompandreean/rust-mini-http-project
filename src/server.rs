use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

use crate::http::{ParseErrors, Request, StatusCode};
use crate::http::response::Response;

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, error: &ParseErrors) -> Response{
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // dbg!(request);
                                    //
                                    // Response::new(StatusCode::Ok, Some("<h1> IT works !!! </h1>".to_string()))
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    // println!("Failed to parse request, {}", e);
                                    //
                                    // Response::new(StatusCode::BadRequest, None)
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e)
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}