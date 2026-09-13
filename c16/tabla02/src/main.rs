use pnet::util::MacAddr;
use std::array::from_fn;

const TABLE_SIZE: usize = 5;

#[derive(Debug)]
struct EntradaTabla {
    mac: MacAddr,
    ip: String,
}

type TipoTabla = [Option<EntradaTabla>; TABLE_SIZE];

fn busca_ip(tabla: &TipoTabla, ip: &str) -> Option<MacAddr> {
    for x in tabla.iter() {
        match x {
            Some(valor) => {
                //println!("mac:{} ip:{}",valor.mac,valor.ip);
                if valor.ip == ip {
                    return Some(valor.mac);
                }
            }
            None => {}
        }
    }
    None
}

fn main() {
    let mut tabla: TipoTabla = from_fn(|_| None);

    let entrada = EntradaTabla {
        mac: MacAddr::new(0x02, 0xa0, 0xc4, 0xb2, 0xd0, 0x02),
        ip: String::from("192.168.1.1"),
    };

    tabla[0] = Some(entrada);
    println!("Contenido del array:\n{:?}\n", tabla);

    let ip = "192.168.1.1";
    println!("Busquemos {}", ip);
    println!("\t{:?}", busca_ip(&tabla, ip));

    let ip = "192.168.1.2";
    println!("Busquemos {}", ip);
    println!("\t{:?}", busca_ip(&tabla, ip));
}
