fn main() {
    // 1. IF STATEMENT
    let number = 5;

    // Las condiciones se escriben usando la palabra clave "if" seguida de la condición, que siempre debe ser un bool (en Rust no existen los truthy o falsy), y el codigo a ejecutar entre llaves.
    if number > 5 {
        println!("The number is greather than 5");
    } else if number > 3 {
        // Se pueden utilizar varias condiciones usando la sentencias "else if"
        println!("The number is greather than 3");
    } else {
        // Tanto las sentencias "else if" y "else" son opcionales, pero deben estar precedidas de un "if"
        println!("The number is smaller than 3");
    }

    // Recordando que las llaves son expresiones es posible usar las ramas del "if" para las sentencias de asignación
    // Ambas ramas deben evaluar a valores del mismo tipo
    let _x = if number < 3 { 1 } else { -1 };

    // 2. loop
    let mut counter = 0;

    // La palabra clave "loop" repetira indefinidamente el codigo dentro de las llaves hasta que sea detenido
    loop {
        counter += 1;

        // Para detener un loop se usa la palabra clave "break", hay que tener en cuenta que cuando break esta solo detiene el ciclo mas interno.
        // "break" solo puede ser usado con este tipo de ciclo
        if counter == 10 {
            break;
        }
    }

    // Los ciclos "loop" pueden retornar valores a traves de las palabras clave "break", de haber varios breaks deben retornar el mismo tipo
    // Los loops pueden ser etiquetados usando una comilla simple, un nombre y dos puntos antes de la sentencia loop. Esto permite especificar detener un loop distinto al mas interno con un "break"
    let result = loop {
        break 5;
    };

    println!("The result returned by loop is: {result}");

    // 3. while
    // El ciclo while permite ejecutar codigo repetidamente siempre que la condición sea verdadera
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    // 4. for
    // El ciclo "for" permite ejecutar un bloque de codigo por cada elemento en una colección, evitando asi posibles errores y sobrecargas de tratar de hacerlo con los otros ciclos
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Tambien es posible usar este ciclo para recorrer rangos, que se crean
    for z in 1..4 {
        println!("{z}!");
    }
    println!("LIFTOFF!!!");
}
