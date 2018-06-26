use exonum::blockchain::{ExecutionError, Transaction};
use exonum::crypto::PublicKey;
use exonum::storage::Fork;

use service::SERVICE_ID;
use storage::{CryptocurrencySchema, Wallet, INITIAL_BALANCE};

fn error(code: u8, description: &str) -> ExecutionError {
    ExecutionError::with_description(code, description)
}

transactions!{
    pub Transactions {
        const SERVICE_ID = SERVICE_ID;

        struct CreateWallet {
            owner: &PublicKey,
        }

        struct Transfer {
            from: &PublicKey,
            to: &PublicKey,
            amount: u64,
            seed: u32,
        }
    }
}

impl Transaction for CreateWallet {
    fn verify(&self) -> bool {
        true
    }

    fn execute(&self, fork: &mut Fork) -> Result<(), ExecutionError> {
        let mut schema = CryptocurrencySchema::new(fork);

        if schema.wallets().contains(self.owner()) {
            return Err(error(0, "Wallet already exists"));
        } else {
            let wallet = Wallet::new(INITIAL_BALANCE);
            schema.wallets_mut().put(self.owner(), wallet);
            Ok(())
        }
    }
}

impl Transaction for Transfer {
    fn verify(&self) -> bool {
        self.from() != self.to()
    }

    fn execute(&self, fork: &mut Fork) -> Result<(), ExecutionError> {
        let mut schema = CryptocurrencySchema::new(fork);

        let sender = schema.wallets().get(self.from()).ok_or(error(1, "Sender not found"))?;
        let receiver = schema.wallets().get(self.to()).ok_or(error(2, "Receiver not found"))?;

        if sender.balance() > self.amount() {
            let sender = sender.decrease(self.amount());
            let receiver = receiver.increase(self.amount());

            schema.wallets_mut().put(self.from(), sender);
            schema.wallets_mut().put(self.to(), receiver);
            Ok(())
        } else {
            Err(error(3, "Insufficient funds"))
        }
    }
}