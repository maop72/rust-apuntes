fn main() {
    let t1 = (10, String::from("hola"));
    let t2 = t1; 
  
    // println!("{:?}", t1);  // t1 ya no se puede usar

    println!("{:?}", t2);

    let texto = String::from("mundo");
    let r1 = &texto;
    let r2 = r1;

    // r1 sigue siendo válida 
    println!("{}", r1);
    println!("{}", r2);
}
