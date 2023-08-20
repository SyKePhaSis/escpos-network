#![allow(unused)]

mod lib;
use lib::{NetworkPrinter};

fn main(){
    let mut printer: NetworkPrinter = NetworkPrinter::new("Printer".to_string(),"192.168.123.100".to_string(),9100);
    let mut buff = [0 as u8; 128];
    printer.feed(0x10);
    printer.cut();
}
