use std::io;

fn main() {
    println!("Piensa un número del 1 al 100.");
    println!("Apúntalo pero no me lo digas.\n");

    let mut minimo = 1;
    let mut maximo = 100;
    let mut intentos = 0;

    loop {
        if minimo > maximo {
            println!("Te has equivocado en alguna respuesta.");
            break;
        }

        if minimo == maximo {
            println!("¡Tu número es {}!", minimo);
            println!("Lo he encontrado en {} intentos.", intentos);
            break;
        }

        let tentativo = (minimo + maximo) / 2;
        intentos += 1;

        println!(
            "Sé que el número está entre {} y {}. ¿{}?",
            minimo, maximo, tentativo
        );
        println!("1 - Sí, ese es.");
        println!("2 - No, es menor.");
        println!("3 - No, es mayor.");

        let mut respuesta = String::new();
        io::stdin()
            .read_line(&mut respuesta)
            .expect("Error al leer la respuesta");

        let respuesta: u32 = match respuesta.trim().parse() {
            Ok(n) if (1..=3).contains(&n) => n,
            _ => {
                println!("Respuesta no válida.");
                intentos -= 1;
                continue;
            }
        };

        match respuesta {
            1 => {
                println!("¡Lo he encontrado en {} intentos!", intentos);
                break;
            }
            2 => maximo = tentativo - 1,
            3 => minimo = tentativo + 1,
            _ => unreachable!(),
        }
    }
}
