/*
    GENERIC LIFETIMES
    Son anotaciones que expresan como se relacionan los tiempos de vida de las distintas referencias de una función. Esto no cambia la duración del tiempo de vida, solo expresa restricciones en como se relacionan la duración de estas referencias. Evitan las referencias colganges, es decir, usar valores que apuntan a memoria invalida.

    Estas anotaciones al igual que los tipos genericos se declaran dentro de los brackets angulares y se diferencian de los genericos porque inician con apostrofe "'", seguido del nombre que se le quiere dar al lifetime, normalmente se usan letra minusculas: 'a, 'b, etc.

    Una vez definido un lifetime, se relaciona con una referencia ubicandolo entre el operador de referencia "&" y el tipo de la referencia.

    Esto permite que el analizador de prestamo rechace los valores que no cumplan con las relaciones que definimos entre las referencias. Recordemos, que el compilador solo conoce la firmas y no los valores concretos con los que invocanlas funciones, por lo que no pueden saber que referencias elegir sin anotaciones de lifetimes.

    LIFETIMES EN FUNCIONES
    Cuando se retorna una referencia desde una función, el tiempo de vida del retorno debe coincidir con alguno de los tiempos de vida de los parametros. Se trata de conectar los tiempos de vida de varios parametros con los valores de retorno.

    No es posible retornar referencias a valores creados internamente, porque al final de la función se limpiaria el valor y la referencia seria un puntero colgante.

    STATIC LIFETIME
    Es un lifetime especial que tienen todos los strings literales y significa que la referencia vive durante toda la duración del programa debido a que se almacena en el binario.

    Se especifica así:
        &'static str
*/

fn main() {
    let player1 = String::from("player 1");
    {
        let player2 = String::from("player 2");
        // Es valido utilizar ambas referencias dentro del scope interno, por lo que el resultado sera valido desde su creación hasta finalizar este scope interno
        let result = first_turn(&player1, &player2);

        println!("Player going first is: {result}");
    }

    // No es correcto, porque para este punto player2 ha sido liberado y ninguna referencia a ella puede ser usada.
    // println!("Player going first is: {result}");
}

// El lifetime expresa que debe haber un rango de tiempo donde ambas referencias p1 y p2 sean validas y durante ese tiempo el retorno será igualmente valido
fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}
