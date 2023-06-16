// Las constantes son valores que nunca podran ser mutados, su valor debe ser conocido en tiempo de compilación.
// Pueden declararse en cualquier scope, incluyendo el scope global (fuera de main) y debe tener siempre declarado el tipo.
// El estandar dicta que se use Screaming Snake Case para nombrar las constantes
// Las constantes son reemplazadas por el valor y no ocupan espacio en memoria
const MAX_PLAYERS: u8 = 10;

// Las variables estaticas al igual que las constantes usan Screaming Snake Case, necesitan que su tipo sea declarado y pueden declararse en cualquier scope.
// A diferencia de las constantes pueden ser marcadas como mutables (con la palabra clave "mut") pero para acceder y modificar una variables estatica mutable se deben usar bloques inseguros. Estas sí ocupan lugar en la memoria, y tienen casos de uso muy especificos.
static CASINO_NAME: &str = "Vegas";

fn main() {
    let a = MAX_PLAYERS;
    let b = MAX_PLAYERS;

    let c = CASINO_NAME;
    let d = CASINO_NAME;
}
