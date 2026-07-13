fn main() {
    let saludo = String::from("Hola")
        .to_uppercase()
        .replace("HOLA", "Buenos días");

    println!("{}", saludo);
}
