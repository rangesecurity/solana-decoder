use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::raydium::{Programs as RaydiumPrograms, RAYDIUM_DECODER};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use anyhow::{anyhow, Context, Result};
use solana_transaction_status::UiPartiallyDecodedInstruction;

pub static PROGRAM_DECODER_MATCHERS: [
    Lazy<Arc<dyn ProgramDecoderMatcher>>; 1
] = [Lazy::new(|| RAYDIUM_DECODER.clone())];

pub trait ProgramDecoder {
    fn decode(&self) -> anyhow::Result<DecodedInstruction>;
    fn debug(&self) -> String;
}

pub trait ProgramDecoderMatcher: Send + Sync {
    fn try_new(&self, ix: &PartiallyDecodedInstruction) -> Option<Box<dyn ProgramDecoder>>;
}


#[derive(Clone, PartialEq, Eq)]
pub enum Protocols {
    Raydium(RaydiumPrograms),
}


#[derive(Clone)]
pub struct Program {
    pub program_id: Pubkey,
    pub name: String,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DecodedInstruction {
    /// Maps the name of an instruction data input to it's value
    pub data: HashMap<IxDataInput, serde_json::Value>,
    pub accounts: HashMap<IxAccountInput, serde_json::Value>
}


#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct IxAccountInput {
    pub index: u16,
    pub name: String,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct IxDataInput {
    /// Indicates the order in which this input was provided
    /// 0 indicating it is the first instruction data argument
    /// 
    /// 
    /// Realistically a u8 should be fine, but for maximum guarantee
    /// use u16 allowing up to 65k inputs
    pub index: u16,
    pub name: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// Equivalent to UiPartiallyDecodedInstruction but with converted types
pub struct PartiallyDecodedInstruction {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: Vec<u8>,
    pub stack_height: Option<u32>,
}


impl TryFrom<UiPartiallyDecodedInstruction> for PartiallyDecodedInstruction {
    type Error = anyhow::Error;
    fn try_from(value: UiPartiallyDecodedInstruction) -> std::result::Result<Self, Self::Error> {
        let pid = Pubkey::from_str(&value.program_id).with_context(|| "failed to parse program id")?;
        
        let pre_parsed_accounts_len = value.accounts.len();
        
        let accounts = value.accounts.into_iter().filter_map(|account| account.parse::<Pubkey>().ok()) .collect::<Vec<_>>();
        if accounts.len() != pre_parsed_accounts_len {
            return Err(anyhow!("failed to parse one or more accounts"))
        }

        let data = bs58::decode(value.data).into_vec().with_context(|| "failed to decode instruction data")?;
        Ok(Self {
            program_id: pid,
            accounts,
            data,
            stack_height: value.stack_height
        })
    }
}