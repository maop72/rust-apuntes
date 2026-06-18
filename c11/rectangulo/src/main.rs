struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }
}

fn main() {
    let r = Rectangulo {
        ancho: 30,
        alto: 20,
    };

    println!("Área: {}", r.area());
}

