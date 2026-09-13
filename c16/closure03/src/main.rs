fn main() {
    let puerto = 443;

    let es_seguro = |numero: &i32| *numero == 443;
    println!("{}", es_seguro(&puerto));
}
