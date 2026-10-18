fn mayor(cad1: &str, cad2: &str) -> &str {   // ¡Error!
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
