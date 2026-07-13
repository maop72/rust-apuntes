fn main() {
    let mut numero = "125".parse::<i32>().unwrap();
    println!("{}", numero);

    numero = "hola".parse::<i32>().unwrap();
    println!("{}", numero);
}

