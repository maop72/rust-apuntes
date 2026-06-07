const ULTIMO_PUERTO_SISTEMA: u32 = 1023;

fn main() {
    let puerto = 8080;

    let categoria = if puerto <= ULTIMO_PUERTO_SISTEMA {
        "sistema"
    } else {
        "usuario"
    };

    println!("{} es puerto de {}", puerto, categoria);
}

