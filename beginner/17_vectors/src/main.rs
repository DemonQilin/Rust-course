// Vectores
// Juntos con los String y los HashMap son los tres tipos de colecciones mas ampliamente utilizadas en Rust, permite almacenar un sequencias de datos del mismo tipo que puede crecer y decrecer durante la ejecución.

fn main() {
    // Dos formas de instanciar un vector: metodo "Vec::new" y la macro "vec!"
    // Con el metodo se crea un array vacio: este necesita conocer el tipo de datos que va a almacenar, y esto se puede lograr con anotación de tipo o agregando elementos
    let v: Vec<String> = Vec::new();
    let mut v = Vec::new();
    // La propiedad de valores que tienen datos en el heap sera transferida al vector. Lo que significa que al ser limpiado el vector todo sus items tambien seran eliminados
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    // Usar la macro te permite iniciar con valores
    let v2 = vec![1, 2, 3];

    // 2. Lectura: la lectura de un valor puede lograrse mediante indexación o el método "get"

    // La indexación se logra usando los corchetes y el indice del item de interes. Sin embargo, si se pasa un indice mayor a la cantidad existente, o invalido, el programa entrara en panico deteniendo la ejecución.
    // Es importante notar el operador "&" debido a que si se trata de almacenar "v[0]" se estaria tratando de tomar propiedad y el path v[_] no tiene permiso de propiedad, ademas de que si lo hiciera el vector apuntaria en su primera posición posible valor invalido.
    let s = &v[0];
    // Una forma de obtener propiedad sobre un item del vector es usando el metodo "remove" que directamente lo elimina del vector y devuelve el valor.
    // let s = v.remove(0);

    // El metodo get devuelve un item opcional, lo que quiere decir que usa el enum Option, indicando que si el resultado es instancia de Some el indice era valido, y si es instancia de None era un indice invalido.
    let s = v.get(0);

    if let Some(e) = s {
        println!("{e}");
    }

    // 3. Iteración: para iterar sobre los items de un vector se usa el ciclo "for" para obtener una referencia a cada item del vector para evitar consumir su propiedad. Aqui tengo un poco de confusión pero es realmente una referencia del vector.
    // Obteniendo referencias mutables. Es azucar sintatica para el metodo "iter_mut"
    for sr in &mut v {
        sr.push('!');
    }

    // Referencia inmutable. Es azucar sintantica para el metodo "iter".
    for sr in &v {
        println!("{sr}");
    }

    // Loops que consumen la propiedad del vector
    let mut v3 = vec![];

    // Se consume la propiedad del vector lo que al terminar el for el valor es liberado e invalido para uso posterior
    // Para este caso de consumo, el for es sugar-sintax para el metodo "into_iter" que crear un iterador consumiendo la proiedad del vector
    for sr in v.into_iter() {
        v3.push(sr);
    }

    // El vector apunta a data invalida
    // let i = v.get(0);
}
