/*
    DERIVAR TRAITS
    El atributo "#[derive()]" permite implementar un Trait con una implementación basica. Es importante notar que no todos los traits pueden ser derivados, unicamente los que tienen asociada la macro derive.
*/

// Implementa de forma automatica y basica el trait Debug para conversión a texto en un formato de debug
// Implementa el trait PartialEq que permite la comparación de items
#[derive(Debug, PartialEq)]
struct Point {
    x: u8,
    y: u8,
}

fn main() {
    let p1 = Point { x: 7, y: 5 };
    let p2 = Point { x: 7, y: 5 };
    let p3 = Point { x: 1, y: 3 };

    // Impresión de p1 con el formato para debuggeo
    println!("{p1:?}");
    println!("{}", p1 == p2);
    println!("{}", p2 == p3);
}
