use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contenido = fs::read_to_string("/tmp/mensaje.txt")?;
    println!("{}", contenido);
    Ok(())
}

