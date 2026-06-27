fn sumar(a: i32, b: i32) -> i32 { 
    a + b;   // ¡Error! Al añadir ';', la expresión se convierte en
             // una sentencia y deja de ser la expresión final del bloque.
}

fn main() { 
    let resultado = sumar(2, 3);
    println!("{}",resultado);
}
