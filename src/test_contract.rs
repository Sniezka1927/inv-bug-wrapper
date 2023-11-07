use scale::Encode as _;

// This file was auto-generated with ink-wrapper (https://crates.io/crates/ink-wrapper).

#[allow(dead_code)]
pub const CODE_HASH: [u8; 32] = [
    242, 81, 130, 86, 84, 165, 109, 126, 74, 239, 165, 179, 228, 52, 29, 131, 63, 233, 226, 92, 82,
    83, 233, 5, 165, 255, 77, 229, 183, 147, 51, 59,
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
    /// Example docs for a constructor.
    /// They are multiline.
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

    ///  Example docs for a message.
    ///  They are multiline.
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn get_u32(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<u32, ink_wrapper_types::InkLangError>> {
        let data = vec![217, 45, 11, 204];
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn set_u32(&self, an_u32: u32) -> ink_wrapper_types::ExecCall {
        let data = {
            let mut data = vec![246, 7, 184, 246];
            an_u32.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
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
