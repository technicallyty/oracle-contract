use connect_sdk::bindings::marketmap::types::CurrencyPair;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Price { base: String, quote: String },
    Prices { currency_pair_ids: Vec<String> },
    CurrencyPairs {},

    Params {},
    LastUpdated {},
    MarketMap {},
    Market {
        currency_pair: CurrencyPair,
    },
}
