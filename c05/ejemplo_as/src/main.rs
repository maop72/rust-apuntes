fn main() {
    let x: i32 = 10;
    let y: f64 = x as f64;

    println!("Conversión del entero {} a real: {}", x,y);

    let a: f64 = 3.75;
    let b: i32 = a as i32;

    println!("Conversión de real {} a entero: {}", a,b);

    let c: i32 = 300;
    let d: u8 = c as u8;

    println!("Conversión del i32 {} a u8: {}",c,d);
}
