// Option Enum
// Para el caso comun donde se puede tener o no un valor, Rust decide no tener un valor "null" y en su lugar usa el enum Option para expresar este concepto de posible ausencia de valor. Para tener un valor que posiblemente pueda ser nulo, debe optar explícitamente por ello haciendo que el tipo de ese valor sea Option<T>.

// El enum Option tiene dos variantes: Some y None. Donde Some representa que hay un valor presente y None represente que no se tiene un valor valido.

// enum Option<T> {
//     Some(T),
//     None
// }

// Tanto el enum como sus variantes son tan utiles que incluyen en el preludio, lo que quiere decir que estan disponibles sin ningun codigo de importación.

// El poder de este enum esta en que un Option<T> es distinto al tipo T por lo que no puede ser usado como valor de este. Asi el compilador obliga a manejar todos los posibles casos. Eliminar el riesgo de asumir incorrectamente un valor no nulo le ayuda a tener más confianza en su código.

// El enum Option<T> tiene un gran número de métodos que son útiles en una gran variedad de situaciones; puedes consultarlos en la documentación.

fn main() {
    let username = get_username(1);

    // Claramente se puede usar la estructura de control match, pero existe un sintaxis mas rapida para validar un unico patrón
    // Es una forma menos verbosa de tratar los valores que coinciden con un patrón ignorando el resto. Toma un patron y una expresión separados por un signo "="
    if let Some(name) = &username {
        // Utilizar if let significa menos tipado, menos indentación y menos código repetitivo. Sin embargo, se pierde la comprobación exhaustiva que impone match. Elegir entre match e if let depende de lo que estés haciendo en tu situación particular y de si ganar concisión es una compensación adecuada por perder la comprobación exhaustiva.
        println!("{name}");
    }

    // if let puede tener un bloque "else" que sirve para ejecutar codigo si el patron no coincide, pero no permite comprobar mas patrones, el if let es una sugar sintaxis para:
    // match username {
    //     Some(name) => todo!(),
    //     _ => todo!()
    // }
}

// El retonor indica que el valor podria estar vacio
fn get_username(id: u32) -> Option<String> {
    let db_result = String::from("Ferris");

    if id == 1 {
        // Notar como no es necesario usar el prefijo Option::Some
        Some(db_result)
    } else {
        None
    }
}
