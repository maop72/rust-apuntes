mod universidad {
    pub mod alumnos {
        pub fn matricular() {
            println!("Alumno matriculado");
        }
    }
}

fn main() {
    universidad::alumnos::matricular();
}
