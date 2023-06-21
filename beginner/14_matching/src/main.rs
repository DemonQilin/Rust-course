// Match
// La construcción de flujo "match" permite comparar un valor con una serie de valores y ejecutar codigo basado en el patron coincidente. La utilidad de match viene de la expresividad y de que obliga a manejar todos los posibles casos.

enum Command {
    Undo,               // Unit
    Redo,               // Unit
    AddText(String),    // Tupla
    MoveCursor(i8, i8), // Tupla
    ReplaceText { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        // Se comienza con la palabra clave "match" seguida de una expresión (que evalua a un valor). El código asociado a cada brazo es una expresión, y el valor resultante de la expresión en el brazo coincidente es el valor que se devuelve para toda la expresión match.
        // Debido a que el match termina en la primera coincidencia, se debe organizar de casos especificos a mas generales
        match self {
            // Cada caso de comparación es llamado "arm" (brazo) y consta de un PATRON y el CODIGO a ejecutar separado por "=>" y una coma final.
            Command::Undo => String::from("{ \"cmd\": \"undo\" }"),
            Command::Redo => String::from("{ \"cmd\": \"redo\" }"),
            // Si el codigo a ejecutar tiene más de una linea se usa llaves y ya no se necesitara la coma final
            // Es posible acceder a partes de los valores coincidentes con el patron
            Command::AddText(s) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            }
            // Las variable como patrones cubren cada posible valor, son conocidos como catch-all
            // "_" es un patrón especial que coincide con cualquier valor y no se vincula a ese valor
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                )
            }
            Command::ReplaceText { from, to } => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
                )
            }
            // Si no quisiera hacer nada se usa la unidad
            // _ => ()
        }
    }
}

fn main() {
    let cmd1 = Command::Undo;
    let cmd2 = Command::Redo;
    let cmd3 = Command::AddText(String::from("Test"));
    let cmd4 = Command::MoveCursor(22, 0);
    let cmd5 = Command::ReplaceText {
        from: String::from("Juan"),
        to: "Esteban".to_string(),
    };

    println!("{}", cmd1.serialize());
    println!("{}", cmd2.serialize());
    println!("{}", cmd3.serialize());
    println!("{}", cmd4.serialize());
    println!("{}", cmd5.serialize());
}