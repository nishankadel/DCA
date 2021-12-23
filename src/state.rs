///into state.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
//deposit tokens
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct DCA {
    pub amount: u64,
    pub sender_account: Pubkey,
    pub admin_pda: Pubkey,
    pub mint_address: Pubkey,
}
