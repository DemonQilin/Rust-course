// Estructura de un proyecto Rust
// Los proyectos de Rust tienen tres divisiones principales: package, crates y modules.

/*
   PACKAGE
   Es un conjunto de uno o varios crates que provee un conjunto de funcionalidades. Un package, normalmente, contiene un Cargo.toml que describe como construir estos creates, y permite constuir, testear y compartir crates.

   Sigue la siguientes reglas:
   - Debe tener minimo un 1 crate
   - Maximo tiene 1 library crate
   - Puede contener cualquier numero de binary crates

   Un package normalmente se crea usando el comando "cargo new", lo cual entrega una estructura base con un src/main.rs o src/lib.rs, puede tener ambos.

   Los crates tienen una raiz (crate root) que es el archivo fuente desde que el compilador de Rust arranca y constituye el modulo raiz del crate.


   CRATE
   Es la cantidad mas pequeña de codigo que el compilador de Rust considera a la vez. Pueden estar compuestos de un arbol de modulos, que se compilaran con el crate.

   Un crate puede venir en una de dos formas: crate binario o create de libreria.

   CRATES BINARIOS
   Son programas que pueden compilar a un ejecutable que se puede correr. Cada create binario debe tener una function main que defina que pasa cuando el ejecutable se ejecuta.

   Un package puede tener un crate binario con el mismo nombre del package, donde su raiz sera el archivo: src/main.rs.
   Los demas creates binarios se ubican en "src/bin" donde cada archivo será un crate binario y la raiz del mismo.

   CREATES DE LIBRERIA
   No compilan a un ejecutbale, y por lo tanto no tienen un función main. Definen funcionalidades con la finalidad de ser compartidas con multiples proyectos.

   La raiz de un crate de libreria es el archivo "src/lib.rs".
*/

// Cuando se tienen varios crates binarios el comando "cargo run" no sabe cual ejecutar, por lo que se debe agregar la bandera "--bin" y el nombre del crate que se quiere ejecutar.

fn main() {
    // Este es el archivo y modulo raiz de un crate binario con el mismo nombre del paquete, que es "structure_project".
    println!("Hello, world!");
}
