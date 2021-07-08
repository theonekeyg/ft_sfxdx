use cosmwasm_std::{
    Extern, Env, Storage, Api, Querier, StdResult, StdError, Uint128,
    InitResponse, HandleResponse, Binary, to_binary,
    HumanAddr
};

use crate::msg::{InitMsg, HandleMsg, QueryMsg};
use crate::state::{Balances, ReadonlyBalances};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::InitBalance;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{ CanonicalAddr };

    /// Helper function for init routines
    fn init_helper(balances: Option<Vec<InitBalance>>) -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>
    ) {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("instantiator", &[]);
        let init_msg = InitMsg { balances: balances };
        (init(&mut deps, env, init_msg), deps)
    }

    #[test]
    fn test_init() {
        let (init_res, _deps) = init_helper(None);
        assert_eq!(init_res.unwrap(), InitResponse::default());
    }

    #[test]
    fn test_init_with_balances() {
        let (init_res, deps) = init_helper(Some(vec![
            InitBalance { address: HumanAddr("testme".into()),
                          amount: Uint128::from(500u128) }
        ]));
        assert_eq!(init_res.unwrap(), InitResponse::default());
        let balances = ReadonlyBalances::from_storage(&deps.storage);
        // Somehow these two lines below don't produce the same result, the first one
        // is failing the test.
        // let addr = CanonicalAddr::from(b"testme" as &[u8]);
        let addr = deps.api.canonical_address(&HumanAddr("testme".into())).unwrap();
        assert_eq!(500, balances.get(&addr).unwrap_or_default());
    }

    #[test]
    fn test_transfer() {
        let (init_res, mut deps) = init_helper(Some(vec![
            InitBalance { address: HumanAddr("sender".into()),
                          amount: Uint128::from(500u128) },
            InitBalance { address: HumanAddr("receiver".into()),
                          amount: Uint128::from(100u128) }
        ]));
        assert_eq!(init_res.unwrap(), InitResponse::default());
        let env = mock_env("sender", &[]);
        let msg = HandleMsg::Transfer {
            to: HumanAddr("receiver".into()), amount: Uint128::from(150u128)
        };
        let handle_res = handle(&mut deps, env, msg);
        assert_eq!(handle_res.unwrap(), HandleResponse::default());

        let balances = ReadonlyBalances::from_storage(&deps.storage);
        let send_addr = deps.api.canonical_address(&HumanAddr("sender".into())).unwrap();
        let recv_addr = deps.api.canonical_address(&HumanAddr("receiver".into())).unwrap();
        assert_eq!(balances.get(&send_addr).unwrap_or_default(), 350);
        assert_eq!(balances.get(&recv_addr).unwrap_or_default(), 250);
    }

    #[test]
    fn test_burn() {
        let (init_res, mut deps) = init_helper(Some(vec![
            InitBalance { address: HumanAddr("burner".into()),
                          amount: Uint128::from(1000u128) },
        ]));
        assert_eq!(init_res.unwrap(), InitResponse::default());
        let env = mock_env("burner", &[]);
        let msg = HandleMsg::Burn { amount: Uint128::from(350u128) };
        let handle_res = handle(&mut deps, env, msg);
        assert_eq!(handle_res.unwrap(), HandleResponse::default());

        let balances = ReadonlyBalances::from_storage(&deps.storage);
        let addr = deps.api.canonical_address(&HumanAddr("burner".into())).unwrap();
        assert_eq!(balances.get(&addr).unwrap_or_default(), 650);
   }

    #[test]
    fn test_burn_overflow() {
        let (init_res, mut deps) = init_helper(Some(vec![
            InitBalance { address: HumanAddr("burner".into()),
                          amount: Uint128::from(250u128) },
        ]));
        assert_eq!(init_res.unwrap(), InitResponse::default());
        let env = mock_env("burner", &[]);
        let msg = HandleMsg::Burn { amount: Uint128::from(500u128) };
        let handle_res = handle(&mut deps, env, msg);
        assert_eq!(handle_res.err(), Some(StdError::generic_err("overflow occurred")));
    }
}
