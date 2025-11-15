use pnet::datalink::{self, NetworkInterface};
fn main() {
    let interface = NetworkInterface::from_name("eth0").unwrap();
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => panic!("Не удалось открыть интерфейс"),
    };

    loop {
        match rx.next() {
            Ok(packet) => println!("Пакет: {:?}", packet),
            Err(e) => eprintln!("Ошибка: {}", e),
        }
    }
}
