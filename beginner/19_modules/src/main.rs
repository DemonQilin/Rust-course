// Crate binario
// Cuando se tiene tanto un crate binario como uno de biblioteca, lo normal es que el arbol de modulos se defina en el create de liberia y que el crate binario sea un cliente de la API publica que comparte el codigo.

// Para utilizar el codigo del crate de libreria se usa el nombre del paquete, que haria referencia al crate de libreria, y se traen los items necesarios.

use auth_service::Credentials;

fn main() {
    let creeds = Credentials::new("Demonqilin", "PinturaMiriada12345*");

    auth_service::authenticate(creeds);
}
