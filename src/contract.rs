use cosmwasm_std::{
    Extern, Env, Storage, Api, Querier, StdResult, StdError, Uint128,
    InitResponse, HandleResponse, QueryResponse, Binary, to_binary,
    HumanAddr};

use crate::msg::{InitMsg, HandleMsg, QueryMsg};
use crate::state::{Balances, ReadonlyBalances};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg
) -> StdResult<InitResponse> {
    let mut balances_storage = Balances::from_storage(&mut deps.storage);
    if let Some(balances) = msg.balances {
        for balance in balances {
            balances_storage.set(
                &deps.api.canonical_address(&balance.address)?,
                balance.amount.u128()
            );
        }
    }
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Transfer { to, amount } =>
            return handle_transfer(deps, env, &to, amount),
        HandleMsg::Burn { amount } => return handle_burn(deps, env, amount),
    }
    Ok(HandleResponse::default())
}

pub fn handle_transfer<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    to: &HumanAddr,
    amount: Uint128
) -> StdResult<HandleResponse> {
    let mut balances = Balances::from_storage(&mut deps.storage);

    let sender_addr = deps.api.canonical_address(&env.message.sender)?;
    let sender_balance = balances.get(&sender_addr).unwrap_or_default();

    /* Check for overflows */
    if let Some(new_sender_balance) = sender_balance.checked_sub(amount.u128()) {
        let recv_addr = deps.api.canonical_address(to)?;
        let recv_balance = balances.get(&recv_addr).unwrap_or_default();
        if let Some(new_recv_balance) = recv_balance.checked_add(amount.u128()) {
            balances.set(&sender_addr, new_sender_balance);
            balances.set(&recv_addr, new_recv_balance);
            return Ok(HandleResponse::default());
        }
    }
    Err(StdError::generic_err("overflow occurred"))
}

pub fn handle_burn<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: Uint128
) -> StdResult<HandleResponse> {
    let mut balances = Balances::from_storage(&mut deps.storage);
    let sender_addr = deps.api.canonical_address(&env.message.sender)?;
    let sender_balance = balances.get(&sender_addr).unwrap_or_default();
    if let Some(new_sender_balance) = sender_balance.checked_sub(amount.u128()) {
        balances.set(&sender_addr, new_sender_balance);
        return Ok(HandleResponse::default());
    }
    Err(StdError::generic_err("overflow occurred"))
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => return query_balance(deps, &address),
    }
    Ok(QueryResponse::default())
}

pub fn query_balance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    addr: &HumanAddr
) -> StdResult<Binary> {
    let balances = ReadonlyBalances::from_storage(&deps.storage);
    if let Some(owner_balance) = balances.get(&deps.api.canonical_address(addr)?) {
        return Ok(to_binary(&Uint128::from(owner_balance))?);
    }
    Err(StdError::not_found("Requested address wasn't found in global chain"))
}
