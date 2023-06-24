/*
  MODULOS

  Con los modulos es posible agrupar definiciones relacionadas y darles un nombre, haciendo asi el codigo mas organizado y encapsulado.

  1. Permiten organizar codigo para legibilidad y reutilización
  2. Controlan el scope y la privacidad
  3. Contienen items: funciones, structs, enums, traits, etc.
  4. Definición explicita: no mapea el sistema de archivos.

   En un archivo se declaran nuevos modulos usando la palabra clave "mod" seguida del nombre del modulo que finalmente puede estar seguido de:
   - punto y coma: asi el compilador buscara un archivo con el nombre de ese modulo en la carpeta respectiva del modulo que declara (que si es el archivo raiz del crate sera "src").
   - llaves: "{}", dentro de estas podran declararse distintos items y otros modulos.

   Las raices de un crate, src/main.rs ó src/lib.rs, forman un modulo llamado "crate" en la raiz del arbol de modulos.

   NOTA: Los modulos no tienen efecto en la ejecución, son puramente para organización en el momento de compilación.

   El siguiente codigo en un archivo src/lib.rs:

   mod front_of_house {
       mod hosting {
           fn add_to_waitlist() {}

           fn seat_at_table() {}
       }

       mod serving {
           fn take_order() {}

           fn serve_order() {}

           fn take_payment() {}
       }
   }

   Forma el siguiente arbol de modulos:

   crate
   └── front_of_house
       ├── hosting
       │   ├── add_to_waitlist
       │   └── seat_at_table
       └── serving
           ├── take_order
           ├── serve_order
           └── take_payment


   PATHS

   Es la dirección donde se encuentra un item en el arbol de modulos. Existen dos tipos de path:

   - Path Absoluto: Es el path completo y comienza desde la raiz del crate. Para un crate externo, comienza con el nombre del crate, y para el crate actual comienza con "crate".

   - Path Relativo: Comienza desde el modulo actual y usa "self", "super", o un identificador en el modulo actual.

   Ambos paths esta seguidos de uno o varios identificadores separados por doble dos puntos (::).


   PRIVACIDAD

   Sin importar la implementación del bloque del modulo (en linea o en archivos separados), todo los items (functions, methods, structs, enums, modules, and constants) dentro de un modulo son PRIVADOS PARA MODULOS PADRES POR DEFECTO. Esto quiere decir, que un modulo padre no puede usar los items privados de un modulo hijo pero los modulos hijos pueden usar los items de sus modulos ancestros.

   Para exponer un item a los modulos ancestors, hacerlo PUBLICO, se debe usar la palabra clave "pub" antes de su declaración.

   Hay que tener en cuentas las siguientes consideraciones al usar "pub" con estructuras:

    - Use "pub" antes de su definición hace publica la estructura pero sus campos siguen siendo privados (no pueden ser liedo, escrito ni establecidos en una instanciación).
    - Se puede hacer publico cada campo de una estructura segun se necesite. Es normal implementar metodo de instaciación para la estructura cuando tiene campos privados.

   Por el contrario, cuando un enum se hace publico todas sus variantes tambien seran publicas.


   SHORTCUTS WITH use KEYWORD

   Permite crear atajos a un path y entonces usar ese atajo en cualquier parte del scope. Para este caso el scope esta relacionado con las declaraciones de un modulo, es decir, un atajo creado con "use" unicamente aplica para ese modulo, es decir como si hubiera sido declarado ahi.

   Se usa la palabra clave "use" seguida de un path, y se traera al scope el ultimo item del path.

   Lo siguiente traer el tipo "HashMap" al scope del modulo actual, y permite usarlo sin referenciar todo el path.

    use std::collections:HashMap;

   La convención dicta que es más idiomatico traer el path del modulo padres de una función y luego acceder desde el a la función, para hacer mas claro que la función esta definida en otro modulo.

   Por el contrario, para estructuras, enums y otros items es mas idiomatico traer el path completo. La unica excepción es cuando se traen dos items con el mismo nombre porque esto dara Error, entonces es mejor traer los modulos padres.

   Para el problema anterior tambien es posible especificar un nombre local distinto para el tipo que se trae, un alias, con la palabra clave "as" despues del path y seguida del nuevo nombre.

   Lo siguiente, habilita un tipo "PerroMapa" que es un alias en el modulo para el tipo "HashMap".

    use std::collections:HashMap as PerroMapa;

   RE-EXPORTACIÓN

   El nombre creado por "use" puede ser visto como otro item, y al igual que pasa con los demas, es privado para ese scope. Para permitir que codigo externo puede utilizar el atajo creado en un modulo, se usa "pub" antes de su declaración. Esto se conoce como re-exporting y es util cuando la estructura internal del codigo es diferente a como los programadores llaman el codigo.

   PATHS ANIDADOS

   Permite traer varios items desde un modulo en una sola linea. Para ellos inicialmente se escribe el path comun, luego "::", y dentro de llaves "{}" los paths que difieren. La palabra "self" dentro de las llaves indica que se trae el path comun.

   La siguiente importación:

    use std::cmp::Ordering;
    use std::io;

   Es igual a:

    use std::{cmp::Ordering, io};

   GLOB OPERATOR

   Permite traer todos los items publicos definidos en un path (supongo que modulo). Hay que ser cuidadoso porque es dificil saber que es todo lo que trae y podria generar conflictos con el codigo.

   Se suele usar para test y el patron preludio.


   PLUGIN cargo-modules
   Es un plugin de cargo que permite visualizar el arbol de modulos de un crate.

   Se instala a traves del comando:

    cargo install cargo-modules

   El comando basico para generar el arbol es:

    cargo modules generate tree

   Considerar las siguientes banderas:

    --types                          Include types (e.g. structs, unions, enums)
    --no-types                       Exclude types (e.g. structs, unions, enums). [default]
    --traits                         Include traits (e.g. trait, unsafe trait)
    --no-traits                      Exclude traits (e.g. trait, unsafe trait). [default]
    --fns                            Include functions (e.g. fns, async fns, const fns)
    --no-fns                         Include functions (e.g. fns, async fns, const fns). [default]
    --tests                          Include tests (e.g. `#[test] fn …`)
    --no-tests                       Exclude tests (e.g. `#[test] fn …`). [default]

*/
#![allow(dead_code, unused_variables)]

// Para usar paquetes externos, se debe primero agregar al Cargo.toml o ejecutar el comando "cargo add <crate_name>". Una vez el paquete ya es traido, se usa su nombre para usar sus items.

mod auth_utils;
mod database;

pub use self::auth_utils::models::Credentials;
use database::Status;
// Uso del glob operator para traer todos los items publicos que expone el modulo prelude del crate rand
use rand::prelude::*;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        // Uso el item "threan_rng" que trajo al scope el preludio de rand
        let timeout = thread_rng().gen_range(100..500);
        println!("The timeout is {timeout}");

        auth_utils::login(creds);
    }
}
