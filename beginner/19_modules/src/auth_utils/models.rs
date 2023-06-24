// Recordemos que hacer una estructura publica no hace publicos sus campos. Por lo tanto, no podria instanciarse, en mi logica de negocio quiero que no se pueda interactuar con los campos despues de creados.
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    // Aqui la función asociada "new" permite instanciar Credentials
    pub fn new(username: &str, password: &str) -> Self {
        Credentials {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}

// Modulo hijo usando función declarada en el modulo padre
// fn hello_world() {
//     super::logout();
// }
