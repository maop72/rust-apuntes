fn main() {
    let mut protocolos = vec!["tcp", "ip"];
    protocolos[0] = "udp";
    println!("{:?}", protocolos);
}
