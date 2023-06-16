// Ownership (propiedad) es una estrategia para el manejo de punteros, y se basa en las siguientes reglas:
/*
 * 1. Toda los datos del heap tienen exactamente un unico propietario.
 * 2. Rust limpia un valor del heap cuando su propietario sale de del scope.
 * 3. La propiedad sobre un valor del heap puede ser transferida a otra variable a traves de movimientos, los cuales se dan en asignación a variables y llamados de función.
 * 4. Los valores del heap pueden ser accedidos unicamente a través de su propietario actual, sus propietarios anteriores no pueden ser usados.
 */

fn main() {
    let s1 = String::from("Rust");
    let s3: String;
    {
        let s2 = String::from("Javascript");
        let s4 = s2.clone(); // Es posible obtener el clone de un valor sin transferir su propiedad mediante el metodo "clone"
        println!("s2 exists yet: {s4}");

        s3 = s2;

        // 1. y 4. En la linea 17 la propiedad del string "Javascript" fue transferida a s3, y los valores no pueden ser accedidos de variables que no son su propietario actual
        // println!("s2 in inner scope is: {s2}");
    }

    println!("s1 is: {s1}");

    // 2. Cuando el scope dentro de las llaves termino s2 fue limpiado junto con el string del que era propietario.
    // println!("s2 is: {s2}");

    // 3. La propiedad del string "Javascript" fue transferida de s2 a s3, por lo tanto, el valor no fue eliminado cuando el scope de las llaves termino.
    println!("s3 is: {s3}");

    // Por defecto, en Rust todas las asignación mueven la propiedad propiedad de un valor a la nueva variable.
    // Excepto los tipos primitivos: bool, enteros, flotantes y caracteres. Estos son clonados por defecto.
    let x = 10;
    let y = x;

    println!("x is: {x}");

    let a1 = String::from("Rust");
    let a2 = generate_string(); // a2 es ahora propietario del String que retorna la función
    let a3 = add_to_string(a1.clone());
    print_string(a1.clone()); // El pasar una variable como argumento mueve la propiedad de su valor al parametro de la función

    println!("a1 is: {a1}");
    println!("a2 is: {a2}");
    println!("a3 is: {a3}");
}

// Se crea una variable "p1" a la que se puede cambiar el valor que almacena, un String por otro String. El mut opera sobre variables no valores
fn add_to_string(mut p1: String) -> String {
    p1.push_str("Ferris");
    p1
}

fn generate_string() -> String {
    // Recordar que la ultima expresión de una función se retorna por defecto. Si es un dato de heap tambien se transfiere su propiedad a la variable que almacene el retorno.
    String::from("Ferris")
}

// Un parametro de una función es "igua" (digo yo) a crear una variable
fn print_string(p1: String) {
    println!("{p1}");
} // Al terminar esta función su frame se desasigna llevandose la variable p1 que ahora es propietaria del String que recibio
