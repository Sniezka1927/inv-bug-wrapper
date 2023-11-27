use scale::Encode as _;

// This file was auto-generated with ink-wrapper (https://crates.io/crates/ink-wrapper).

#[allow(dead_code)]
pub const CODE_HASH: [u8; 32] = [
    29, 24, 200, 24, 110, 220, 67, 212, 217, 141, 188, 3, 133, 0, 239, 218, 41, 104, 205, 103, 168,
    197, 255, 125, 87, 129, 102, 34, 59, 46, 115, 91,
];

#[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
pub enum NoChainExtension {}

pub mod event {
    #[allow(dead_code, clippy::large_enum_variant)]
    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    pub enum Event {}
}

#[derive(Debug, Clone, Copy)]
pub struct Instance {
    account_id: ink_primitives::AccountId,
}

impl From<ink_primitives::AccountId> for Instance {
    fn from(account_id: ink_primitives::AccountId) -> Self {
        Self { account_id }
    }
}

impl From<Instance> for ink_primitives::AccountId {
    fn from(instance: Instance) -> Self {
        instance.account_id
    }
}

impl ink_wrapper_types::EventSource for Instance {
    type Event = event::Event;
}

#[allow(dead_code)]
pub fn upload() -> ink_wrapper_types::UploadCall {
    let wasm = include_bytes!("../test_contract/target/ink/test_contract.wasm");
    ink_wrapper_types::UploadCall::new(wasm.to_vec(), CODE_HASH)
}

impl Instance {
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn new() -> ink_wrapper_types::InstantiateCall<Self> {
        let data = vec![155, 174, 157, 94];
        ink_wrapper_types::InstantiateCall::new(CODE_HASH, data)
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn default() -> ink_wrapper_types::InstantiateCall<Self> {
        let data = vec![237, 75, 157, 27];
        ink_wrapper_types::InstantiateCall::new(CODE_HASH, data)
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn update_timestamp(&self) -> ink_wrapper_types::ExecCall {
        let data = vec![227, 121, 222, 228];
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn get_timestamps(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<(u128, u128), ink_wrapper_types::InkLangError>> {
        let data = vec![133, 163, 73, 165];
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
}
