// Structs
// Las struct permiten agrupadar que tienen relación. Desde el punto de vista de Rust, permiten dar semantica al codigo mediante la agrupación de valores.

// Para crear una estructura se usa la palabra clave "struct" y el nombre, en Upper Camel Case. Luego dentro de llaves se declaran pares clave: tipo, donde la clave es el nombre que se le quiere dar al dato, se denominan "field".

// Rust permite hacer una implementación automatica para Debug pero de quererse algo personalizado se debe hacer la implementación concreta
#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool
}

// Las funciones que estan relacionada con una estructuras se suelen definitir como metodos de esa estructura. Para esto, se usan los bloques "impl"

// Todas las funciones de un bloque impl son conocidas como "Associated functions" pero solo las funciones que usan self se conocen como metodos. Todas las funciones asociadas pueden ser usadas con la sintaxis "::".
impl Product {
    // La función new (no es propia del lenguaje podria llamarse juanito) se usa para realizar la instaciación de una estructura
    fn new(name: &str, price: f32) -> Product {
        Product {
            name: name.to_string(),
            price,
            in_stock: true
        }
    }

    fn get_sales_tax() -> f32 {
        0.1
    }

    // En los metodos se usa como primer parametro la instancia de la estructura, para esto se usa la sintaxis: &self, que es un shorthand para "self: &Self"
    fn calculate_sales_tax(&self) -> f32 {
        self.price * Self::get_sales_tax()
    }

    // Se pueden recibir tres tipos de referencias a la instancia: prestamo inmutable, prestamo mutable y la propiedad. Los parametros del metodo son todos los que siguen al self.
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    // Cuando un metodo recibe la instancia con propiedad es importante notar que finalizado el metodo la instancia sera dropeada
    fn buy(self) -> u32 {
        let name = self.name;
        println!("{name} was bought!");
        123
    }
}

fn main() {
    let name = String::from("The Rust Language");
    // Para crear una instancia de una estructura se usa su nombre y dentro de las llaves se dan los valores a cada uno de los campos
    let book = Product {
        // Cuando se tiene un variable que se quiere asignar a un field y tiene el mismo nombre, se puede omitir el "field: field" y colocarlo una vez, se llamada Field Init Shorthand
        name,
        price: 29.95,
        in_stock: true
    };

    let mut book2 = Product {
        name: String::from("Rust: Zero to Production"),
        // Si se desea usar los datos de una instancia de la estructura ya existente, se usa la Struct Update Syntax, siempre debe ir de ultima. Si algun field de la estructura base usada tenia propiedad sobre algun dato en el heap, se transferira la propiedad quedando inutilizable la instancia base
        ..book
    };

    // Tanto para acceder como para mutar un field se usa la sintaxis de punto
    // book2.price = 40.15; // Para que se pueda mutar se debe haber declarado como mutable.
    book2.price = 40.15;
    println!("The second book price: $ {}", book2.price);

    // Las estructuras no implementan una representación en consola por defecto, por lo que tratar de imprimirlas genera error
    // println!("{book2}"); // Es importante notar que la notación "{}" es para la salida final

    // Para fines de debuggeo se usa "{:?}" o "{:#?}", sin embargo, requiere que la estructura implemente Debug.
    println!("{:#?}", book2);

    // Para usar un metodo se usa la sintaxis punto
    let tax = book2.calculate_sales_tax();
    println!("The book tax is: {tax}");

    book2.set_price(10.15);
    // Despues de ejecutado este metodo la estructura sera invalida
    book2.buy();

    let new_book = Product::new("Rust for rustaceans", 35.25);
    println!("{:#?}", new_book);
}
