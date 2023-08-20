use std::net::{TcpStream, ToSocketAddrs, Shutdown};
use std::io::{Write, Result};
use std::clone::Clone;
use std::vec::Vec;

const BUFFER_SIZE: usize = 1024;

pub struct NetworkPrinter {
    name: String,
    stream: TcpStream,
    ip: String,
    port: u16,
    buffer: Vec<u8>,
    settings: u8
}

impl NetworkPrinter {
    pub fn new(name: String, ip:String, port: u16) -> NetworkPrinter {
        let mut stream = TcpStream::connect((ip.clone(),port)).expect("Couldn't connect to the server!");

        let buffer: Vec<u8> = Vec::new();
        let settings: u8 = 0x00;
        return NetworkPrinter{
            name,
            stream,
            ip,
            port,
            buffer,
            settings
        }
    }

    fn add_to_buffer(&mut self, buf: Vec<u8>) {
        self.buffer.extend(buf);
    }

    fn send_buffer(&mut self){
        self.stream.write(&self.buffer);
        self.clear_buffer();
    }

    fn clear_buffer(&mut self){
        self.buffer.clear();
    }

    pub fn initialize(&mut self) {
        let mut cmd: Vec<u8> = vec![0x1B, 0x40];
        self.add_to_buffer(cmd);
        self.send_buffer();
        self.send_settings();
    }

    fn send_settings(&mut self) {
        let mut cmd: Vec<u8> = vec![0x1B, 0x21, self.settings];
        self.add_to_buffer(cmd);
        self.send_buffer();
    }

    pub fn underline(&mut self) {
        self.settings = self.settings ^ 0x80;
        self.send_settings();
    }

    pub fn emphasize(&mut self){
        self.settings = self.settings ^ 0x08;
        self.send_settings();
    }

    pub fn double_height(&mut self){
        self.settings = self.settings ^ 0x10;
        self.send_settings();
    }

    pub fn double_width(&mut self){
        self.settings = self.settings ^ 0x20
    }

    pub fn character_font(&mut self){
        self.settings = self.settings ^ 0x01;
        self.send_settings();
    }

    pub fn select_codetable(&mut self, value: u8){
        let mut cmd: Vec<u8> = vec![0x1B, 0x74, value];
        self.add_to_buffer(cmd);
        self.send_buffer();
    }

    pub fn feed(&mut self, lines: u8){
        let cmd: Vec<u8> = vec![0x1B, 0x64, lines];
        self.add_to_buffer(cmd);
        self.send_buffer();
    }

    pub fn cut(&mut self){
        let cmd: Vec<u8> = vec![0x1D, 0x56, 0x01];
        self.add_to_buffer(cmd);
        self.send_buffer();
    }

}

impl Drop for NetworkPrinter {
    fn drop(&mut self) {
        self.stream.shutdown(Shutdown::Both);
    }
}