#[derive(Debug)]
struct Conexion {
    ip: String,
    puerto: u16,
    activa: bool,
}

impl Conexion {
    fn new(ip: String, puerto: u16) -> Conexion {
        Conexion {
            ip,
            puerto,
            activa: false,
        }
    }

    fn conectar(&mut self) {
        self.activa = true;
    }

    fn desconectar(&mut self) {
        self.activa = false;
    }

    fn esta_activa(&self) -> bool {
        self.activa
    }

    fn direccion(&self) -> String {
        self.ip.clone()
    }

    fn puerto(&self) -> u16 {
        self.puerto
    }
}

fn main() {
    let mut c = Conexion::new(String::from("193.147.71.64"), 443);
    println!("Conexión recién creada");
    println!("{:?}", c);
    println!("Conectamos");
    c.conectar();
    println!("{:?}", c);
    println!("Desconectamos");
    c.desconectar();
    println!("{:?}", c);
    println!("Activa: {}", c.esta_activa());
    println!("Endpoint: {}:{}", c.direccion(), c.puerto());
}
