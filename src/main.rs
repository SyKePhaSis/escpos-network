#![allow(unused)]

mod lib;
use lib::escpos::NetworkPrinter;
use std::io::{self, BufRead};
use std::string::String;

fn main(){
    let mut printer: NetworkPrinter = NetworkPrinter::new("Printer".to_string(),"192.168.123.100".to_string(),9100);
    printer.initialize();
    printer.print_contents_txt("test.txt".to_string(),true);
    printer.feed(0x05);
    printer.cut();
}
