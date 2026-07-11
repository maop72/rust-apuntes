fn main() {
    let s1 = String::from("Hola");
    let s2 = String::from(", mundo");

    let s3 = s1 + s2;  // ¡Error!. El segundo operando no puede ser un String

    println!("s1:{}", s1);
    println!("s2:{}", s2);
    println!("s3:{}", s3);
}
