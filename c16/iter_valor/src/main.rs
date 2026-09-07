fn main() {
    let puertos = [80, 443, 8080];

    for puerto in puertos.into_iter() {
        println!("Puerto: {}", puerto);
    }
    println!("{:?}", puertos);
}
