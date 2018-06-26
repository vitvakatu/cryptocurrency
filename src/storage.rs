use exonum::crypto::PublicKey;
use exonum::storage::{Fork, MapIndex, Snapshot};

pub const INITIAL_BALANCE: u64 = 100;

encoding_struct!{
    struct Wallet {
        balance: u64,
    }
}

impl Wallet {
    pub fn increase(self, amount: u64) -> Self {
        let balance = self.balance() + amount;
        Wallet::new(balance)
    }

    pub fn decrease(self, amount: u64) -> Self {
        let balance = self.balance() - amount;
        Wallet::new(balance)
    }
}

pub struct CryptocurrencySchema<T> {
    view: T
}

impl<T: AsRef<Snapshot>> CryptocurrencySchema<T> {
    pub fn new(view: T) -> Self {
        Self { view }
    }

    pub fn wallets(&self) -> MapIndex<&Snapshot, PublicKey, Wallet> {
        MapIndex::new("wallets", self.view.as_ref())
    }
}

impl<'a> CryptocurrencySchema<&'a mut Fork> {
    pub fn wallets_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Wallet> {
        MapIndex::new("wallets", self.view)
    }
}