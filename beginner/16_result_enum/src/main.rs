// Result Enum
// Hace parte del tema de errores recuperables, que se basa en posibles errores que pueden ser manejados sin detener el programa. Este enum es utilizado para indicar que un proceso puede ser exitoso o fallar. El preludio trae al scope este enum y sus variantes, igual que Option, por lo que puede ser usado sin codigo extra.

// Option puede representar simplemente que una operación ha fallado, pero Result puede explicar por qué la operación ha fallado

/* La forma de Result sigue la forma:
   enum Result<T, E> {
       Ok(T),
       Err(E),
   }

   Donde, la variante Ok contendra el resultado de tipo T cuando la operación sea exitosa, y la variante Err contendra el valor de tipo T cuando la operación falle.

   Result tienes dos metodos: expect y unwrap, que permiten acceder al contenido de la variante Ok pero que si fallan llevaran a panico al programa.

   - unwrap: Si el valor Result es la variante Ok, unwrap devolverá el valor dentro del Ok. Si el Result es la variante Err, unwrap llamará a la macro panic! por nosotros.

   - expect: Funciona igual que unwrap pero permite escoger el mensage de error que mostrara la panic!.

   Este ultimo metodo suele ser mas utilizado porque da mas contexto acerca de porque se esperaba que operación fuera exitosa.
*/



fn main() {
    let username = get_username(1);

    if let Some(name) = &username {
        println!("{name}");
    }
}

fn get_username(id: u32) -> Option<String> {
    // Recordemos la macro format que permite concatenar strings en un nuevo String
    let query = format!("GET username FROM users WHERE id={id}");
    let db_result = query_db(query);

    // Se necesita pasar de una instancia de Result a Option, donde si no hubo error se devuelva Some y si hubo error devolver None
    // Este necesita se cubre con el metodo "ok" del enum Result
    db_result.ok()
}

// Se usa el enum Result para indicar que la función puede fallar, y en cualquier caso almacenara un string con información referente al caso. Exitoso -> el username, Error -> Una descripción
fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query is empty!"))
    } else {
        Ok(String::from("Ferris"))
    }
}
