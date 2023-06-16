// Strings
// Los strings en Rust estan codificados en UTF-8 lo cual implica que cada caracter puede estar conformado entre 1 y 4 bytes. Donde es importante recordar que el alfabeto americano (ASCII) todos utilizan 1 byte.

// Debido a lo anterior en Rust no es posible indexar strings mediante enteros porque seria posible acceder a caracteres incompletos que no pueden ser procesados. Sin embargo, se pueden crear slices entre bytes especificos pero teniendo en consideración que si no coincide con un limite de caracter valido el programa entrara en panico.

// El patron de usar slices strings como parametro de una función es util cuando no se necesita una referencia mutable o propiedad de un string, y usa una caracteristica llamada Draft Coercion para convertir las referencias a string en strings slices.

// En Go existe un tecnica que hace todos los caracteres de un string utf-8 de 4 bytes para asi poder tener una forma constante de busqueda.

fn main() {
    create_strings();
    manipulate_strings();
    concatening_strings();
    indexing_string();
}

fn create_strings() {
    let s1 = String::from("Hola");
    // Los string slice tienen metodos como "to_string" y "to_owned" para convertir a String. Su unica diferencia es que owned hace un copiado mas profundo, por lo que ocupa mas memoria.
    let s2 = "mundo".to_string();
    let s3 = "mundo".to_owned();

    println!("{s1}, {s2}, {s3}");
}

fn manipulate_strings() {
    // Para modificar un string lo primero que se debe hacer es crear la variable de forma mutable
    let mut s = String::from("foo");
    // Para agregar otro string al final
    s.push_str(" bar");
    println!("{s}");
    
    // Para reemplazar una porción del string
    s.replace_range(.., "baz");
    println!("{s}");
}

fn concatening_strings() {
    let s1 = String::from("Hello");
    let s2 = String::from(", world");
    // Para concatenar strings existen varias estrategias

    // 1. Operador "+". El String debe ir primero y tomara propiedad sobre este, los siguientes deben ser referencias/string slices.
    let s3 = s1 + &s2;
    // println!("{s1}"); // s1 ya no puede ser usado porque transfirio su propiedad a s3
    println!("{s3}");

    // 2. Usando la macro format!, la cual utiliza la interpolación usando los simbolos "{}" (como print!). Durante este proceso se clonan todos los valores, por lo que no es la solución de mejor rendimiento
    let s1 = "Tic".to_string();
    let s2 = "Tac".to_owned();
    let s3 = String::from("Toe");
    let s3 = format!("{s1}-{s2}-{s3}");
    println!("{s3}");

    // 3. Colocar los string en una matriz y llamar el metodo "concat"
    let s1 = ["first", "second"].concat();
    println!("{s1}");

    // 4. Usando la macro concat!, la cual retorna un valor de tipo string slice y funciona solo para strings literales
    let s3 = concat!("first", "second");
    println!("{s3}");
}

fn indexing_string() {
    use unicode_segmentation::UnicodeSegmentation;
    // Nuevamente, debido a que son caracteres utf-8 y que cada uno puede tener desde 1 a 4 bytes, la indexación en Rust no es posible por enteros 
    let s1 = "🦀🦀🦀🦀🦀";
    // Podemos acceder al primer cangrejo usando slices, especificamente porque sabemos que tiene 4 bytes
    let first = &s1[..4];
    println!("{first}");

    // Para hacer al busqueda de caracteres se debe recorrer todos los caracteres del string.

    // 1. Recorriendo bytes
    println!("=== Bytes ===");
    for b in "नमस्ते".bytes() {
        println!("{b}");
    }
    println!("=== Bytes ===");
    
    // 2. Recorriendo caracteres
    // Para este ejemplo es importante recordar que los caracteres son valores escalares unicode, que son la unidad minima, y no los caracteres percibidos. Estos ultimos se conocen como grapheme cluster, que suelen estar compuesta por mas de un valor escalar unicode.
    
    println!("=== Chars ===");
    for c in "नमस्ते".chars() {
        println!("{c}");
    }
    println!("=== Chars ===");
    
    // 3. Recorriendo los grafemas, solo posible usando un crate externo
    println!("=== Graphemes clusters ===");
    for g in "नमस्ते".graphemes(true) {
        println!("{g}");
    }
    println!("=== Graphemes clusters ===");
}