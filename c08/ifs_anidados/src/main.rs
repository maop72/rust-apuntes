const TEMP_ALTA_X64: u32 = 70;
const TEMP_M_ALTA_X64: u32 = 85;
const TEMP_CRITICA_X64: u32 = 95;

const TEMP_ALTA_ARM: u32 = 60;
const TEMP_M_ALTA_ARM: u32 = 75;
const TEMP_CRITICA_ARM: u32 = 85;

fn main() {
    let es_arm = true;
    let temp_cpu = 78;

    if es_arm {
        if temp_cpu < TEMP_ALTA_ARM {
            println!("temp normal");
        } else if temp_cpu < TEMP_M_ALTA_ARM {
            println!("CPU ALTA");
        } else if temp_cpu < TEMP_CRITICA_ARM {
            println!("CPU muy ALTA");
        } else {
            println!("temp crítica");
        }
    } else {
        if temp_cpu < TEMP_ALTA_X64 {
            println!("temp normal");
        } else if temp_cpu < TEMP_M_ALTA_X64 {
            println!("CPU ALTA");
        } else if temp_cpu < TEMP_CRITICA_X64 {
            println!("CPU muy ALTA");
        } else {
            println!("temp crítica");
        }
    }
}
