use pnet::util::MacAddr;
use std::array::from_fn;

const TABLE_SIZE: usize = 5;

#[derive(Debug)]
struct EntradaTabla {
    mac: MacAddr,
    ip: String,
}

type TipoTabla = [Option<EntradaTabla>; TABLE_SIZE];

fn encuentra_hueco(tabla: &TipoTabla) -> Option<usize> {
    tabla.iter().position(|x| x.is_none())
}

fn main() {
    let mut tabla: TipoTabla = from_fn(|_| None);
    println!("Contenido del array:\n{:?}", tabla);

    let hueco = encuentra_hueco(&tabla);

    match hueco {
        Some(h) => {
            println!("Hueco en {}", h);

            let entrada = EntradaTabla {
                mac: MacAddr::new(0x02, 0xa0, 0xc4, 0xb2, 0xd0, 0x02),
                ip: String::from("192.168.1.1"),
            };

            tabla[h] = Some(entrada);
        }
        None => println!("Tabla llena"),
    }

    println!("Contenido del array:\n{:?}", tabla);
}
