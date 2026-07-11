fn main() {
    let direccion = "192.168.1.5";
    let puerto = 22;

    let s = format!("Endpoint: {}:{}", direccion, puerto);

    println!("{}",s);
}
