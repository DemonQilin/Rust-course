// Tuples structs
// Funcionan como una tupla pero tienen el significado agregado que da el nombre de la struct. Ademas de que funcionan como tipos diferentes.

// Usan paretensis y sus campos no tienen nombre solo el tipo
struct Rgb(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// unit-like structs
// Existe una version de la unidad (tupla vacia) como estructura y se suele usar, segun el libro y Bodgan, en la implementación de metodos y traits que no requieren almacenar ninguna información
// Esta solo recibe nombre
struct AlwaysEqual;

fn main() {
    let black = Rgb(0,0,0);
    let origin = Point(0,0,0);

    println!("Red light in black is: {}", black.0);
    println!("Origin is: {:?}", origin);

    // La instanciación de un unit-like struct usa solo el nombre
    let _subject = AlwaysEqual;
}
