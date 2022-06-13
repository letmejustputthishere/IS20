use crate::ledger::Ledger;
use crate::types::{Allowances, AuctionInfo, StatsData, Timestamp};

use candid::{CandidType, Deserialize, Nat, Principal};
use common::types::Metadata;
use ic_storage::stable::Versioned;
use ic_storage::IcStorage;
use std::collections::{BTreeSet, HashMap};

#[derive(Default, CandidType, Deserialize, IcStorage)]
pub struct CanisterState {
    pub(crate) bidding_state: BiddingState,
    pub(crate) balances: Balances,
    pub(crate) balances_tree: BalancesTree,
    pub(crate) auction_history: AuctionHistory,
    pub(crate) stats: StatsData,
    pub(crate) allowances: Allowances,
    pub(crate) ledger: Ledger,
}

impl CanisterState {
    pub fn get_metadata(&self) -> Metadata {
        Metadata {
            logo: self.stats.logo.clone(),
            name: self.stats.name.clone(),
            symbol: self.stats.symbol.clone(),
            decimals: self.stats.decimals,
            totalSupply: self.stats.total_supply.clone(),
            owner: self.stats.owner,
            fee: self.stats.fee.clone(),
            feeTo: self.stats.fee_to,
            isTestToken: Some(self.stats.is_test_token),
        }
    }

    pub fn allowance(&self, owner: Principal, spender: Principal) -> Nat {
        match self.allowances.get(&owner) {
            Some(inner) => match inner.get(&spender) {
                Some(value) => value.clone(),
                None => Nat::from(0),
            },
            None => Nat::from(0),
        }
    }

    pub fn allowance_size(&self) -> usize {
        self.allowances
            .iter()
            .map(|(_, v)| v.len())
            .reduce(|accum, v| accum + v)
            .unwrap_or(0)
    }

    pub fn user_approvals(&self, who: Principal) -> Vec<(Principal, Nat)> {
        match self.allowances.get(&who) {
            Some(allow) => Vec::from_iter(allow.clone().into_iter()),
            None => Vec::new(),
        }
    }
}
impl Versioned for CanisterState {
    type Previous = ();

    fn upgrade((): ()) -> Self {
        Self::default()
    }
}

#[derive(Default, CandidType, Deserialize)]
pub struct Balances(pub HashMap<Principal, Nat>);

impl Balances {
    pub fn balance_of(&self, who: &Principal) -> Nat {
        self.0.get(who).cloned().unwrap_or_else(|| Nat::from(0))
    }
}

#[derive(Default, CandidType, Deserialize)]
pub struct BalancesTree(pub BTreeSet<(Nat, Principal)>);

impl BalancesTree {
    pub fn get_holders(&self, start: usize, limit: usize) -> Vec<(Principal, Nat)> {
        let mut balance = Vec::new();
        for (i, (v, k)) in self.0.iter().rev().enumerate() {
            if i >= start && i < start + limit {
                balance.push((*k, v.clone()));
            }
            if i >= start + limit {
                break;
            }
        }
        balance
    }

    pub fn get_holders_between(&self, max: Nat, min: Nat) -> Vec<(Principal, Nat)> {
        let min_principal = Principal::from_slice(&[]);
        let max_principal = Principal::from_slice(&[
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]);
        let max_new = max.clone().max(min.clone());
        let min_new = min.min(max);
        let balance = self
            .0
            .range((min_new, min_principal)..=(max_new, max_principal))
            .rev()
            .map(|(v, k)| (*k, v.clone()))
            .collect();
        balance
    }
}

#[derive(CandidType, Default, Debug, Clone, Deserialize)]
pub struct BiddingState {
    pub fee_ratio: f64,
    pub last_auction: Timestamp,
    pub auction_period: Timestamp,
    pub cycles_since_auction: u64,
    pub bids: HashMap<Principal, u64>,
}

impl BiddingState {
    pub fn is_auction_due(&self) -> bool {
        let curr_time = ic_canister::ic_kit::ic::time();
        let next_auction = self.last_auction + self.auction_period;
        curr_time >= next_auction
    }
}

#[derive(Default, CandidType, Deserialize)]
pub struct AuctionHistory(pub Vec<AuctionInfo>);
