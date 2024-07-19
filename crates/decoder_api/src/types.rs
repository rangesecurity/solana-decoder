use {
    serde::{Deserialize, Serialize}, solana_transaction_status::{UiInstruction, UiParsedInstruction, UiPartiallyDecodedInstruction},
};


#[derive(Serialize, Deserialize)]
pub struct DecodeInstruction {
    pub data: String,
    pub accounts: Vec<String>,
    #[serde(alias = "programId")]
    pub program_id: String,
    #[serde(alias = "stackHeight")]
    pub stack_height: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub msg: String,
}

impl Into<UiPartiallyDecodedInstruction> for DecodeInstruction {
    fn into(self) -> UiPartiallyDecodedInstruction {
        UiPartiallyDecodedInstruction {
            data: self.data,
            accounts: self.accounts,
            program_id: self.program_id,
            stack_height: self.stack_height
        }
    }
}

impl Into<UiInstruction> for DecodeInstruction {
    fn into(self) -> UiInstruction {
        let partial: UiPartiallyDecodedInstruction = Into::into(self);
        UiInstruction::Parsed(UiParsedInstruction::PartiallyDecoded(partial))
    }
}