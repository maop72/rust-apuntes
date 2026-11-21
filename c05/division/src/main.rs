fn main() {
    let a = 7;
    let b = 2;

    let resultado1 = a / b;
    println!("División entera entera entre {} y {}: {}",a,b,resultado1);

    let resultado2 = a as f64 / b as f64;
    println!("División real entre {} y {}: {}",a,b,resultado2);
}
