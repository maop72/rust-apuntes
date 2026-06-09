fn main() {
    for i in 1..5 {
        // De 1 a 4
        print!("{} ", i);
    }
    println!();

    for i in 1..=5 {
        // De 1 a 5
        print!("{} ", i);
    }
    println!();

    for i in (1..=5).rev() {
        // de 5 a 1
        print!("{} ", i);
    }
    println!();
}
