const TEMPERATURA_MAXIMA_CPU: u32 = 80;

fn main() {
    let temperatura_cpu = 72;

    if temperatura_cpu > TEMPERATURA_MAXIMA_CPU {
        println!("Aviso: temperatura excesiva");
    }

    println!("Monitorización completada");
}
