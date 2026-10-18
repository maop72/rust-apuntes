use pnet::datalink;

fn main() {
    let interfaces = datalink::interfaces();

    for iface in interfaces {
        println!("{}", iface.name);
    }
}
