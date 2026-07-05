struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn nuevo(ancho: u32, alto: u32) -> Self {  // sin parámetro 'self'
        Self { ancho, alto }
    }
}

fn main() {
    let r = Rectangulo::nuevo(20, 10);  // tipo::método

    println!("{} × {}", r.ancho, r.alto);
}

