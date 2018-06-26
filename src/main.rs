#[macro_use]
extern crate exonum;
extern crate bodyparser;
extern crate env_logger;
extern crate iron;
extern crate router;
extern crate serde_json;

use exonum::helpers::fabric::NodeBuilder;
use service::CryptocurrencyServiceFactory;

mod api;
mod service;
mod storage;
mod transactions;

fn main() {
    env_logger::init();
    NodeBuilder::new().with_service(
        Box::new(CryptocurrencyServiceFactory)
    ).run();
}
