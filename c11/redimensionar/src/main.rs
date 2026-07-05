struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn redimensionar(&mut self, ancho: u32, alto: u32) {
        self.ancho = ancho;
        self.alto = alto;
    }
}

fn main() {
    let mut r = Rectangulo {
        ancho: 20,
        alto: 10,
    };

    r.redimensionar(40, 25);

    println!("{} × {}", r.ancho, r.alto);
}
