#[derive(Debug)] 
enum EstadoConexion {
    Free,
    Listening,
    Connecting,
    Connected,
    Disconnecting,
}

fn main() {
    let estado = EstadoConexion::Connected;
    println!("{:?}", estado);
}

