// Slices
// Son referencias inmutables a sequecuencias CONTINUAS de una colección en memoria (heap, stack o static).
// Al ser referencias estan enlazadas a la colección y hacen que la variable propietaria sea solo se lectura para que no ser invalidada.

// Los slices normalmente se crean utilizando los rangos de Rust que consiste de corchetes con dos puntos que separan dos indices: [inital..end_plus_one]
// - Indice inicial: un valor numerico desde donde se quiere empezar a cortar la colección, si es cero, puede ser omitido
// - Indice final: valor numerico del indica hasta donde se quiere cortar la colección, no lo incluye, si coincide con la longitud de la colección puede 

fn main() {
    let tweet = String::from("Estoy aprendiento Rust y me esta encando este nuevo lenguaje de programación");
    let trimmed_tweet = trim_tweet(&tweet);
    println!("{trimmed_tweet}");

    // Los string literales son slices que apuntan al string que se almacena en el binario (static).
    let tweet2 = "Rust es un lenguaje con muchas caracteristicas interesantes";
    let trimmed_tweet2 = trim_tweet(tweet2);
    println!("{trimmed_tweet2}");
}

// Cuando se necesita un string de solo lectura como parametro siempre es mejor usar un slice porque amplia la cantidad de parametros que puede recibir, dando mas extensibilidad. Por ejemplo puede recibir: slices de un string, la referencia del string, string literales y slice del string literal. Tambien aplica con los vectores.
fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}
