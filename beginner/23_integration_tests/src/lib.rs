pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Create a `SavingsAccount` with a balance of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use integration_tests::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(*account.get_balance(), 0);
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_a_starting_balance_of_0() {
        let account = SavingsAccount::new();
        assert_eq!(*account.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        assert_eq!(
            *account.get_balance(),
            100,
            "Balance should be 100, balance was {} instead",
            account.get_balance()
        );
        assert_ne!(*account.get_balance(), 0);
        assert!(*account.get_balance() != 0);
    }

    #[test]
    #[should_panic(expected = "can't be negative")]
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingsAccount::new();
        account.deposit(-1);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();
        account.deposit(100);
        account.transfer(123456, 55)?;
        Ok(())
    }
}
