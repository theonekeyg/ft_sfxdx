use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

use cosmwasm_std::{CanonicalAddr, HumanAddr, Storage, ReadonlyStorage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
// use cosmwasm_storage::prefixed_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

pub const STORAGE_PREFIX: &[u8] = b"prefixed_storage";

pub struct ReadonlyBalances<'a, T: Storage> {
    storage: ReadonlyPrefixedStorage<'a, T>
}

impl<'a, T: Storage> ReadonlyBalances<'a, T> {
    pub fn from_storage(storage: &'a T) -> Self {
        return Self {
            storage: ReadonlyPrefixedStorage::new(STORAGE_PREFIX, storage)
        };
    }

    pub fn get(&self, addr: &CanonicalAddr) -> Option<u128> {
        if let Some(val) = self.storage.get(addr.as_slice()) {
            if let Ok(bytes) = <[u8; 16]>::try_from(val.as_slice()) {
                return Some(u128::from_be_bytes(bytes));
            }
        }
        None
    }
}

pub struct Balances<'a, T: Storage> {
    storage: PrefixedStorage<'a, T>
}

impl<'a, T: Storage> Balances<'a, T> {
    pub fn from_storage(storage: &'a mut T) -> Self {
        return Self {
            storage: PrefixedStorage::new(STORAGE_PREFIX, storage)
        };
    }

    pub fn get(&self, addr: &CanonicalAddr) -> Option<u128> {
        if let Some(val) = self.storage.get(addr.as_slice()) {
            if let Ok(bytes) = <[u8; 16]>::try_from(val.as_slice()) {
                return Some(u128::from_be_bytes(bytes));
            }
        }
        None
    }

    pub fn set(&mut self, addr: &CanonicalAddr, amount: u128) {
        self.storage.set(addr.as_slice(), &amount.to_be_bytes());
    }
}
