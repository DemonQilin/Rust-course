use integration_tests::SavingsAccount;

// El directorio "tests" es especial que Cargo proceso unicamente cuando se corre "cargo test" y trata cada archivo directo de esta carpeta como un crate aislado. Unicamente puede importar la interfaz publica de la libreria del paquete.

// En caso de necesitar compartir items entre distintos test, se crea un carpeta que almacena un archivo "mod.rs" y se declara el modulo en el test. Los archivos en subdirectorios no son compilados como crates separados.

// Debido a que no se pueden importar items desde crates binarios es muy comun en Rust ver el patron de un crate binario pequeño con lo minimo necesario y crate de libreria con la mayoria de la funcionalidad con la finalidad de ser testeada.

mod utils;

#[test]
fn should_have_a_starting_balance_of_0() {
    // Invocando función compartida
    utils::setup();
    let account = SavingsAccount::new();
    assert_eq!(*account.get_balance(), 0);
}
