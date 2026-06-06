const TEMPERATURA_FRIA: u32 = 40;
const TEMPERATURA_NORMAL: u32 = 70;
const TEMPERATURA_CALIENTE: u32 = 85;

fn main() {
    let temperatura_cpu = 72;

    if temperatura_cpu < TEMPERATURA_FRIA {
        println!("CPU fría");
    } else if temperatura_cpu < TEMPERATURA_NORMAL {
        println!("Temperatura normal");
    } else if temperatura_cpu < TEMPERATURA_CALIENTE {
        println!("CPU caliente");
    } else {
        println!("Advertencia: temperatura excesiva");
    }
}
