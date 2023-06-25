/*
    FEATURES
    Mediante esta caracteristica de Cargo se pueden optimizar los tiempos de compilación a traves de las siguientes dos funcionalidades:

        1. Compilación Condicional
        2. Dependencias Opcionales

    Para hacer uso de ellas, se declara la section [features] en el archivo cargo.toml del paquete. Y dentro de esta se declaran las distintias features que el paquete tendra, las cuales pueden ser activadas o desactivadas.

    Por defecto, todas las features de un paquete estan desactivadas. Pero mediante la feature default pueden activarse otras features, aunque solo es util cuando se tienen features que son necesarias para el uso comun.

    Las dependencias condicionales, se trata de dependencias que no se compilan por defecto. Y que se activan mediante las features del paquete. Para estas se usa la notación "dep:[nombre_depedencia]"

    Tambien es posible activar features de una dependencia usando la notación "nombre_dependencia/nombre_feature", importante que esto activa la dependencia si era opcional, para hacer que la feature se active solo si ya la dependencia fue activada por otra feature, se usa: "nombre_dependencia?/nombre_feature"
*/

pub fn draw_line(x: i32, y: i32) {
    // draw line without color
}

// Se conocen como expresión cfg y permiten compilar codigo si segun ciertas condiciones
// En este caso se verifica que este habilitada la feature color
#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;

    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("{color}");
        // draw line with color
    }
}

#[cfg(feature = "shapes")]
pub mod shapes {
    use rgb::RGB;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub color: RGB<u32>,
        pub width: u32,
        pub height: u32,
    }
}
