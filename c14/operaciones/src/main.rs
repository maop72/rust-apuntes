fn main() {
    let mut protocolos = vec!["HTTP", "HTTPS", "FTP"];

    protocolos.push("SSH"); 
    println!("{:?}", protocolos);

    protocolos.insert(1, "SMTP");
    println!("{:?}", protocolos);

    protocolos.remove(2);
    println!("{:?}", protocolos);

    println!("Número de protocolos: {}", protocolos.len());
    println!("¿Contiene HTTPS? {}", protocolos.contains(&"HTTPS"));
  
    let ultimo = protocolos.pop();
    println!("Extraemos {:?}", ultimo);
    println!("Queda {:?}", protocolos);

    protocolos.clear();
    println!("¿Vector vacío? {}", protocolos.is_empty());
} 
