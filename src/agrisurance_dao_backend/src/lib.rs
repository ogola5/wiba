#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
//use ic_cdk::api::time;
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

#[ic_cdk::update]
fn create_user_profile(name: String, role: UserRole, stake_in_dao: f64) -> Option<UserProfile> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let user_profile = UserProfile {
        id,
        name,
        role,
        transaction_history: Vec::new(),
        stake_in_dao,
    };

    USER_PROFILES.with(|profiles| {
        profiles.borrow_mut().insert(id, user_profile.clone());
    });

    Some(user_profile)
}

#[ic_cdk::query]
fn read_user_profile(user_id: u64) -> Result<UserProfile, Error> {
    if let Some(profile) = USER_PROFILES.with(|profiles| profiles.borrow().get(&user_id)) {
        Ok(profile.clone())
    } else {
        Err(Error::NotFound {
            msg: format!("User profile with id={} not found", user_id),
        })
    }
}
#[ic_cdk::update]
fn update_user_profile(user_id: u64, name: String, stake_in_dao: f64) -> Result<UserProfile, Error> {
    USER_PROFILES.with(|profiles| {
        let mut profiles = profiles.borrow_mut();

        // Check if the user profile exists
        if let Some(mut profile) = profiles.remove(&user_id) {
            // Update the fields
            profile.name = name;
            profile.stake_in_dao = stake_in_dao;

            // Insert the updated profile back into the map
            profiles.insert(user_id, profile.clone());

            // Return the updated profile
            Ok(profile)
        } else {
            // User profile not found
            Err(Error::NotFound {
                msg: format!("User profile with id={} not found", user_id),
            })
        }
    })
}

#[ic_cdk::update]
fn delete_user_profile(user_id: u64) -> Result<UserProfile, Error> {
    USER_PROFILES
        .with(|profiles| profiles.borrow_mut().remove(&user_id))
        .ok_or(Error::NotFound {
            msg: format!("User profile with id={} not found", user_id),
        })
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

// Similar CRUD operations can be implemented for TransactionRecord, InsuranceContract, GovernanceProposal, and StakeAdjustment
// need this to generate candid
ic_cdk::export_candid!();