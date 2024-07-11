//! Instruction decoding functions for the v1 CPMM program

use anyhow::{anyhow, Context};
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
                    ("nonce".to_string(), serde_json::to_value(ix.nonce)?),
                    ("openTime".to_string(), serde_json::to_value(ix.open_time)?),
                ]
                .into_iter()
                .collect(),
                accounts: Default::default(),
                name: "initialize".to_string(),
            }),

            AmmInstruction::Initialize2(ix) => Ok(DecodedInstruction {
                data: [
                    ("nonce".to_string(), serde_json::to_value(ix.nonce)?),
                    ("openTime".to_string(), serde_json::to_value(ix.open_time)?),
                    (
                        "initPcAmount".to_string(),
                        serde_json::to_value(ix.init_pc_amount)?,
                    ),
                ]
                .into_iter()
                .collect(),
                accounts: Default::default(),
                name: "initialize2".to_string(),
            }),

            AmmInstruction::MonitorStep(ix) => Ok(DecodedInstruction {
                data: [
                    (
                        "planOrderLimit".to_string(),
                        serde_json::to_value(ix.plan_order_limit)?,
                    ),
                    (
                        "placeOrderLimit".to_string(),
                        serde_json::to_value(ix.place_order_limit)?,
                    ),
                    (
                        "cancelOrderLimit".to_string(),
                        serde_json::to_value(ix.cancel_order_limit)?,
                    ),
                ]
                .into_iter()
                .collect(),
                accounts: Default::default(),
                name: "monitorStep".to_string(),
            }),

            AmmInstruction::Deposit(ix) => Ok(DecodedInstruction {
                data: [
                    (
                        "maxCoinAmount".to_string(),
                        serde_json::to_value(ix.max_coin_amount)?,
                    ),
                    (
                        "maxPcAmount".to_string(),
                        serde_json::to_value(ix.max_pc_amount)?,
                    ),
                    ("baseSide".to_string(), serde_json::to_value(ix.base_side)?),
                ]
                .into_iter()
                .collect(),
                accounts: self
                    .ix
                    .accounts
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, account)| {
                        let account_name = match idx {
                            0 => "tokenProgram",
                            1 => "amm",
                            2 => "ammAuthority",
                            3 => "ammOpenOrders",
                            4 => "ammTargetOrders",
                            5 => "poolLpTokenMint",
                            6 => "poolCoinTokenAccount",
                            7 => "poolPcTokenAccount",
                            8 => "dexMarket",
                            9 => "userCoinTokenAccount",
                            10 => "userPcTokenAccount",
                            11 => "userLpTokenAccount",
                            12 => "user",
                            13 => "dexEventQueue",
                            _ => return None,
                        }
                        .to_string();
                        Some((account_name, serde_json::to_value(account.to_string()).ok()?))
                    })
                    .collect(),
                name: "deposit".to_string(),
            }),

            AmmInstruction::Withdraw(ix) => Ok(DecodedInstruction {
                data: [(
                    "planOrderLimit".to_string(),
                    serde_json::to_value(ix.amount)?,
                )]
                .into_iter()
                .collect(),
                accounts: self
                    .ix
                    .accounts
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, account)| {
                        let account_name = match idx {
                            0 => "tokenProgram",
                            1 => "amm",
                            2 => "ammAuthority",
                            3 => "ammOpenOrders",
                            4 => "ammTargetOrders",
                            5 => "poolLpTokenMint",
                            6 => "poolCoinTokenAccount",
                            7 => "poolPcTokenAccount",
                            8 => "dexProgram",
                            9 => "dexMarket",
                            10 => "dexCoinTokenAccount",
                            11 => "dexPcTokenAccount",
                            12 => "dexAuthority",
                            13 => "userLpTokenAccount",
                            14 => "userCoinTokenAccount",
                            15 => "userPcTokenAccount",
                            16 => "user",
                            17 => "dexEventQueue",
                            18 => "dexBids",
                            19 => "dexAsks",
                            _ => return None,
                        }
                        .to_string();
                        Some((account_name, serde_json::to_value(account.to_string()).ok()?))
                    })
                    .collect(),
                name: "withdraw".to_string(),
            }),

            AmmInstruction::MigrateToOpenBook {} => Ok(DecodedInstruction {
                name: "migrateToOpenBook".to_string(),
                ..Default::default()
            }),
            AmmInstruction::SetParams(ix) => Err(anyhow!("unimplemented")),
            AmmInstruction::WithdrawPnl => Err(anyhow!("unimplemented")),
            AmmInstruction::WithdrawSrm(WithdrawSrmInstruction) => Err(anyhow!("unimplemented")),
            AmmInstruction::SwapBaseIn(ix) => Ok(DecodedInstruction {
                data: [
                    ("amountIn".to_string(), serde_json::to_value(ix.amount_in)?),
                    (
                        "minimumAmountOut".to_string(),
                        serde_json::to_value(ix.minimum_amount_out)?,
                    ),
                ]
                .into_iter()
                .collect(),
                accounts: {
                    // swap instruction has optional targetOrders account
                    fn account_match_with_target_orders(
                        idx: usize,
                        account: &Pubkey,
                    ) -> Option<(String, serde_json::Value)> {
                        let account_name = match idx {
                            0 => "tokenProgram",
                            1 => "amm",
                            2 => "ammAuthority",
                            3 => "ammOpenOrders",
                            4 => "ammTargetOrders",
                            5 => "poolCoinTokenAccount",
                            6 => "poolPcTokenAccount",
                            7 => "dexProgram",
                            8 => "dex",
                            9 => "dexBids",
                            10 => "dexAsks",
                            11 => "dexEventQueue",
                            12 => "dexCoinTokenAccount",
                            13 => "dexPcTokenAccount",
                            14 => "dexAuthority",
                            15 => "userInputTokenAccount",
                            16 => "userOutputTokenAccount",
                            17 => "user",
                            _ => return None,
                        }
                        .to_string();
                        Some((account_name, serde_json::to_value(account.to_string()).ok()?))
                    }
                        // swap instruction has optional targetOrders account
                        fn account_match_without_target_orders(
                            idx: usize,
                            account: &Pubkey,
                        ) -> Option<(String, serde_json::Value)> {
                            let account_name = match idx {
                                0 => "tokenProgram",
                                1 => "amm",
                                2 => "ammAuthority",
                                3 => "ammOpenOrders",
                                4 => "poolCoinTokenAccount",
                                5 => "poolPcTokenAccount",
                                6 => "dexProgram",
                                7 => "dex",
                                8 => "dexBids",
                                9 => "dexAsks",
                                10 => "dexEventQueue",
                                11 => "dexCoinTokenAccount",
                                12 => "dexPcTokenAccount",
                                13 => "dexAuthority",
                                14 => "userInputTokenAccount",
                                15 => "userOutputTokenAccount",
                                16 => "user",
                                _ => return None,
                            }
                            .to_string();
                            Some((account_name, serde_json::to_value(account.to_string()).ok()?))
                        }
                    let accounts_len = self.ix.accounts.len();
                    self.ix.accounts.iter().enumerate().filter_map(|(idx, account)| {
                        if accounts_len == 18 {
                            // target orders account was supplied
                            account_match_with_target_orders(idx, account)
                        } else {
                            account_match_without_target_orders(idx, account)
                        }
                    }).collect()
                },
                name: "swapBaseIn".to_string(),
            }),
            AmmInstruction::PreInitialize(PreInitializeInstruction) => {
                Err(anyhow!("unimplemented"))
            }
            AmmInstruction::SwapBaseOut(ix) => Ok(DecodedInstruction {
                data: [
                    (
                        "maxAmountIn".to_string(),
                        serde_json::to_value(ix.max_amount_in)?,
                    ),
                    (
                        "amountOut".to_string(),
                        serde_json::to_value(ix.amount_out)?,
                    ),
                ]
                .into_iter()
                .collect(),
                accounts: {
                    // swap instruction has optional targetOrders account
                    fn account_match_with_target_orders(
                        idx: usize,
                        account: &Pubkey,
                    ) -> Option<(String, serde_json::Value)> {
                        let account_name = match idx {
                            0 => "tokenProgram",
                            1 => "amm",
                            2 => "ammAuthority",
                            3 => "ammOpenOrders",
                            4 => "ammTargetOrders",
                            5 => "poolCoinTokenAccount",
                            6 => "poolPcTokenAccount",
                            7 => "dexProgram",
                            8 => "dex",
                            9 => "dexBids",
                            10 => "dexAsks",
                            11 => "dexEventQueue",
                            12 => "dexCoinTokenAccount",
                            13 => "dexPcTokenAccount",
                            14 => "dexAuthority",
                            15 => "userInputTokenAccount",
                            16 => "userOutputTokenAccount",
                            17 => "user",
                            _ => return None,
                        }
                        .to_string();
                        Some((account_name, serde_json::to_value(account.to_string()).ok()?))
                    }
                        // swap instruction has optional targetOrders account
                        fn account_match_without_target_orders(
                            idx: usize,
                            account: &Pubkey,
                        ) -> Option<(String, serde_json::Value)> {
                            let account_name = match idx {
                                0 => "tokenProgram",
                                1 => "amm",
                                2 => "ammAuthority",
                                3 => "ammOpenOrders",
                                4 => "poolCoinTokenAccount",
                                5 => "poolPcTokenAccount",
                                6 => "dexProgram",
                                7 => "dex",
                                8 => "dexBids",
                                9 => "dexAsks",
                                10 => "dexEventQueue",
                                11 => "dexCoinTokenAccount",
                                12 => "dexPcTokenAccount",
                                13 => "dexAuthority",
                                14 => "userInputTokenAccount",
                                15 => "userOutputTokenAccount",
                                16 => "user",
                                _ => return None,
                            }
                            .to_string();
                            Some((account_name, serde_json::to_value(account.to_string()).ok()?))
                        }
                    let accounts_len = self.ix.accounts.len();
                    self.ix.accounts.iter().enumerate().filter_map(|(idx, account)| {
                        if accounts_len == 18 {
                            // target orders account was supplied
                            account_match_with_target_orders(idx, account)
                        } else {
                            account_match_without_target_orders(idx, account)
                        }
                    }).collect()
                },
                name: "swapBaseOut".to_string(),
            }),

            AmmInstruction::SimulateInfo(SimulateInstruction) => Err(anyhow!("unimplemented")),

            AmmInstruction::AdminCancelOrders(AdminCancelOrdersInstruction) => {
                Err(anyhow!("unimplemented"))
            }
            AmmInstruction::CreateConfigAccount => Err(anyhow!("unimplemented")),
            AmmInstruction::UpdateConfigAccount(ConfigArgs) => Err(anyhow!("unimplemented")),
        }
    }
}
