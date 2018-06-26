use exonum::api::Api;
use exonum::blockchain::{ApiContext, Service, Transaction, TransactionSet};
use exonum::crypto::Hash;
use exonum::encoding::Error;
use exonum::helpers::fabric::{Context, ServiceFactory};
use exonum::messages::RawMessage;
use exonum::storage::Snapshot;

use iron::Handler;
use router::Router;

use api::CryptocurrencyApi;
use transactions::Transactions;

pub const SERVICE_ID: u16 = 128;
const SERVICE_NAME: &str = "cryptocurrency";

pub struct CryptocurrencyService;

impl Service for CryptocurrencyService {
    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn service_name(&self) -> &str {
        SERVICE_NAME
    }

    fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
        Vec::new()
    }

    fn tx_from_raw(&self, raw: RawMessage) -> Result<Box<Transaction>, Error> {
        Ok(Transactions::tx_from_raw(raw)?.into())
    }

    fn public_api_handler(&self, context: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = CryptocurrencyApi::new(&context.node_channel(), &context.blockchain());
        api.wire(&mut router);
        Some(Box::new(router))
    }
}

pub struct CryptocurrencyServiceFactory;

impl ServiceFactory for CryptocurrencyServiceFactory {
    fn make_service(&mut self, run_context: &Context) -> Box<Service> {
        Box::new(CryptocurrencyService)
    }
}
