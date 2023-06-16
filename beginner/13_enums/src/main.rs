// Enums
// Permiten crear un valor de un conjunto de valores establecidos, enumeración de valores, es posible valores son conocidos como "variantes", y pueden ser unitarios, tuplas y estructuras. La principal ventaja es que el enum pueden agrupar todos estas posibles variantes bajo un mismo tipo.

// Para crear una enumeración se usa la palabra clave "enum"
// Cada variante puede tener diferentes tipos y cantidades de datos asociados
enum Command {
    Undo,               // Unit
    Redo,               // Unit
    AddText(String),    // Tupla
    MoveCursor(i8, i8), // Tupla
    ReplaceText { from: String, to: String },
}

// Para los enums tambien se pueden definir metodos mediante un bloque impl
impl Command {
    fn serialize(&self) -> String {
        // Para hacer uso de un enum se suele usar la estructura de control "match"
        "JSON dummy".to_string()
    }
}

fn main() {
    // Una instancia del enum UNICAMENTE PUEDE SER UNA VARIANTE a la vez
    // Para instanciar una variante se usa la notación de doble ":" y el nombre de la variante
    let cmd = Command::Undo;
    let cmd = Command::Redo;
    // El nombre de cada variante se convierte en una función que construye una instancia del enum
    let cmd = Command::AddText(String::from("Test"));
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::ReplaceText {
        from: String::from("Juan"),
        to: "Esteban".to_string(),
    };

    let json_string = cmd.serialize();
    println!("{json_string}");
}
