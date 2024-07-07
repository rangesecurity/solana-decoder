//! Instruction decoding functions for the v1 CPMM program

use anyhow::Context;
use ix::AmmInstruction;
use solana_sdk::pubkey::Pubkey;

use crate::types::{DecodedInstruction, IxDataInput, PartiallyDecodedInstruction};

pub mod ix;
pub mod state;

pub const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CpmmDecoder {
    ix: PartiallyDecodedInstruction,
}

impl CpmmDecoder {
    pub fn new(ix: PartiallyDecodedInstruction) -> Self {
        Self { ix }
    }
    pub fn decode(&self) -> anyhow::Result<DecodedInstruction> {
        let ix = AmmInstruction::unpack(&self.ix.data)
            .with_context(|| "failed to decode instruction")?;
        match ix {
            AmmInstruction::Initialize(ix) => Ok(DecodedInstruction {
                data: [
                    (
                        IxDataInput {
                            index: 0 as u16,
                            name: "nonce".to_string(),
                        },
                        serde_json::to_value(ix.nonce)?,
                    ),
                    (
                        IxDataInput {
                            index: 1 as u16,
                            name: "open_time".to_string(),
                        },
                        serde_json::to_value(ix.open_time)?,
                    ),
                ]
                .into_iter()
                .collect(),
                accounts: Default::default(),
            }),

            AmmInstruction::Initialize2(ix) => Ok(DecodedInstruction {
                data: [
                    (
                        IxDataInput {
                            index: 0 as u16,
                            name: "nonce".to_string(),
                        },
                        serde_json::to_value(ix.nonce)?,
                    ),
                    (
                        IxDataInput {
                            index: 1 as u16,
                            name: "open_time".to_string(),
                        },
                        serde_json::to_value(ix.open_time)?,
                    ),
                    (
                        IxDataInput {
                            index: 2 as u16,
                            name: "init_pc_amount".into(),
                        },
                        serde_json::to_value(ix.init_pc_amount)?,
                    ),
                ]
                .into_iter()
                .collect(),
                accounts: Default::default(),
            }),

            AmmInstruction::MonitorStep(ix) => {
                unimplemented!()
            }

            AmmInstruction::Deposit(DepositInstruction) => {
                unimplemented!()
            }
            AmmInstruction::Withdraw(WithdrawInstruction) => {
                unimplemented!()
            }
            AmmInstruction::MigrateToOpenBook {} => {
                unimplemented!()
            }
            AmmInstruction::SetParams(SetParamsInstruction) => {
                unimplemented!()
            }
            AmmInstruction::WithdrawPnl => {
                unimplemented!()
            }
            AmmInstruction::WithdrawSrm(WithdrawSrmInstruction) => {
                unimplemented!()
            }
            AmmInstruction::SwapBaseIn(ix) => Ok(DecodedInstruction {
                data: [
                    (
                        IxDataInput {
                            index: 0 as u16,
                            name: "amount_In".to_string(),
                        },
                        serde_json::to_value(ix.amount_in)?,
                    ),
                    (
                        IxDataInput {
                            index: 1 as u16,
                            name: "open_time".to_string(),
                        },
                        serde_json::to_value(ix.minimum_amount_out)?,
                    ),
                ]
                .into_iter()
                .collect(),
                accounts: Default::default(),
            }),
            AmmInstruction::PreInitialize(PreInitializeInstruction) => {
                unimplemented!()
            }
            AmmInstruction::SwapBaseOut(SwapInstructionBaseOut) => {
                unimplemented!()
            }

            AmmInstruction::SimulateInfo(SimulateInstruction) => {
                unimplemented!()
            }

            AmmInstruction::AdminCancelOrders(AdminCancelOrdersInstruction) => {
                unimplemented!()
            }
            AmmInstruction::CreateConfigAccount => {
                unimplemented!()
            }
            AmmInstruction::UpdateConfigAccount(ConfigArgs) => {
                unimplemented!()
            }
        }
    }
}
