const ULTIMO_PUERTO_SISTEMA: u32 = 1023;

fn main() {
    let puerto = 8080;

    if puerto <= ULTIMO_PUERTO_SISTEMA {
        println!("{} es puerto de sistema", puerto);
    } else {
        println!("{} es puerto de usuario", puerto);
    }
}

