pub mod escpos {

    //NETWORK COMMUNICATION
    use std::net::{TcpStream, Shutdown};
    use std::io::Write;
    use std::clone::Clone;
    
    //FOR BUFFERS
    use std::vec::Vec;
    use std::string;
    
    //FOR FILES
    use std::io;
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    use std::path::Path;

    pub struct NetworkPrinter{
        name: String,
        stream: TcpStream,
        ip: String,
        port: u16,
        buffer: Vec<u8>,
        settings: u8
    }
    
    impl NetworkPrinter{
        const BUFFER_SIZE: usize = 1024;
    
        pub fn new(name: String, ip:String, port: u16) -> NetworkPrinter {
            let stream = TcpStream::connect((ip.clone(),port)).expect("Couldn't connect to the server!");
    
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
    
        //BASIC PRIVATE FUNCTIONS
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
        //
        
        // INITIALIZATION && SETTINGS    
        pub fn initialize(&mut self) {
            let cmd: Vec<u8> = vec![0x1B, 0x40];
            self.add_to_buffer(cmd);
            self.send_buffer();
            self.send_settings();
            println!("[INFO]Printer Initialized");
        }
    
        fn send_settings(&mut self) {
            let cmd: Vec<u8> = vec![0x1B, 0x21, self.settings];
            self.add_to_buffer(cmd);
            self.send_buffer();
            println!("[INFO] Printer Settings Set");
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
            let cmd: Vec<u8> = vec![0x1B, 0x74, value];
            self.add_to_buffer(cmd);
            self.send_buffer();
        }
        //
        
        // COMMANDS
        pub fn print(&mut self, text: String, lines: u8){
            let cmd: Vec<u8> = vec![0x1B, 0x4A, lines];
            self.add_to_buffer(cmd);
            self.add_to_buffer((*text.as_bytes()).to_vec());
            self.send_buffer();
        }
    
        pub fn feed(&mut self, lines: u8){
            let cmd: Vec<u8> = vec![0x1B, 0x64, lines];
            self.add_to_buffer(cmd);
            self.send_buffer();
            println!("[FEED] Printer Fed {} Lines", lines);
        }
    
        pub fn cut(&mut self){
            let cmd: Vec<u8> = vec![0x1D, 0x56, 0x01];
            self.add_to_buffer(cmd);
            self.send_buffer();
        }
        //
        
        //PRINTING FROM FILES
        pub fn print_contents_txt(&mut self, name: String, cut: bool){
            let path = Path::new(&name);
            let f = File::open(path);
            let reader = BufReader::new(f.unwrap());
            for line in reader.lines(){
                self.print(line.unwrap(),0x01);
            }
            self.feed(0x0A);
        }   
        //
        
        //IMPLIMENTING 'CLI'
            
        //
    }
    
    impl Drop for NetworkPrinter {
        fn drop(&mut self) {
            self.stream.shutdown(Shutdown::Both);
        }
    }
}