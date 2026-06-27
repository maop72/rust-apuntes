#[derive(Debug)]
struct EntradaNat {
    ip_privada: String,
    puerto_privado: u16,
    ip_publica: String,
    puerto_publico: u16,
    protocolo: String,
}


fn escribe_entrada(entrada: &EntradaNat){
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
    let entrada = EntradaNat {
        ip_privada: String::from("192.168.1.10"),
        puerto_privado: 52341,
        ip_publica: String::from("203.0.113.1"),
        puerto_publico: 40001,
        protocolo: String::from("TCP"),
    };

    escribe_entrada(&entrada);
}

