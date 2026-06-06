fn sumar(a: i32, b: i32) -> i32 { 
    return a + b;   // Raro en Rust, pero legal.
}

fn main() { 
    let resultado = sumar(2, 3);
    println!("{}",resultado);
}
