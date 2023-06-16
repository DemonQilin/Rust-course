// Las funciones son declaradas con la palabra clave "fn" seguida del nombre, parentesis, donde iran los parametros separados por coma, y finalmente llaves que contienen el cuerpo de la función.
// Para los nombres de funciones y variables normalmente se usa snake_case.
fn main() {
    // Para llamar una función se usa su nombre seguido de parentesis con los valores concretos para los parametros
    let z = my_function(5);
    println!("my_function returned x: {z}");
}

// Para los parametros de una función siempre se deben escribir sus tipos
// Las funciones opcionalmente pueden retornar un valor, para esto primero debe declarar el tipo del retorno usando la sintaxis de flecha "->". Las funciones por defecto retornan la unidad () => tupla vacia.
// Las funcionen retornan automaticamente la ultima expresión () en su cuerpo, si se quiere hacer un retorno mas temprano debe usarse la palabra clave "return".
// Para usar la expresión final como retorno se debe evitar el uso del punto y coma al final de la linea, porque esto convierte una expresión en una sentencia.

// En Rust la expresiones evaluan a un valor. Mientras las sentencias son instrucciones que no retornan ningun valor.
// Nota: Las llaves ({}) son expresiones
fn my_function(x: i8) -> i8 {
    println!("my_function called with x: {x}");
    let y = 10;
    y
}
