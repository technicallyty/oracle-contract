use connect_sdk::bindings::marketmap::query::{LastUpdatedResponse, MarketMapResponse, MarketResponse, ParamsResponse};
use crate::msgs::QueryMsg;
use connect_sdk::bindings::oracle::query::{GetAllCurrencyPairsResponse, GetPriceResponse, GetPricesResponse};
use connect_sdk::bindings::marketmap::types::CurrencyPair;
use connect_sdk::bindings::query::ConnectQuery;
use connect_sdk::bindings::querier::ConnectQuerier;
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, to_json_binary, Int128, StdError};

mod msgs;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps<ConnectQuery>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Price { base, quote } => {
            to_json_binary(&query_price(deps, base, quote)?)
        }
        QueryMsg::Prices { currency_pair_ids } => {
            to_json_binary(&query_prices(deps, currency_pair_ids)?)
        }
        QueryMsg::CurrencyPairs {} => {
            to_json_binary(&query_currency_pairs(deps)?)
        }
        QueryMsg::Params {} => {
            to_json_binary(&query_market_map_params(deps)?)
        }
        QueryMsg::LastUpdated {} => {
            to_json_binary(&query_last_updated(deps)?)
        }
        QueryMsg::MarketMap {} => {
            to_json_binary(&query_market_map(deps)?)
        }
        QueryMsg::Market { currency_pair } => {
            to_json_binary(&query_market(deps, currency_pair)?)
        }
    }
}
fn do_something_with_price(
    deps: Deps<ConnectQuery>,
    env: Env,
    currency_pair: CurrencyPair
) -> StdResult<Int128> {
    let connect_querier = ConnectQuerier::new(&deps.querier);
    let base = currency_pair.base.to_uppercase();
    let quote = currency_pair.quote.to_uppercase();

    // Check if the market exists and is enabled
    let market = connect_querier.get_marketmap_market(base.clone(), quote.clone())?;
    if !market.market.ticker.enabled {
        return Err(StdError::generic_err("market is not enabled"));
    }

    // Check price validity
    let price_response = connect_querier.get_oracle_price(base, quote)?;
    if price_response.nonce == 0 {
        return Err(StdError::generic_err("price has never been updated"));
    }

    let max_price_age: u64 = 3; // adjust based on appetite for price freshness
    let price_age = env.block.height - price_response.price.block_height.unwrap();
    if price_age > max_price_age {
        return Err(StdError::generic_err("price is too old"));
    }

    // We can now do something with the price
    let valid_price = price_response.price.price;

    // Placeholder for actual price processing
    Ok(valid_price)
}

fn query_market_map_params(
    deps: Deps<ConnectQuery>,
) -> StdResult<ParamsResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_marketmap_params()
}

fn query_last_updated(
    deps: Deps<ConnectQuery>,
) -> StdResult<LastUpdatedResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_marketmap_last_updated()
}

fn query_market_map(
    deps: Deps<ConnectQuery>,
) -> StdResult<MarketMapResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_marketmap_market_map()
}

fn query_market(
    deps: Deps<ConnectQuery>,
    currency_pair: CurrencyPair
) -> StdResult<MarketResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_marketmap_market(currency_pair.base, currency_pair.quote)
}

fn query_currency_pairs(
    deps: Deps<ConnectQuery>,
) -> StdResult<GetAllCurrencyPairsResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_oracle_all_currency_pairs()
}

fn query_prices(
    deps: Deps<ConnectQuery>,
    currency_pair_ids: Vec<String>,
) -> StdResult<GetPricesResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_oracle_prices(currency_pair_ids)
}

fn query_price(
    deps: Deps<ConnectQuery>,
    base: String,
    quote: String,
) -> StdResult<GetPriceResponse> {
    let connect_querier = ConnectQuerier::new(&deps.querier);

    connect_querier.get_oracle_price(base, quote)
}
