use std::cell::RefCell;

use crate::types::ContextConfigs;
use crate::types::ICPSigned;
use crate::types::Request;

pub mod guard;
pub mod mutate;
pub mod types;

thread_local! {
    pub static CONTEXT_CONFIGS: RefCell<ContextConfigs> = RefCell::new(ContextConfigs::default());
}

#[ic_cdk::init]
fn init() {
    CONTEXT_CONFIGS.with(|state| {
        *state.borrow_mut() = ContextConfigs::default();
    });
}

ic_cdk::export_candid!();