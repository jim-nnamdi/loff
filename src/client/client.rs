use std::{
    io::{self, BufRead, BufReader, Error, Write},
    net::{SocketAddr, TcpStream},
};

const _NO_BYTES:usize = 0;

pub fn _loff_client(ip_address: SocketAddr) -> Result<(), Error> {
    let mut loff_stream = TcpStream::connect(ip_address).expect("could not connect to server");
    println!("{}", loff_stream.peer_addr()?);
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        io::stdin().read_line(&mut input).unwrap();
        let num_of_bytes_written = loff_stream.write(&input.as_bytes()).unwrap();
        if num_of_bytes_written <= _NO_BYTES { return Ok(());}
        let mut reader = BufReader::new(&loff_stream);
        reader.read_until(b'\n',&mut buffer).unwrap();
        println!("{}", std::str::from_utf8(&buffer).unwrap());
    }
}
