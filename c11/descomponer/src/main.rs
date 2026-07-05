struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn descomponer(self) -> (u32, u32) {
        (self.ancho, self.alto)
    }
}

fn main() {
    let r = Rectangulo {
        ancho: 20,
        alto: 10,
    };

    let t = r.descomponer();

    println!("{:?}", t);

    // println!("{}", r.ancho);  // Esto sería un error, r
                                 // ya no existe
}
