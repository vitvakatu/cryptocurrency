use exonum::api::{Api, ApiError};
use exonum::blockchain::{Blockchain, Transaction};
use exonum::crypto::{Hash, PublicKey};
use exonum::node::{ApiSender, TransactionSend};

use bodyparser;
use iron::prelude::*;
use router::Router;
use serde_json;

use storage::{CryptocurrencySchema, Wallet};
use transactions::Transactions;

#[derive(Clone)]
pub struct CryptocurrencyApi {
    channel: ApiSender,
    blockchain: Blockchain,
}

impl CryptocurrencyApi {
    pub fn new(channel: &ApiSender, blockchain: &Blockchain) -> Self {
        Self {
            channel: channel.clone(),
            blockchain: blockchain.clone(),
        }
    }

    fn wire_transaction(self, router: &mut Router) {
        let post_transaction = move |req: &mut Request| -> IronResult<Response> {
            match req.get::<bodyparser::Struct<Transactions>>() {
                Ok(Some(transaction)) => {
                    let transaction: Box<Transaction> = transaction.into();
                    let hash = transaction.hash();
                    self.channel.send(transaction).map_err(ApiError::from)?;
                    self.ok_response(&serde_json::to_value(&hash).unwrap())
                }
                Ok(None) => Err(ApiError::BadRequest("Bad request".to_string()))?,
                Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
            }
        };

        router.post("/v1/transaction", post_transaction, "post_tx");
    }

    fn wire_wallets(self, router: &mut Router) {
        let get_wallets = move |_: &mut Request| -> IronResult<Response> {
            let snapshot = self.blockchain.snapshot();
            let schema = CryptocurrencySchema::new(snapshot);
            let wallets = schema.wallets();
            let wallets: Vec<(PublicKey, Wallet)> = wallets.iter().collect();
            self.ok_response(&serde_json::to_value(&wallets).unwrap())
        };

        router.get("/v1/wallets", get_wallets, "get_wallets");
    }
}

impl Api for CryptocurrencyApi {
    fn wire(&self, router: &mut Router) {
        self.clone().wire_transaction(router);
        self.clone().wire_wallets(router);
    }
}
