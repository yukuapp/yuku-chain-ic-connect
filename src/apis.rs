use candid::Principal;

use crate::stable::*;

// ================== canister ==================

#[ic_cdk::query]
fn user_query(random: u64) -> Option<Principal> {
    with_state(|s| s.user_query(random))
}

#[ic_cdk::update]
async fn user_update() -> u64 {
    let caller = ic_cdk::caller();

    if caller == Principal::anonymous() {
        panic!("Anonymous caller is rejected.")
    }

    async fn random() -> [u8; 32] {
        let random = ic_cdk::api::management_canister::main::raw_rand()
            .await
            .unwrap()
            .0;

        let mut data = [0; 32];
        for i in 0..32 {
            data[i] = random[i];
        }
        data
    }

    let random = random().await;
    let mut bytes: [u8; 8] = [0; 8];
    bytes.copy_from_slice(&random[0..8]);
    let random = u64::from_be_bytes(bytes);

    let now = ic_cdk::api::time();
    let user_principal = UserPrincipal {
        user: caller,
        created: now,
    };

    with_mut_state(|s| s.user_update(random, user_principal));

    random
}

#[ic_cdk::update]
async fn user_clean(random: u64, user: candid::Principal) {
    with_mut_state(|s| s.user_clean(random, user));
}

// ================== common ==================

#[ic_cdk::query]
pub fn wallet_balance() -> candid::Nat {
    candid::Nat::from(ic_cdk::api::canister_balance128())
}

#[ic_cdk::update]
async fn canister_status() -> ic_cdk::api::management_canister::main::CanisterStatusResponse {
    ic_cdk::api::management_canister::main::canister_status(
        ic_cdk::api::management_canister::main::CanisterIdRecord {
            canister_id: ic_cdk::api::id(),
        },
    )
    .await
    .unwrap()
    .0
}

#[ic_cdk::query]
async fn whoami() -> Principal {
    ic_cdk::api::caller()
}

#[ic_cdk::query]
#[cfg(not(test))]
fn __get_candid_interface_tmp_hack() -> String {
    candid::export_service!();
    __export_service()
}
