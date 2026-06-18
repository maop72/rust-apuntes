struct EntradaNat {
    ip_privada: String,
    puerto_privado: u16,
    ip_publica: String,
    puerto_publico: u16,
    protocolo: String,
}

fn crea_entrada(
    ip_privada: String,
    puerto_privado: u16,
    ip_publica: String,
    puerto_publico: u16,
    protocolo: String,
) -> EntradaNat {
    EntradaNat {
        ip_privada,
        puerto_privado,
        ip_publica,
        puerto_publico,
        protocolo,
    }
}

fn escribe_entrada(entrada: &EntradaNat) {
    println!(
        "{}:{} -> {}:{} ({})",
        entrada.ip_privada,
        entrada.puerto_privado,
        entrada.ip_publica,
        entrada.puerto_publico,
        entrada.protocolo
    );
}

fn main() {
    let entrada_tcp = crea_entrada(
        String::from("192.168.1.10"),
        52341,
        String::from("203.0.113.1"),
        40001,
        String::from("TCP"),
    );

    let entrada_udp = EntradaNat {
        protocolo: String::from("UDP"),
        ip_privada: entrada_tcp.ip_privada.clone(),
        ip_publica: entrada_tcp.ip_publica.clone(),
        ..entrada_tcp
    };

    escribe_entrada(&entrada_tcp);
    escribe_entrada(&entrada_udp);
}
