use solana_sdk::pubkey::Pubkey;

pub mod ix;
pub mod matching;


pub const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SerumDecoder {
    
}