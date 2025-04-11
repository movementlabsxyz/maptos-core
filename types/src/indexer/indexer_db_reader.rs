// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    account_address::AccountAddress,
    contract_event::{ContractEventV1, ContractEventV2, EventWithVersion},
    event::EventKey,
    state_store::{
        state_key::{prefix::StateKeyPrefix, StateKey},
        state_value::StateValue,
        table::{TableHandle, TableInfo},
    },
    transaction::{AccountOrderedTransactionsWithProof, ReplayProtector, Version},
};
use anyhow::Result;
use aptos_crypto::HashValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Order {
    Ascending,
    Descending,
}

// Question[Orderless]: Do we need any more information here? How about gas_used and block timestamp?
// Question[Orderless]: As this struct is stored in the DB, do changes to this struct break the DB?
// Question[Orderless]: What's the right file to define this struct?
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexedTransactionSummary {
    V1 {
        sender: AccountAddress,
        version: Version,
        transaction_hash: HashValue,
        replay_protector: ReplayProtector,
    },
}

pub trait IndexerReader: Send + Sync {
    fn is_internal_indexer_enabled(&self) -> bool;

    fn get_table_info(&self, handle: TableHandle) -> Result<Option<TableInfo>>;

    fn get_events(
        &self,
        event_key: &EventKey,
        start: u64,
        order: Order,
        limit: u64,
        ledger_version: Version,
    ) -> Result<Vec<EventWithVersion>>;

    fn get_events_by_event_key(
        &self,
        event_key: &EventKey,
        start_seq_num: u64,
        order: Order,
        limit: u64,
        ledger_version: Version,
    ) -> Result<Vec<EventWithVersion>>;

    fn get_account_ordered_transactions(
        &self,
        address: AccountAddress,
        start_seq_num: u64,
        limit: u64,
        include_events: bool,
        ledger_version: Version,
    ) -> Result<AccountOrderedTransactionsWithProof>;

    fn get_prefixed_state_value_iterator(
        &self,
        key_prefix: &StateKeyPrefix,
        cursor: Option<&StateKey>,
        version: Version,
    ) -> Result<Box<dyn Iterator<Item = Result<(StateKey, StateValue)>> + '_>>;

    fn get_latest_internal_indexer_ledger_version(&self) -> Result<Option<Version>>;
    fn get_latest_table_info_ledger_version(&self) -> Result<Option<Version>>;

    #[cfg(any(test, feature = "fuzzing"))]
    fn wait_for_internal_indexer(&self, version: Version) -> Result<()> {
        while self
            .get_latest_internal_indexer_ledger_version()?
            .map_or(true, |v| v < version)
        {
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        Ok(())
    }
    fn get_translated_v1_event_by_version_and_index(
        &self,
        version: Version,
        index: u64,
    ) -> Result<ContractEventV1>;

    fn translate_event_v2_to_v1(&self, v2: &ContractEventV2) -> Result<Option<ContractEventV1>>;
}
