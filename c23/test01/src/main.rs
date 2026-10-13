fn producto(a: i32, b: i32) -> i32 {
    a * b
}

#[test]
fn producto_positivos() {
    let p = producto(3, 4);
    assert_eq!(p, 12);
    assert!(p > 0);
}

#[test]
fn producto_negativos() {
    let p = producto(-3, -4);
    assert_eq!(p, 12);
    assert!(p > 0);
}

fn main() {
    let a = -2;
    let b = 7;
    println!("{} * {} = {}", a, b, producto(a, b));
}
