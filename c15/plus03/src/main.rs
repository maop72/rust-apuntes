fn main() {
    let s1 = String::from("¡Hola");
    let s2 = String::from(", mundo");
    let s3 = s1 + &s2;    // String y &String. Correcto.

    // println!("s1:{}", s1);   // s1 se ha consumido. Ya no existe.
    println!("s2:{}", s2);
    println!("s3:{}", s3);

    let s3 = s3 + "!";     // String y cadena literal (&str). Correcto.
    println!("s3:{}", s3);
}
