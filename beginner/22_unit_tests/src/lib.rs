pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    pub fn new() -> Self {
        Self { balance: 0 }
    }

    pub fn get_balance(&self) -> &i32 {
        &self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Deposit can't be negative");
        }

        self.balance += amount;
    }

    pub fn transfer(&mut self, acc_number: u32, amount: u32) -> Result<String, String> {
        self.balance -= amount as i32;
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }
}

// Este atributo le dice a Rust que corra este codigo cuando se para la configuración test, es decir, "cargo test"
#[cfg(test)]
// Para tests unitarios, lo normal en Rust es crear un modulo que mantenga todas las funciones de test por cada archivo/modulo.
mod tests {
    // Los test unitarios al estar en el mismo modulo que se testea pueden traer campos publicos y privados (los que pueda ver este modulo siguien la conveción de privacidad).
    use super::*;

    // Este atributo convierte la función debajo de ella en un función de test.
    #[test]
    // El nombre de la función se convierte en el nombre del test que aparece en la consola al correr.
    // Un test pasa si no entra en panico o cuando tiene tipo de retorno Result cuando su vairante es OK que mantiene la unidad.
    // Un test falla cuando entra en panico o cuando tiene tipo de retorno Result cuando su variante es Err que mantiene un string.
    // Cuando la función de test no tiene tipo de retorno, el test pasara si el codigo en su interior entra en panico y pasara cuando el codigo corra sin ningun incoveniente.
    fn should_have_a_starting_balance_of_0() {
        let account = SavingsAccount::new();

        // Esta macro permite testear igualdad de dos valores. Tanto esta macro, como asser_ne!, imprimen los argumentos como usando formato de debuggeo, por lo que estos valores deben implementar los traits "PartialEq" y "Debug", una forma facil de hacerlo es usar "#[derive(PartialEq, Debug)]".
        assert_eq!(*account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        // 1. Configure cualquier dato o estado necesario.
        let mut account = SavingsAccount::new();

        // 2. Ejecute el código que desea probar.
        account.deposit(100);

        // 3. Comprueba que los resultados son los esperados.
        // Todas las macros de aserción permiten agregar un mensaje personalizado para el error como parametro opcional, y seguido a este se pasaran argumento que seran usados en un "format" para ese string.
        assert_eq!(
            *account.get_balance(),
            100,
            "Balance should be 100, balance was {} instead",
            account.get_balance()
        );

        // La macro "assert_ne!" permite comprobar que dos valores no son iguales. Esta macro es muy útil en los casos en los que no estamos seguros de cuál será el valor, pero sabemos cuál no debería ser.
        assert_ne!(*account.get_balance(), 0);

        // La macro "assert!" comprueba que una condición sea verdadera. Es decir, debe recibir un booleano.
        assert!(*account.get_balance() != 0);
    }

    #[test]
    #[should_panic(expected = "can't be negative")]
    // Este atributo hace que la función de test pase solo si el codigo dentro de ella entra en panico y de lo contrario falla.
    // Si queremos comprobar que entre en panico con un mensaje especifico se usa el parametro "expected" que recibe un string que debe estar contenido en el mensaje de panico
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingsAccount::new();
        account.deposit(-1);
    }

    #[test]
    // Una función de test tambien puede retornar un Result que fallara cuando retorne la variante Err y pasara cuando retorne la variante Ok. Estas variantes estan limitas a valores que implementen el trait "Termination", por ahora, Result<(), String>
    // No es posible usar "#[should_panic]" con una función de test que retorne Result.
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        // Recordemos que el signo "?" al final de una expresión que es un Result/Option retornara la función si es un Err/None o de lo contario obtendra el valor.
        account.transfer(123456, 55)?;
        Ok(())
    }
}
