use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::io::Write;
use std::io::Result;
use std::clone::Clone;

pub struct NetworkPrinter {
    name: String,
    stream: TcpStream,
    ip: String,
    port: u16,
    buffer: [u8; 128]
}

pub trait Printer {
    fn new(name: String, ip: String, port: u16) -> Self;
    fn feed(&mut self, lines: u8);
    fn cut(&mut self);
}

impl Printer for NetworkPrinter {
    fn new(name: String, ip:String, port: u16) -> NetworkPrinter {
        let mut stream = TcpStream::connect((ip.clone(),port)).expect("Couldn't connect to the server!");

        let buffer = [0 as u8; 128];
        return NetworkPrinter{
            name,
            stream,
            ip,
            port,
            buffer
        }
    }

    fn feed(&mut self, lines: u8){
        let cmd: [u8; 3] = [0x1B, 0x64, lines];
        self.stream.write(&cmd);
    }

    fn cut(&mut self){
        let cmd: [u8; 3] = [0x1D, 0x56, 0x01];
        self.stream.write(&cmd);
    }
}