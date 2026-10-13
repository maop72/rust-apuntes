fn mayor<'a>(cad1: &'a str, cad2: &'a str) -> &'a str {
    if cad1.len() >= cad2.len() {
        cad1
    } else {
        cad2
    }
}

fn main() {
    let s1 = String::from("Hola mundo");
    let resultado;
    {
        let s2 = String::from("Adiós");
        resultado = mayor(&s1, &s2);
        println!("{}", resultado);   
    }
}
