fn main() {
    let puerto = 443;

    match puerto {
        22 => println!("SSH"),
        25 => println!("SMTP"),
        53 => println!("DNS"),
        80 => println!("HTTP"),
        443 => println!("HTTPS"),
        _ => println!("Servicio desconocido"),
    }
}
