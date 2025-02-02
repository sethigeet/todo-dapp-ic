use candid::CandidType;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

impl Storable for Todo {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        serde_json::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Todo {
    pub fn new(id: u64, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}
