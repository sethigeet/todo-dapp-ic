use candid::CandidType;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
pub struct Todo {
    pub id: String,
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
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            completed: false,
        }
    }
}
