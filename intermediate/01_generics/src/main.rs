/*
    GENERICS

    Cuando se quiere crear funciones, estructuras o enums que funcionen con varios tipos de datos en concreto se usan los genericos. Estos permiten expresar que se trabaja con cualquier tipo que sera reemplazado por uno en concreto mas adelante en su uso.

    Los genericos se declaran dentro de llaves angulares (<>), y suelen usar una letra mayuscula del alfabeto como nombre, normalmente: T, U. Tambien se suele usar T0, T1 o en menor medida nombres en UpperCamelCase.

    FUNCTIONS
    Similar a como se hace en Typescript, en la funciones lo genericos se declaran despues del nombre de la función pero antes de la lista de parametros.

    STRUCTS
    Se declaran despues del nombre de la estructura y antes de la llave que comienza su definición, una vez declarados pueden ser usados en la definición de la estructura.

    ENUMS
    Los ejemplos mas claro de genericos con enums son Result y Option. Se declaran igual que en los structs.

    METODOS
    Segun veo yo, se refieren sobre todo cuando el struct o enum sobre el que se implementa el metodo usa genericos en su definición, lo que permite definir metodos en todas las versiones y/o en solo algunas en base al generico que se le pase al item. Se definen despues de la palabra clave "impl" y se usan en el item y las firmas de los metodos.

    Tambien se puede optar por implementar con genericos concretos, y estos metodos solo estran disponibles para la version del generico que tenga ese tipo:

        impl Point<i32> {
            fn only_available_in_point_with_i32() {}
        }

    Es importante resaltar que los metodos al ser funciones tambien pueden definir sus propios genericos.


    LOS GENERICOS NO AFECTAN EL RENDIMIENTO
    Rust utiliza una tecnica llamada "Monomorfización" que consiste en buscar las implementaciones de los genericos y generar los items con estos tipos especificos y reemplazarlos donde se usan, todo en tiempo de compilación. Es decir no hay sobrecargas ni reserva de memoria, se compila lo que se usa y como si hubiera escrito cada variante a mano.
*/

// Puede utilizar cualquier tipo de valor como payload
struct BrowserCommand<T> {
    name: String,
    payload: T,
}

// Esto define un metodo generico, disponible siempre
impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }

    // Retorna una referencia al payload que es del mismo tipo que T
    fn get_payload(&self) -> &T {
        &self.payload
    }
}

// Define un metodo solo disponible cuando el payload es un String
impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }

    // No se puede definir el mismo metodo dos veces
    // fn get_payload(&self) -> &String {}
}

fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://rust-book.cs.brown.edu/".to_owned(),
    );
    let cmd2 = BrowserCommand::new("zoom".to_owned(), 200);

    cmd1.print_payload();

    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    // Puede ser llamado con distitnos tipos
    serialize_payload(p1);
    serialize_payload(p2);
}

// Recibe una referencia de cualquier tipo
fn serialize_payload<T>(payload: &T) -> String {
    "placeholder".to_owned()
}
