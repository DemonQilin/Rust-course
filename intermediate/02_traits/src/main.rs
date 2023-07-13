/*
    TRAITS

    Los traits son la forma en que se definen funcionalidades compartidas en tipos. Se trata de definir comportamiento de una forma abstracta, similar a las interfaces en otros lenguajes.

    Para crear un trait se usa la palabra reservada "trait" seguida del nombre que se le va a dar y dentro de llaves "{}" se declaran la firma de los metodos que agrupa donde:

        - Pueden terminar en punto y coma ";", obligando al tipo que implementa el trait a definir el comportamiento
        - Tener una implementación, que se conoce como Implementación Por Defecto, y cualquier tipo que implemente el Trait tendra esta implementación a menos que la sobreescriba con su propia definición.

    Los Traits solo permiten compartir/abstraer funcionalidad, es decir, métodos.

    IMPLEMENTACIÓN

    Implementar un Trait es muy similar a implementar metodos normales, la diferencia es que despues de "impl" se pone el nombre del Trait seguido de la palabra reservada "for" y finalmente el nombre del tipo al que se le va hacer la implementación. Luego dentro de llaves usando las firmas de los metodos del Trait definimos los comportamientos que se requieran y/o se quieran sobreescribir.

    Los metodos de un Trait pueden ser invocados de la misma forma que los normales, pero antes se debe traer el Trair al scope.
*/

// Definimos un trait con el proposito de que cualquier que lo implemente tenga la capacidad de parquear
trait Park {
    fn park(&self);
}

// Definimos un trait que busca dar la funcionalidad de pintar y ademas tiene una implementación por defecto, que mas adelante los tipos que lo implementen podran elegir sobreescribir
trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {color}");
    }
}

// Para abstraer campos e información usamos un struct porque los Trait solo pueden abstraer metodos
struct VehicleInfo {
    make: String,
    model: String,
    yeat: u16,
}

struct Car {
    info: VehicleInfo,
}

// Implementamos el Trait Park y definimos el comportamiento deseado al metodo "park"
impl Park for Car {
    fn park(&self) {
        print!("Parking car");
    }
}

// Implementa el trait Paint y utilizara el comportamiento por defecto
impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        print!("unloading truck");
    }
}

// Al igual que Car implementa pintar y define su propia forma de hacerlo
impl Paint for Truck {
    fn paint(&self, color: String) {
        println!("painting Truck with color: {color}");
    }
}

fn main() {
    println!("Hello, world!");
}
