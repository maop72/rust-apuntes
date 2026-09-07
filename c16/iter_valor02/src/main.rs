fn main() {
    let puertos = [
        String::from("HTTP"),
        String::from("HTTPS"),
        String::from("SSH"),
    ];
    println!("{:?}", puertos);

    for puerto in puertos.into_iter() {
        println!("Puerto: {}", puerto);
    }
//    println!("{:?}", puertos); // Error
}
