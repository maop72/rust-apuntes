const TEMP_CALIENTE_X64: u32 = 70;
const TEMP_M_CALIENTE_X64: u32 = 85;
const TEMP_CRITICA_X64: u32 = 95;

const TEMP_CALIENTE_ARM: u32 = 60;
const TEMP_M_CALIENTE_ARM: u32 = 75;
const TEMP_CRITICA_ARM: u32 = 85;

fn main() {
    let es_arm = true;
    let temp_cpu = 78;

    if es_arm {
        if temp_cpu < TEMP_CALIENTE_ARM {
            println!("temp normal");
        } else if temp_cpu < TEMP_M_CALIENTE_ARM {
            println!("CPU caliente");
        } else if temp_cpu < TEMP_CRITICA_ARM {
            println!("CPU muy caliente");
        } else {
            println!("temp crítica");
        }
    } else {
        if temp_cpu < TEMP_CALIENTE_X64 {
            println!("temp normal");
        } else if temp_cpu < TEMP_M_CALIENTE_X64 {
            println!("CPU caliente");
        } else if temp_cpu < TEMP_CRITICA_X64 {
            println!("CPU muy caliente");
        } else {
            println!("temp crítica");
        }
    }
}
