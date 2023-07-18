/*
    LIFETIMES
    El lifetime de una variable es el rango de tiempo en el que los datos a los que apuntan son válidos. El lifetime comienza desde donde se define una variable hasta donde sale del scope o donde mueve sus datos.

    Los lifetimes permiten al verificador de prestamos asegurar que no existen punteros colgantes siguiente la regla de cada todo valor deberia sobrevivir a sus referencias, es decir, el lifetime de un valor deberia ser mayor que el de sus referencias.
*/

fn main() {
    lifetime_of_reference_too_long();
    lifetime_inmutable_and_mutalbe_references();
}

fn lifetime_of_reference_too_long() {
    // El lifetime de la referencia comienza en la linea 14 y finaliza al finalizar la función. Es decir, su lifetime es mas largo que el del valor al que apunta por lo que de ser permitido podria apuntar a memoria invalida
    let r;

    {
        // El lifetime del string comienza en la linea 16 y termina el linea 18 con el final del bloque
        let s = String::from("I love Rust ♥");
        r = &s;
        println!("r: {r}");
    }

    // Para este momento el string fue limpiado y la referencia quedo aputando a memoria liberada
    // println!("r: {r}");
}

fn lifetime_inmutable_and_mutalbe_references() {
    // El lifetime de esta variable va desde la linea 28 hasta el final de la función
    let mut s = String::from("I love Rust 🦀");
    // Los lifetimes de referencias mutables e inmutables no pueden coincidir
    // El lifetime de la referencias r1 va desde la linea 32 hasta su ultimo uso en la linea 33
    let r1 = &s;
    println!("r1: {r1}");

    // El lifetime de r2 va desde la linea 36 hasta la linea 37
    let r2 = &mut s;
    r2.push_str("♥");
    // De mover la impresión el ultimo uso seria en la linea 39 y le lifetime de la referencia inmutable r1 se extenderia hasta alli por lo que ambas referencias existirian al mismo tiempo lo cual no puede ser por seguridad
    // println!("r1: {r1}");
}
