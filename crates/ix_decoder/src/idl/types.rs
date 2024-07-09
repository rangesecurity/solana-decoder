use {
    super::idl::IdlType,
    anyhow::anyhow,
    serde::{Serialize, Deserialize},
    solana_sdk::pubkey::Pubkey,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ParsedIdl {
    pub instructions: Vec<ParsedIdlInstruction>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParsedIdlInstruction {
    pub name: String,
    pub accounts: Vec<ParsedIdlInstructionAccount>,
    pub args: Vec<ParsedIdlInstructionArgument>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParsedIdlInstructionAccount {
    pub name: String,
    #[serde(alias = "isMut")]
    pub is_mut: bool,
    #[serde(alias = "isSigner")]
    pub is_signer: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParsedIdlInstructionArgument {
    pub name: String,
    #[serde(alias = "type")]
    pub type_: IdlType,
}
