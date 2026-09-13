fn main() {
    let fqdn = "dns.google";
    let ip = "8.8.8.8";

    let mensaje = format!("La dirección de {} es {}", fqdn, ip);
    println!("{}", mensaje);
}
