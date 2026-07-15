fn main() {
    let s = "hola-hola";
    let v = ["hola", "mundo"];

    println!("{}", s); // Forma general. ok.
    println!("{s}");   // Forma abreviada. Aquí, ok.
    //println!("{v[0]}"); // ¡Error! La forma abreviada no es válida
    println!("{}", v[0]); // Forma general. ok.
}
