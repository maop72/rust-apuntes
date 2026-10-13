use std::time::Instant;

fn main() {
    let inicio = Instant::now();

    let mut suma: u64 = 0;
    for i in 0..1_000_000 {
        suma += i;
    }

    let intervalo = inicio.elapsed();

    // Error: Instant no implementa Display, por lo que no admite el formato {}
    // println!("Instante inicial: {}", inicio);

    println!("Instante inicial: {:?}", inicio);

    println!("Tiempo transcurrido:");
    println!("Nanosegundos: {}", intervalo.as_nanos());
    println!("Microsegundos: {}", intervalo.as_micros());
    println!("Milisegundos: {}", intervalo.as_millis());
    println!("Segundos: {}", intervalo.as_secs_f64());
    println!("Suma: {}", suma);
}
