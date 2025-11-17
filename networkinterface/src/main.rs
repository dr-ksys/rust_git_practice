use pnet::datalink::{self, NetworkInterface};

fn main() {
    let interface = NetworkInterface::from_name("eth0").unwrap();
    let (_tx, mut rx) = datalink::channel(&interface, Default::default()).unwrap();

    loop {
        match rx.next() {
            Ok(packet) => println!("Пакет: {:?}", packet),
            Err(e) => eprintln!("Ошибка: {}", e),
        
        }
    }
}
