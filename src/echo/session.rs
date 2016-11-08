use std::net::TcpStream;
use std::io;

pub fn start(mut stream: TcpStream) -> io::Result<()> {
	let address = try!(stream.peer_addr());
	println!("{:?} connected.", address);
	let mut stream_clone = try!(stream.try_clone());
	try!(io::copy(&mut stream_clone, &mut stream));
	println!("{:?} closed.", address);
	return Ok(());
}