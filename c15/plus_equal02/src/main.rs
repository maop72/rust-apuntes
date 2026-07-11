fn main() {
    let mut s1 = String::from("¡Hola");
    let s2 = String::from(", mundo!");

    s1 += &s2;

    println!("s1:{}",s1);
    println!("s2:{}",s2);
}
