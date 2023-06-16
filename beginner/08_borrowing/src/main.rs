// Borrowing
// Consiste en crear referencia para acceder a un valor sin ser su propietario. Es util por terminos de eficiente y cuando no se desea mover la propiedad de un valor. Las referencias no son propietarias, es decir, terminado el frame al que pertenecen no se limpian los valores que contienen.

/*
    El borrowing sigue dos reglas:
    1. En un momento dado, solo se puede darse una referencia mutable o variables referencias inmutables, pero no ambas.
    2. Los datos debe sobrevivir a todas las referencias que los apuntan
*/

fn main() {
    let mut s1 = String::from("Rust");
    // Las referencias se crean mediante el operador &
    let r1 = &s1;
    print_string(r1);
    // Las referencias mutables se crean mediante el operador &mut, permiten cambiar el valor al que apunta la referencia
    let r2 = &mut s1;
    add_to_string(r2);
    // print_string(r1); // El uso de r1 en esta linea hace que existan al mismo momento una referencia inmutable y una mutable causando que el programa no compile
    ohter();
}

// Una referencia mutable permite cambiar el valor al que se apunta
fn add_to_string(s: &mut String) {
    // Para acceder al valor de una referencia se debe usar el operador de desreferencia *
    // Este operador no suele ser muy visible porque se usa implicitamente en metodos de punto
    // Rust implicitamente inserta referencias y desreferencias
    (*s).push_str(" is awesome");
}

// El tipo de una referencia se crea con el operador & y el tipo del valor al que apunta
fn print_string(p: &String) {
    println!("{p}");
}

fn ohter() {
    let mut s = String::from("Hello");
    let s_ref = &s;
    println!("s is: {s}");
    println!("{s_ref}");
}
