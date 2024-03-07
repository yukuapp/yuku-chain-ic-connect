use std::{cell::RefCell, collections::HashMap};

#[derive(candid::CandidType, serde::Deserialize, Debug, Default)]
pub struct State {
    pub data: Inner,
}

#[derive(candid::CandidType, serde::Deserialize, Debug)]
pub struct Inner {
    pub max_alive: u64,
    pub users: HashMap<u64, UserPrincipal>,
}

impl Default for Inner {
    fn default() -> Self {
        const DEFAULT_MAX_ALIVE: u64 = 1000000 * 1000 * 60 * 10; // 10 minutes
        Self {
            max_alive: DEFAULT_MAX_ALIVE,
            users: HashMap::new(),
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

#[allow(unused)]
pub fn with_state<F, R>(callback: F) -> R
where
    F: FnOnce(&State) -> R,
{
    STATE.with(|_state| {
        let state = _state.borrow();
        callback(&state)
    })
}

#[allow(unused)]
pub fn with_mut_state<F, R>(callback: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|_state| {
        let mut state = _state.borrow_mut();
        callback(&mut state)
    })
}

pub type TimestampNanos = u64;

#[derive(candid::CandidType, serde::Deserialize, Debug)]

pub struct UserPrincipal {
    pub user: candid::Principal,
    pub created: TimestampNanos,
}

impl UserPrincipal {
    pub fn is_alive(&self, max_alive: u64, now: TimestampNanos) -> bool {
        now < self.created + max_alive
    }
}

impl State {
    #[allow(unused)]
    pub fn max_alive_query(&self) -> u64 {
        self.data.max_alive
    }

    #[allow(unused)]
    pub fn max_alive_update(&mut self, max_alive: u64) -> u64 {
        std::mem::replace(&mut self.data.max_alive, max_alive)
    }

    pub fn user_query(&self, random: u64) -> Option<candid::Principal> {
        self.data.users.get(&random).and_then(|user| {
            let now = ic_cdk::api::time();
            if user.is_alive(self.data.max_alive, now) {
                Some(user.user)
            } else {
                None
            }
        })
    }

    pub fn user_update(&mut self, random: u64, user_principal: UserPrincipal) {
        self.data.users.insert(random, user_principal);
    }

    pub fn user_clean(&mut self, random: u64, user: candid::Principal) {
        if let Some(cached) = self.data.users.get(&random) {
            if cached.user == user {
                self.data.users.remove(&random);
            }
        }
    }

    #[allow(unused)]
    pub fn clean_users(&mut self) {
        let max_alive = self.data.max_alive;
        let now = ic_cdk::api::time();
        self.data
            .users
            .retain(|_, user| user.is_alive(max_alive, now));
    }
}
