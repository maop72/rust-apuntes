fn sumar(a: i32, b: i32) -> i32 { 
    a + b
};    // ¡MAL! El cuerpo de una función no puede llevar ';' a continuación

fn main() { 
    let resultado = sumar(2, 3);
    println!("{}",resultado);
}
