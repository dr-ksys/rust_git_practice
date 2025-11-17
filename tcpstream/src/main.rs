use std::net::TcpStream;

fn main() {
    let target = "127.0.0.1";
    for port in 1..=1024 {
        if TcpStream::connect((target, port)).is_ok(){
            println!("[+] Порт {} открыт", port);
        }   
    }
}
