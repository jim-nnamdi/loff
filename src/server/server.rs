use std::{
    io::{self, Error, Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn _loff_server(mut stream: TcpStream) -> Result<(), Error> {
    println!("reading data from : {}", stream.peer_addr()?);
    let mut buffer: [u8; 512] = [0; 512];
    let mut vector: Vec<u8> = Vec::new();
    loop {
        let loff_stream = stream.read(&mut buffer)?;
        vector.extend_from_slice(&buffer);
        if loff_stream == 0 {
            return Ok(());
        }
        let _ = stream.write(&vector);
        println!(
            "{}",
            std::str::from_utf8(&vector).expect("could not read data")
        );
    }
}
pub fn _core_loff_server(bind_addr: &str) {
    let listener = TcpListener::bind(bind_addr).expect("msg");
    let streams = listener.incoming();
    for stream in streams {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    _loff_server(stream).unwrap_or_else(|error| {
                        if error.kind() == io::ErrorKind::AddrNotAvailable {
                            println!("address already in use");
                        }
                    })
                });
            }
            Err(estream) => {
              // todo: proper error handling
                println!("{}", estream)
            }
        }
    }
}
