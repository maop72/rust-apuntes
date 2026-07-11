fn main() {
    let s1 = "Hola";
    let s2 = ", mundo.";

    let s3 = s1 + s2;   // ¡Error!. + no puede concatenar dos &str

    println!("s1:{}", s1);
    println!("s2:{}", s2);
    println!("s3:{}", s3);
}
