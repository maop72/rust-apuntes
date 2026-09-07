fn main() {
    let puertos = [80, 443, 8080];
    let puerto_buscado = 443;

    for puerto in puertos.iter(){  // Iterador explícito
        if *puerto == puerto_buscado {    
            println!("Encontrado el puerto {}", *puerto);
        }
    }

    for puerto in &puertos {      // Sintaxis simplificada
        if *puerto == puerto_buscado {    
            println!("Encontrado el puerto {}", *puerto);
        }
    }
}
