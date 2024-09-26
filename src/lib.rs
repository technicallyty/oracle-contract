use crate::msgs::QueryMsg;
use connect_sdk::bindings::oracle::query::{
    GetAllCurrencyPairsResponse, GetPriceResponse, GetPricesResponse,
};
use connect_sdk::bindings::query::ConnectQuery;
use connect_sdk::bindings::querier::ConnectQuerier;

mod msgs;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, to_json_binary
};

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
    }
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
