fn main() {
    let x: Option<i32> = Some(42);

    println!("{}", x.is_some()); // true
    println!("{}", x.is_none()); // false
}
