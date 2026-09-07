fn main() {
    let mut numero = "125".parse::<i32>();
    println!("{:?}", numero);   // ok

    numero = "abc".parse::<i32>();
    println!("{:?}", numero);  // Error (recuperable). 
}

