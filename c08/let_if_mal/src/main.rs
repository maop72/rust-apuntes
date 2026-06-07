const ULTIMO_PUERTO_SISTEMA: u32 = 1023;

fn main() {
    let puerto = 8080;

    let categoria = if puerto <= ULTIMO_PUERTO_SISTEMA {
        "sistema" ;  // ¡MAL! Si acaba en ; ya no es una expresión
    } else {
        2;  // ¡MAL! El tipo es distinto al de la rama "then"
    };

    println!("{} es puerto de {}", puerto, categoria);
}

