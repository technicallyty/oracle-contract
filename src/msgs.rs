use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Price { base: String, quote: String },
    Prices { currency_pair_ids: Vec<String> },
    CurrencyPairs {},
}
