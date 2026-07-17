fn division(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("División por cero");
    }
    a / b
}

#[test]
#[should_panic]
fn division_por_cero() {
    division(10, 0);
}

fn main() {
}
