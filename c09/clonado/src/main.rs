fn main() {
    let mut s1 = String::from("Hola");
    let mut s2 = s1.clone();

    println!("s1: {}", s1); // No hemos perdido acceso a s1
    println!("s2: {}", s2);

    s1.push_str(", soy s1"); // Ahora tenemos dos cadenas independientes
    s2.push_str(", soy s2");

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
