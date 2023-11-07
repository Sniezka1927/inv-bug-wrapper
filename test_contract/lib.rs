#![cfg_attr(not(feature = "std"), no_std)]
#![no_main]

#[ink::contract]
mod test_contract {
    #[ink(storage)]
    #[derive(Default)]
    pub struct TestContract {
        u32_val: u32,
        last_timestamp: u128,
        value: u128,

    }

    impl TestContract {
        /// Example docs for a constructor.
        /// They are multiline.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                ..Default::default()
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                ..Default::default()
            }
        }
    
        /// Example docs for a message.
        /// They are multiline.
        #[ink(message)]
        pub fn get_u32(&self) -> u32 {
            self.u32_val
        }

        #[ink(message)]
        pub fn set_u32(&mut self, an_u32: u32) {
            self.u32_val = an_u32;
        }

        #[ink(message)]
        pub fn update_timestamp(&mut self) {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;
            let timestamp_delta = 100;

            // let value = 1000000000000000000000000000000 * timestamp_delta / 100;
            let value = (1000000000000000000000000000000 * timestamp_delta) / 1000000000000;

            if value == 0 {
                ink::env::debug_println!("Value is zero");
            }

            self.last_timestamp = current_timestamp as u128;
            self.value = value;
        }

        #[ink(message)]
        pub fn get_timestamps(&self) -> (u128, u128) {
            (self.last_timestamp, self.value)
        }
    }
}
