fn main() {
    let puerto = 443;

    let servicio = match puerto {
        22 => "SSH",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        443 => "HTTPS",
        _ => "Desconocido",
    };

    println!("Servicio: {}", servicio);
}
