use std::array::from_fn;
const TABLE_SIZE: usize = 5;

fn main() {
    let mut tabla: [Option<String>; TABLE_SIZE] = from_fn(|_| None);
    println!("{:?}", tabla);

    tabla[0] = Some(String::from("Hola"));
    println!("{:?}", tabla);
}
