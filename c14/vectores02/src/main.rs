fn main() {
    let numeros = vec![10, 20, 30, 40];

    println!("Número de elementos: {}", numeros.len());

    for n in &numeros {   // Recorremos los elementos, sin índice 
        println!("{}", n);
 
    }

    for i in 0..numeros.len() { // Recorremos los elementos, usando el índice  
        println!("Elemento {}: {}", i, numeros[i]);
    }
}

