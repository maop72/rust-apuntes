mod geometria {
    pub fn area_cuadrado(lado: f64) -> f64 {
        lado * lado
    }

    pub fn area_rectangulo(base: f64, altura: f64) -> f64 {
        base * altura
    }
}

fn main() {
    println!(
        "Área del cuadrado: {}",
        geometria::area_cuadrado(5.0)
    );

    println!(
        "Área del rectángulo: {}",
        geometria::area_rectangulo(3.0, 4.0)
    );
}
