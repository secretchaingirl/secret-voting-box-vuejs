use std::any::type_name;
use cosmwasm_std::{CanonicalAddr, StdResult, StdError, ReadonlyStorage, Storage};
use cosmwasm_storage::{
   singleton, singleton_read, 
   ReadonlySingleton, Singleton,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use secret_toolkit::serialization::{Bincode2, Serde};


pub static CONFIG_KEY: &[u8] = b"config";
pub static PROPOSALS_KEY: &[u8] = b"proposals";
pub static VOTERS_KEY: &[u8] = b"voters";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: CanonicalAddr,
    pub proposal_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Proposal {
    pub name: String,
    pub creator: CanonicalAddr,
    pub status: u64,
    pub num_votes: u64,
    pub tally_yes: u64,
    pub tally_no: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vote {
    pub proposal_id: u64,
    pub status: bool,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn save<T: Serialize, S: Storage>(
    storage: &mut S,
    key: &[u8],
    value: &T
) -> StdResult<()> {
    storage.set(key, &Bincode2::serialize(value)?);
   Ok(())
}

pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8]
) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
        .get(key)
        .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
}

pub fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
    match storage.get(key) {
        Some(value) => Bincode2::deserialize(&value).map(Some),
        None => Ok(None),
    }
}

