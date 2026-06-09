fn main() {
    let puertos = ["22/tcp", "80/tcp", "443/tcp"];

    for i in 0..puertos.len() {
        println!("{}", puertos[i]);
    }
}
