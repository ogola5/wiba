#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UserProfile {
    id: u64,
    name: String,
    role: UserRole,
    transaction_history: Vec<TransactionRecord>,
    stake_in_dao: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum UserRole {
    Farmer,
    Consumer,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Farmer
    }
}


#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TransactionRecord {
    id: u64,
    amount: f64,
    date: u64, // Unix timestamp
    involved_parties: Vec<u64>, // User IDs
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct InsuranceContract {
    id: u64,
    farmer_id: u64,
    consumer_id: u64,
    terms: String,
    conditions: String,
    payout_criteria: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct GovernanceProposal {
    id: u64,
    proposal_details: String,
    proposer_id: u64,
    voting_records: Vec<VotingRecord>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct VotingRecord {
    user_id: u64,
    vote: VoteType,
}


#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum VoteType {
    Approve,
    Reject,
    Abstain,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct StakeAdjustment {
    user_id: u64,
    old_stake: f64,
    new_stake: f64,
    reason: String,
}
impl Storable for UserProfile {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for UserProfile {
    const MAX_SIZE: u32 = 2048; // Adjust based on expected data size
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for TransactionRecord {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TransactionRecord {
    const MAX_SIZE: u32 = 1024; // Adjust as necessary
    const IS_FIXED_SIZE: bool = false;
}
impl Storable for InsuranceContract {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for InsuranceContract {
    const MAX_SIZE: u32 = 2048; // Adjust according to your needs
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for GovernanceProposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for GovernanceProposal {
    const MAX_SIZE: u32 = 2048; // Adjust based on expected size
    const IS_FIXED_SIZE: bool = false;
}
impl Storable for StakeAdjustment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for StakeAdjustment {
    const MAX_SIZE: u32 = 1024; // Adjust as needed
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    // Storage for UserProfiles
    static USER_PROFILES: RefCell<StableBTreeMap<u64, UserProfile, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    // Storage for TransactionRecords
    static TRANSACTION_RECORDS: RefCell<StableBTreeMap<u64, TransactionRecord, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    // Storage for InsuranceContracts
    static INSURANCE_CONTRACTS: RefCell<StableBTreeMap<u64, InsuranceContract, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    // Storage for GovernanceProposals
    static GOVERNANCE_PROPOSALS: RefCell<StableBTreeMap<u64, GovernanceProposal, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    // Storage for StakeAdjustments
    static STAKE_ADJUSTMENTS: RefCell<StableBTreeMap<u64, StakeAdjustment, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );
}