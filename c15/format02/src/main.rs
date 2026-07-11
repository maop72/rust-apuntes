fn main() {
    let s1 = String::from("Hola");
    let s2 = String::from("mundo");

    let saludo = format!("!{}, {}!", s1, s2);

    println!("{}",s1);
    println!("{}",s2);
    println!("{}",saludo);
}
