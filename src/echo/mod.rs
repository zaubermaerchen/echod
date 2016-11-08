use std::net::TcpListener;
use std::thread;
use std::io;
mod session;

pub struct Service {
    address: String
}

impl Service {
    pub fn new(port: u32) -> Service {
    	Service {
            address: format!("0.0.0.0:{}", port)
        }
    }

    pub fn start(&self) -> io::Result<()> {
		let listener = try!(TcpListener::bind(self.address.as_str()));
        println!("started on {:?}", listener);
        for stream in listener.incoming() {
            let stream = try!(stream);
            thread::spawn(move || {
				session::start(stream).unwrap();
            });
        }
        Ok(())
    }
}