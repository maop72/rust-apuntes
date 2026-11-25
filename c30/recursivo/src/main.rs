use serde_json::Value;

fn procesar(valor: &Value, nivel: usize) {
    let indentacion = "  ".repeat(nivel);

    if valor.is_null() {
        println!("{}Valor nulo", indentacion);

    } else if let Some(b) = valor.as_bool() {
        println!("{}Booleano: {}", indentacion, b);

    } else if let Some(n) = valor.as_number() {
        println!("{}Número: {}", indentacion, n);

    } else if let Some(cadena) = valor.as_str() {
        println!("{}Cadena: {}", indentacion, cadena);

    } else if let Some(array) = valor.as_array() {
        println!("{}Array ({} elementos):", indentacion, array.len());

        for elemento in array {
            procesar(elemento, nivel + 1);
        }

    } else if let Some(objeto) = valor.as_object() {
        println!("{}Objeto ({} propiedades):", indentacion, objeto.len());

        for (clave, valor) in objeto {
            println!("{}{}:", indentacion, clave);
            procesar(valor, nivel + 1);
        }
    }
}

fn main() {
    let json = r#"
    {
        "nombre": "eth0",
        "mtu": 1500,
        "activa": true,
        "direcciones": [
            "192.168.1.10",
            "fe80::1"
        ],
        "configuracion": {
            "automatica": false,
            "prioridad": 2
        },
        "observaciones": null
    }
    "#;

    let datos: Value = serde_json::from_str(json).unwrap();

    procesar(&datos, 0);
}
