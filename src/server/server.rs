use std::{io::{Error, Read, Write}, net::TcpStream};




pub fn _loff_server(mut stream: TcpStream) -> Result<(), Error>  {
  println!("reading data from : {}", stream.peer_addr()?);
  let mut buffer:[u8; 512] = [0; 512];
  let mut vector:Vec<u8> = Vec::new();
  loop{
  let loff_stream = stream.read(&mut buffer)?;
  vector.extend_from_slice(&buffer);
  if loff_stream == 0 {return Ok(());}
  let _ = stream.write(&vector);
  println!("{}", std::str::from_utf8(&vector).expect("could not read data"));
  }
}