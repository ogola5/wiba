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

#[ic_cdk::update]
fn create_transaction_record(
    amount: f64,
    date: u64,
    involved_parties: Vec<u64>,
) -> Option<TransactionRecord> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let transaction_record = TransactionRecord {
        id,
        amount,
        date,
        involved_parties,
    };

    TRANSACTION_RECORDS.with(|records| {
        records.borrow_mut().insert(id, transaction_record.clone());
    });

    Some(transaction_record)
}

#[ic_cdk::query]
fn read_transaction_record(record_id: u64) -> Result<TransactionRecord, Error> {
    if let Some(record) = TRANSACTION_RECORDS.with(|records| records.borrow().get(&record_id)) {
        Ok(record.clone())
    } else {
        Err(Error::NotFound {
            msg: format!("Transaction record with id={} not found", record_id),
        })
    }
}

#[ic_cdk::update]
fn update_transaction_record(
    record_id: u64,
    amount: f64,
    date: u64,
    involved_parties: Vec<u64>,
) -> Result<TransactionRecord, Error> {
    TRANSACTION_RECORDS.with(|records| {
        let mut records = records.borrow_mut();

        // Check if the transaction record exists
        if let Some(mut record) = records.remove(&record_id) {
            // Update the fields
            record.amount = amount;
            record.date = date;
            record.involved_parties = involved_parties;

            // Insert the updated record back into the map
            records.insert(record_id, record.clone());

            // Return the updated record
            Ok(record)
        } else {
            // Transaction record not found
            Err(Error::NotFound {
                msg: format!("Transaction record with id={} not found", record_id),
            })
        }
    })
}

#[ic_cdk::update]
fn delete_transaction_record(record_id: u64) -> Result<TransactionRecord, Error> {
    TRANSACTION_RECORDS
        .with(|records| records.borrow_mut().remove(&record_id))
        .ok_or(Error::NotFound {
            msg: format!("Transaction record with id={} not found", record_id),
        })
}

#[ic_cdk::update]
fn create_insurance_contract(
    farmer_id: u64,
    consumer_id: u64,
    terms: String,
    conditions: String,
    payout_criteria: String,
) -> Option<InsuranceContract> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let insurance_contract = InsuranceContract {
        id,
        farmer_id,
        consumer_id,
        terms,
        conditions,
        payout_criteria,
    };

    INSURANCE_CONTRACTS.with(|contracts| {
        contracts.borrow_mut().insert(id, insurance_contract.clone());
    });

    Some(insurance_contract)
}

#[ic_cdk::query]
fn read_insurance_contract(contract_id: u64) -> Result<InsuranceContract, Error> {
    if let Some(contract) = INSURANCE_CONTRACTS.with(|contracts| contracts.borrow().get(&contract_id)) {
        Ok(contract.clone())
    } else {
        Err(Error::NotFound {
            msg: format!("Insurance contract with id={} not found", contract_id),
        })
    }
}

#[ic_cdk::update]
fn update_insurance_contract(
    contract_id: u64,
    farmer_id: u64,
    consumer_id: u64,
    terms: String,
    conditions: String,
    payout_criteria: String,
) -> Result<InsuranceContract, Error> {
    INSURANCE_CONTRACTS.with(|contracts| {
        let mut contracts = contracts.borrow_mut();

        // Check if the insurance contract exists
        if let Some(mut contract) = contracts.remove(&contract_id) {
            // Update the fields
            contract.farmer_id = farmer_id;
            contract.consumer_id = consumer_id;
            contract.terms = terms;
            contract.conditions = conditions;
            contract.payout_criteria = payout_criteria;

            // Insert the updated contract back into the map
            contracts.insert(contract_id, contract.clone());

            // Return the updated contract
            Ok(contract)
        } else {
            // Insurance contract not found
            Err(Error::NotFound {
                msg: format!("Insurance contract with id={} not found", contract_id),
            })
        }
    })
}

#[ic_cdk::update]
fn delete_insurance_contract(contract_id: u64) -> Result<InsuranceContract, Error> {
    INSURANCE_CONTRACTS
        .with(|contracts| contracts.borrow_mut().remove(&contract_id))
        .ok_or(Error::NotFound {
            msg: format!("Insurance contract with id={} not found", contract_id),
        })
}

#[ic_cdk::update]
fn create_governance_proposal(
    proposal_details: String,
    proposer_id: u64,
    voting_records: Vec<VotingRecord>,
) -> Option<GovernanceProposal> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let governance_proposal = GovernanceProposal {
        id,
        proposal_details,
        proposer_id,
        voting_records,
    };

    GOVERNANCE_PROPOSALS.with(|proposals| {
        proposals.borrow_mut().insert(id, governance_proposal.clone());
    });

    Some(governance_proposal)
}

#[ic_cdk::query]
fn read_governance_proposal(proposal_id: u64) -> Result<GovernanceProposal, Error> {
    if let Some(proposal) = GOVERNANCE_PROPOSALS.with(|proposals| proposals.borrow().get(&proposal_id)) {
        Ok(proposal.clone())
    } else {
        Err(Error::NotFound {
            msg: format!("Governance proposal with id={} not found", proposal_id),
        })
    }
}

#[ic_cdk::update]
fn update_governance_proposal(
    proposal_id: u64,
    proposal_details: String,
    proposer_id: u64,
    voting_records: Vec<VotingRecord>,
) -> Result<GovernanceProposal, Error> {
    GOVERNANCE_PROPOSALS.with(|proposals| {
        let mut proposals = proposals.borrow_mut();

        // Check if the governance proposal exists
        if let Some(mut proposal) = proposals.remove(&proposal_id) {
            // Update the fields
            proposal.proposal_details = proposal_details;
            proposal.proposer_id = proposer_id;
            proposal.voting_records = voting_records;

            // Insert the updated proposal back into the map
            proposals.insert(proposal_id, proposal.clone());

            // Return the updated proposal
            Ok(proposal)
        } else {
            // Governance proposal not found
            Err(Error::NotFound {
                msg: format!("Governance proposal with id={} not found", proposal_id),
            })
        }
    })
}

#[ic_cdk::update]
fn delete_governance_proposal(proposal_id: u64) -> Result<GovernanceProposal, Error> {
    GOVERNANCE_PROPOSALS
        .with(|proposals| proposals.borrow_mut().remove(&proposal_id))
        .ok_or(Error::NotFound {
            msg: format!("Governance proposal with id={} not found", proposal_id),
        })
}

#[ic_cdk::update]
fn create_stake_adjustment(
    user_id: u64,
    old_stake: f64,
    new_stake: f64,
    reason: String,
) -> Option<StakeAdjustment> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let stake_adjustment = StakeAdjustment {
        user_id,
        old_stake,
        new_stake,
        reason,
    };

    STAKE_ADJUSTMENTS.with(|adjustments| {
        adjustments.borrow_mut().insert(id, stake_adjustment.clone());
    });

    Some(stake_adjustment)
}

#[ic_cdk::query]
fn read_stake_adjustment(adjustment_id: u64) -> Result<StakeAdjustment, Error> {
    if let Some(adjustment) = STAKE_ADJUSTMENTS.with(|adjustments| adjustments.borrow().get(&adjustment_id)) {
        Ok(adjustment.clone())
    } else {
        Err(Error::NotFound {
            msg: format!("Stake adjustment with id={} not found", adjustment_id),
        })
    }
}

#[ic_cdk::update]
fn update_stake_adjustment(
    adjustment_id: u64,
    user_id: u64,
    old_stake: f64,
    new_stake: f64,
    reason: String,
) -> Result<StakeAdjustment, Error> {
    STAKE_ADJUSTMENTS.with(|adjustments| {
        let mut adjustments = adjustments.borrow_mut();

        // Check if the stake adjustment exists
        if let Some(mut adjustment) = adjustments.remove(&adjustment_id) {
            // Update the fields
            adjustment.user_id = user_id;
            adjustment.old_stake = old_stake;
            adjustment.new_stake = new_stake;
            adjustment.reason = reason;

            // Insert the updated adjustment back into the map
            adjustments.insert(adjustment_id, adjustment.clone());

            // Return the updated adjustment
            Ok(adjustment)
        } else {
            // Stake adjustment not found
            Err(Error::NotFound {
                msg: format!("Stake adjustment with id={} not found", adjustment_id),
            })
        }
    })
}

#[ic_cdk::update]
fn delete_stake_adjustment(adjustment_id: u64) -> Result<StakeAdjustment, Error> {
    STAKE_ADJUSTMENTS
        .with(|adjustments| adjustments.borrow_mut().remove(&adjustment_id))
        .ok_or(Error::NotFound {
            msg: format!("Stake adjustment with id={} not found", adjustment_id),
        })
}

// need this to generate candid
ic_cdk::export_candid!();
