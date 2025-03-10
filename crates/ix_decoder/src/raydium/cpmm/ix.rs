//! Instruction types

#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]

use super::state::{AmmParams, Fees, LastOrderDistance, SimulateParams};
use arrayref::array_ref;
use solana_sdk::{program_error::ProgramError, program_pack::Pack, pubkey::Pubkey};
use std::convert::TryInto;
use std::mem::size_of;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// utc timestamps for pool open
    pub open_time: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InitializeInstruction2 {
    /// nonce used to create valid program address
    pub nonce: u8,
    /// utc timestamps for pool open
    pub open_time: u64,
    /// init token pc amount
    pub init_pc_amount: u64,
    /// init token coin amount
    pub init_coin_amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PreInitializeInstruction {
    /// nonce used to create valid program address
    pub nonce: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MonitorStepInstruction {
    /// max value of plan/new/cancel orders
    pub plan_order_limit: u16,
    pub place_order_limit: u16,
    pub cancel_order_limit: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DepositInstruction {
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WithdrawInstruction {
    /// Pool token amount to transfer. token_a and token_b amount are set by
    /// the current exchange rate and size of the pool
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SetParamsInstruction {
    pub param: u8,
    pub value: Option<u64>,
    pub new_pubkey: Option<Pubkey>,
    pub fees: Option<Fees>,
    pub last_order_distance: Option<LastOrderDistance>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WithdrawSrmInstruction {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstructionBaseIn {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub minimum_amount_out: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapInstructionBaseOut {
    // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
    pub max_amount_in: u64,
    /// Minimum amount of DESTINATION token to output, prevents excessive slippage
    pub amount_out: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimulateInstruction {
    pub param: u8,
    pub swap_base_in_value: Option<SwapInstructionBaseIn>,
    pub swap_base_out_value: Option<SwapInstructionBaseOut>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AdminCancelOrdersInstruction {
    pub limit: u16,
}

/// Update config acccount params
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ConfigArgs {
    pub param: u8,
    pub owner: Option<Pubkey>,
    pub create_pool_fee: Option<u64>,
}

/// Instructions supported by the AmmInfo program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum AmmInstruction {
    ///   Initializes a new AmmInfo.
    ///
    ///   Not supported yet, please use `Initialize2` to new a AMM pool
    #[deprecated(note = "Not supported yet, please use `Initialize2` instead")]
    Initialize(InitializeInstruction),

    ///   Initializes a new AMM pool.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Associated Token program id
    ///   2. `[]` Sys program id
    ///   3. `[]` Rent program id
    ///   4. `[writable]` New AMM Account to create.
    ///   5. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   6. `[writable]` AMM open orders Account
    ///   7. `[writable]` AMM lp mint Account
    ///   8. `[]` AMM coin mint Account
    ///   9. `[]` AMM pc mint Account
    ///   10. `[writable]` AMM coin vault Account. Must be non zero, owned by $authority.
    ///   11. `[writable]` AMM pc vault Account. Must be non zero, owned by $authority.
    ///   12. `[writable]` AMM target orders Account. To store plan orders informations.
    ///   13. `[]` AMM config Account, derived from `find_program_address(&[&&AMM_CONFIG_SEED])`.
    ///   14. `[]` AMM create pool fee destination Account
    ///   15. `[]` Market program id
    ///   16. `[writable]` Market Account. Market program is the owner.
    ///   17. `[writable, signer]` User wallet Account
    ///   18. `[]` User token coin Account
    ///   19. '[]` User token pc Account
    ///   20. `[writable]` User destination lp token ATA Account
    Initialize2(InitializeInstruction2),

    ///   MonitorStep. To monitor place Amm order state machine turn around step by step.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Rent program id
    ///   2. `[]` Sys Clock id
    ///   3. `[writable]` AMM Account
    ///   4. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   5. `[writable]` AMM open orders Account
    ///   6. `[writable]` AMM target orders Account. To store plan orders infomations.
    ///   7. `[writable]` AMM coin vault Account. Must be non zero, owned by $authority.
    ///   8. `[writable]` AMM pc vault Account. Must be non zero, owned by $authority.
    ///   9. `[]` Market program id
    ///   10. `[writable]` Market Account. Market program is the owner.
    ///   11. `[writable]` Market coin vault Account
    ///   12. `[writable]` Market pc vault Account
    ///   13. '[]` Market vault signer Account
    ///   14. '[writable]` Market request queue Account
    ///   15. `[writable]` Market event queue Account
    ///   16. `[writable]` Market bids Account
    ///   17. `[writable]` Market asks Account
    ///   18. `[writable]` (optional) the (M)SRM account used for fee discounts
    ///   19. `[writable]` (optional) the referrer pc account used for settle back referrer
    MonitorStep(MonitorStepInstruction),

    ///   Deposit some tokens into the pool.  The output is a "pool" token representing ownership
    ///   into the pool. Inputs are converted to the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[]` AMM open_orders Account
    ///   4. `[writable]` AMM target orders Account. To store plan orders infomations.
    ///   5. `[writable]` AMM lp mint Account. Owned by $authority.
    ///   6. `[writable]` AMM coin vault $authority can transfer amount,
    ///   7. `[writable]` AMM pc vault $authority can transfer amount,
    ///   8. `[]` Market Account. Market program is the owner.
    ///   9. `[writable]` User coin token Account to deposit into.
    ///   10. `[writable]` User pc token Account to deposit into.
    ///   11. `[writable]` User lp token. To deposit the generated tokens, user is the owner.
    ///   12. '[signer]` User wallet Account
    ///   13. `[]` Market event queue Account.
    Deposit(DepositInstruction),

    ///   Withdraw the vault tokens from the pool at the current ratio.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` AMM target orders Account
    ///   5. `[writable]` AMM lp mint Account. Owned by $authority.
    ///   6. `[writable]` AMM coin vault Account to withdraw FROM,
    ///   7. `[writable]` AMM pc vault Account to withdraw FROM,
    ///   8. `[]` Market program id
    ///   9. `[writable]` Market Account. Market program is the owner.
    ///   10. `[writable]` Market coin vault Account
    ///   11. `[writable]` Market pc vault Account
    ///   12. '[]` Market vault signer Account
    ///   13. `[writable]` User lp token Account.
    ///   14. `[writable]` User token coin Account. user Account to credit.
    ///   15. `[writable]` User token pc Account. user Account to credit.
    ///   16. `[signer]` User wallet Account
    ///   17. `[writable]` Market event queue Account
    ///   18. `[writable]` Market bids Account
    ///   19. `[writable]` Market asks Account
    Withdraw(WithdrawInstruction),

    ///   Migrate the associated market from Serum to OpenBook.
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` Sys program id
    ///   2. `[]` Rent program id
    ///   3. `[writable]` AMM Account
    ///   4. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   5. `[writable]` AMM open orders Account
    ///   6. `[writable]` AMM coin vault account owned by $authority,
    ///   7. `[writable]` AMM pc vault account owned by $authority,
    ///   8. `[writable]` AMM target orders Account
    ///   9. `[]` Market program id
    ///   10. `[writable]` Market Account. Market program is the owner.
    ///   11. `[writable]` Market bids Account
    ///   12. `[writable]` Market asks Account
    ///   13. `[writable]` Market event queue Account
    ///   14. `[writable]` Market coin vault Account
    ///   15. `[writable]` Market pc vault Account
    ///   16. '[]` Market vault signer Account
    ///   17. '[writable]` AMM new open orders Account
    ///   18. '[]` mew Market program id
    ///   19. '[]` new Market market Account
    ///   20. '[]` Admin Account
    MigrateToOpenBook,

    ///   Set AMM params
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account.
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` AMM target orders Account
    ///   5. `[writable]` AMM coin vault account owned by $authority,
    ///   6. `[writable]` AMM pc vault account owned by $authority,
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market coin vault Account
    ///   10. `[writable]` Market pc vault Account
    ///   11. '[]` Market vault signer Account
    ///   12. `[writable]` Market event queue Account
    ///   13. `[writable]` Market bids Account
    ///   14. `[writable]` Market asks Account
    ///   15. `[signer]` Admin Account
    ///   16. `[]` (optional) New AMM open orders Account to replace old AMM open orders Account
    SetParams(SetParamsInstruction),

    ///   Withdraw Pnl from pool by protocol
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` AMM config Account, derived from `find_program_address(&[&&AMM_CONFIG_SEED])`.
    ///   3. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   4. `[writable]` AMM open orders Account
    ///   5. `[writable]` AMM coin vault account to withdraw FROM,
    ///   6. `[writable]` AMM pc vault account to withdraw FROM,
    ///   7. `[writable]` User coin token Account to withdraw to
    ///   8. `[writable]` User pc token Account to withdraw to
    ///   9. `[signer]` User wallet account
    ///   10. `[writable]` AMM target orders Account
    ///   11. `[]` Market program id
    ///   12. `[writable]` Market Account. Market program is the owner.
    ///   13. `[writable]` Market event queue Account
    ///   14. `[writable]` Market coin vault Account
    ///   15. `[writable]` Market pc vault Account
    ///   16. '[]` Market vault signer Account
    ///   17. `[]` (optional) the referrer pc account used for settle back referrer
    WithdrawPnl,

    ///   Withdraw (M)SRM from the (M)SRM Account used for fee discounts by admin
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[]` AMM Account.
    ///   2. `[signer]` Admin wallet Account
    ///   3. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   4. `[writable]` the (M)SRM Account withdraw from
    ///   5. `[writable]` the (M)SRM Account withdraw to
    WithdrawSrm(WithdrawSrmInstruction),

    /// Swap coin or pc from pool, base amount_in with a slippage of minimum_amount_out
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` (optional)AMM target orders Account, no longer used in the contract, recommended no need to add this Account.
    ///   5. `[writable]` AMM coin vault Account to swap FROM or To.
    ///   6. `[writable]` AMM pc vault Account to swap FROM or To.
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market bids Account
    ///   10. `[writable]` Market asks Account
    ///   11. `[writable]` Market event queue Account
    ///   12. `[writable]` Market coin vault Account
    ///   13. `[writable]` Market pc vault Account
    ///   14. '[]` Market vault signer Account
    ///   15. `[writable]` User source token Account.
    ///   16. `[writable]` User destination token Account.
    ///   17. `[signer]` User wallet Account
    SwapBaseIn(SwapInstructionBaseIn),

    ///   Continue Initializes a new Amm pool because of compute units limit.
    ///   Not supported yet, please use `Initialize2` to new a Amm pool
    #[deprecated(note = "Not supported yet, please use `Initialize2` instead")]
    PreInitialize(PreInitializeInstruction),

    /// Swap coin or pc from pool, base amount_out with a slippage of max_amount_in
    ///
    ///   0. `[]` Spl Token program id
    ///   1. `[writable]` AMM Account
    ///   2. `[]` $authority derived from `create_program_address(&[AUTHORITY_AMM, &[nonce]])`.
    ///   3. `[writable]` AMM open orders Account
    ///   4. `[writable]` (optional)AMM target orders Account, no longer used in the contract, recommended no need to add this Account.
    ///   5. `[writable]` AMM coin vault Account to swap FROM or To.
    ///   6. `[writable]` AMM pc vault Account to swap FROM or To.
    ///   7. `[]` Market program id
    ///   8. `[writable]` Market Account. Market program is the owner.
    ///   9. `[writable]` Market bids Account
    ///   10. `[writable]` Market asks Account
    ///   11. `[writable]` Market event queue Account
    ///   12. `[writable]` Market coin vault Account
    ///   13. `[writable]` Market pc vault Account
    ///   14. '[]` Market vault signer Account
    ///   15. `[writable]` User source token Account.
    ///   16. `[writable]` User destination token Account.
    ///   17. `[signer]` User wallet Account
    SwapBaseOut(SwapInstructionBaseOut),

    SimulateInfo(SimulateInstruction),

    AdminCancelOrders(AdminCancelOrdersInstruction),

    /// Create amm config account by admin
    CreateConfigAccount,

    /// Update amm config account by admin
    UpdateConfigAccount(ConfigArgs),
}

impl AmmInstruction {
    /// Unpacks a byte buffer into a [AmmInstruction](enum.AmmInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let (nonce, rest) = Self::unpack_u8(rest)?;
                let (open_time, _reset) = Self::unpack_u64(rest)?;
                Self::Initialize(InitializeInstruction { nonce, open_time })
            }
            1 => {
                let (nonce, rest) = Self::unpack_u8(rest)?;
                let (open_time, rest) = Self::unpack_u64(rest)?;
                let (init_pc_amount, rest) = Self::unpack_u64(rest)?;
                let (init_coin_amount, _reset) = Self::unpack_u64(rest)?;
                Self::Initialize2(InitializeInstruction2 {
                    nonce,
                    open_time,
                    init_pc_amount,
                    init_coin_amount,
                })
            }
            2 => {
                let (plan_order_limit, rest) = Self::unpack_u16(rest)?;
                let (place_order_limit, rest) = Self::unpack_u16(rest)?;
                let (cancel_order_limit, _rest) = Self::unpack_u16(rest)?;
                Self::MonitorStep(MonitorStepInstruction {
                    plan_order_limit,
                    place_order_limit,
                    cancel_order_limit,
                })
            }
            3 => {
                let (max_coin_amount, rest) = Self::unpack_u64(rest)?;
                let (max_pc_amount, rest) = Self::unpack_u64(rest)?;
                let (base_side, _rest) = Self::unpack_u64(rest)?;
                Self::Deposit(DepositInstruction {
                    max_coin_amount,
                    max_pc_amount,
                    base_side,
                })
            }
            4 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::Withdraw(WithdrawInstruction { amount })
            }
            5 => Self::MigrateToOpenBook,
            6 => {
                let (param, rest) = Self::unpack_u8(rest)?;
                match AmmParams::from_u64(param as u64) {
                    AmmParams::AmmOwner => {
                        if rest.len() >= 32 {
                            let new_pubkey = array_ref![rest, 0, 32];
                            Self::SetParams(SetParamsInstruction {
                                param,
                                value: None,
                                new_pubkey: Some(Pubkey::new_from_array(*new_pubkey)),
                                fees: None,
                                last_order_distance: None,
                            })
                        } else {
                            return Err(ProgramError::InvalidInstructionData.into());
                        }
                    }
                    AmmParams::Fees => {
                        if rest.len() >= Fees::LEN {
                            let (fees, _rest) = rest.split_at(Fees::LEN);
                            let fees = Fees::unpack_from_slice(fees)?;
                            Self::SetParams(SetParamsInstruction {
                                param,
                                value: None,
                                new_pubkey: None,
                                fees: Some(fees),
                                last_order_distance: None,
                            })
                        } else {
                            return Err(ProgramError::InvalidInstructionData.into());
                        }
                    }
                    AmmParams::LastOrderDistance => {
                        if rest.len() >= 16 {
                            let (last_order_numerator, rest) = Self::unpack_u64(rest)?;
                            let (last_order_denominator, _rest) = Self::unpack_u64(rest)?;
                            Self::SetParams(SetParamsInstruction {
                                param,
                                value: None,
                                new_pubkey: None,
                                fees: None,
                                last_order_distance: Some(LastOrderDistance {
                                    last_order_numerator,
                                    last_order_denominator,
                                }),
                            })
                        } else {
                            return Err(ProgramError::InvalidInstructionData.into());
                        }
                    }
                    _ => {
                        if rest.len() >= 8 {
                            let (value, _rest) = Self::unpack_u64(rest)?;
                            Self::SetParams(SetParamsInstruction {
                                param,
                                value: Some(value),
                                new_pubkey: None,
                                fees: None,
                                last_order_distance: None,
                            })
                        } else {
                            return Err(ProgramError::InvalidInstructionData.into());
                        }
                    }
                }
            }
            7 => Self::WithdrawPnl,
            8 => {
                let (amount, _rest) = Self::unpack_u64(rest)?;
                Self::WithdrawSrm(WithdrawSrmInstruction { amount })
            }
            9 => {
                let (amount_in, rest) = Self::unpack_u64(rest)?;
                let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::SwapBaseIn(SwapInstructionBaseIn {
                    amount_in,
                    minimum_amount_out,
                })
            }
            10 => {
                let (nonce, _rest) = Self::unpack_u8(rest)?;
                Self::PreInitialize(PreInitializeInstruction { nonce })
            }
            11 => {
                let (max_amount_in, rest) = Self::unpack_u64(rest)?;
                let (amount_out, _rest) = Self::unpack_u64(rest)?;
                Self::SwapBaseOut(SwapInstructionBaseOut {
                    max_amount_in,
                    amount_out,
                })
            }
            12 => {
                let (param, rest) = Self::unpack_u8(rest)?;
                match SimulateParams::from_u64(param as u64) {
                    SimulateParams::PoolInfo | SimulateParams::RunCrankInfo => {
                        Self::SimulateInfo(SimulateInstruction {
                            param,
                            swap_base_in_value: None,
                            swap_base_out_value: None,
                        })
                    }
                    SimulateParams::SwapBaseInInfo => {
                        let (amount_in, rest) = Self::unpack_u64(rest)?;
                        let (minimum_amount_out, _rest) = Self::unpack_u64(rest)?;
                        let swap_base_in = Some(SwapInstructionBaseIn {
                            amount_in,
                            minimum_amount_out,
                        });
                        Self::SimulateInfo(SimulateInstruction {
                            param,
                            swap_base_in_value: swap_base_in,
                            swap_base_out_value: None,
                        })
                    }
                    SimulateParams::SwapBaseOutInfo => {
                        let (max_amount_in, rest) = Self::unpack_u64(rest)?;
                        let (amount_out, _rest) = Self::unpack_u64(rest)?;
                        let swap_base_out = Some(SwapInstructionBaseOut {
                            max_amount_in,
                            amount_out,
                        });
                        Self::SimulateInfo(SimulateInstruction {
                            param,
                            swap_base_in_value: None,
                            swap_base_out_value: swap_base_out,
                        })
                    }
                }
            }
            13 => {
                let (limit, _rest) = Self::unpack_u16(rest)?;
                Self::AdminCancelOrders(AdminCancelOrdersInstruction { limit })
            }
            14 => Self::CreateConfigAccount,
            15 => {
                let (param, rest) = Self::unpack_u8(rest)?;
                match param {
                    0 | 1 => {
                        let pubkey = array_ref![rest, 0, 32];
                        Self::UpdateConfigAccount(ConfigArgs {
                            param,
                            owner: Some(Pubkey::new_from_array(*pubkey)),
                            create_pool_fee: None,
                        })
                    }
                    2 => {
                        let (create_pool_fee, _rest) = Self::unpack_u64(rest)?;
                        Self::UpdateConfigAccount(ConfigArgs {
                            param,
                            owner: None,
                            create_pool_fee: Some(create_pool_fee),
                        })
                    }
                    _ => {
                        return Err(ProgramError::InvalidInstructionData.into());
                    }
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData.into()),
        })
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.len() >= 1 {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

    fn unpack_u16(input: &[u8]) -> Result<(u16, &[u8]), ProgramError> {
        if input.len() >= 2 {
            let (amount, rest) = input.split_at(2);
            let amount = amount
                .get(..2)
                .and_then(|slice| slice.try_into().ok())
                .map(u16::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((amount, rest))
        } else {
            Err(ProgramError::InvalidInstructionData.into())
        }
    }

    /// Packs a [AmmInstruction](enum.AmmInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Result<Vec<u8>, ProgramError> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match &*self {
            Self::Initialize(InitializeInstruction { nonce, open_time }) => {
                buf.push(0);
                buf.push(*nonce);
                buf.extend_from_slice(&open_time.to_le_bytes());
            }
            Self::Initialize2(InitializeInstruction2 {
                nonce,
                open_time,
                init_pc_amount,
                init_coin_amount,
            }) => {
                buf.push(1);
                buf.push(*nonce);
                buf.extend_from_slice(&open_time.to_le_bytes());
                buf.extend_from_slice(&init_pc_amount.to_le_bytes());
                buf.extend_from_slice(&init_coin_amount.to_le_bytes());
            }
            Self::MonitorStep(MonitorStepInstruction {
                plan_order_limit,
                place_order_limit,
                cancel_order_limit,
            }) => {
                buf.push(2);
                buf.extend_from_slice(&plan_order_limit.to_le_bytes());
                buf.extend_from_slice(&place_order_limit.to_le_bytes());
                buf.extend_from_slice(&cancel_order_limit.to_le_bytes());
            }
            Self::Deposit(DepositInstruction {
                max_coin_amount,
                max_pc_amount,
                base_side,
            }) => {
                buf.push(3);
                buf.extend_from_slice(&max_coin_amount.to_le_bytes());
                buf.extend_from_slice(&max_pc_amount.to_le_bytes());
                buf.extend_from_slice(&base_side.to_le_bytes());
            }
            Self::Withdraw(WithdrawInstruction { amount }) => {
                buf.push(4);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::MigrateToOpenBook => {
                buf.push(5);
            }
            Self::SetParams(SetParamsInstruction {
                param,
                value,
                new_pubkey,
                fees,
                last_order_distance,
            }) => {
                buf.push(6);
                buf.push(*param);
                match AmmParams::from_u64(*param as u64) {
                    AmmParams::AmmOwner => {
                        let new_pubkey = match new_pubkey {
                            Some(a) => a,
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&new_pubkey.to_bytes());
                    }
                    AmmParams::Fees => {
                        let fees = match fees {
                            Some(a) => a,
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        let mut fees_slice = [0u8; Fees::LEN];
                        Pack::pack_into_slice(fees, &mut fees_slice[..]);
                        buf.extend_from_slice(&fees_slice);
                    }
                    AmmParams::LastOrderDistance => {
                        let distance = match last_order_distance {
                            Some(a) => a,
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&distance.last_order_numerator.to_le_bytes());
                        buf.extend_from_slice(&distance.last_order_denominator.to_le_bytes());
                    }
                    _ => {
                        let value = match value {
                            Some(a) => a,
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&value.to_le_bytes());
                    }
                }
            }
            Self::WithdrawPnl => {
                buf.push(7);
            }
            Self::WithdrawSrm(WithdrawSrmInstruction { amount }) => {
                buf.push(8);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::SwapBaseIn(SwapInstructionBaseIn {
                amount_in,
                minimum_amount_out,
            }) => {
                buf.push(9);
                buf.extend_from_slice(&amount_in.to_le_bytes());
                buf.extend_from_slice(&minimum_amount_out.to_le_bytes());
            }
            Self::PreInitialize(PreInitializeInstruction { nonce }) => {
                buf.push(10);
                buf.push(*nonce);
            }
            Self::SwapBaseOut(SwapInstructionBaseOut {
                max_amount_in,
                amount_out,
            }) => {
                buf.push(11);
                buf.extend_from_slice(&max_amount_in.to_le_bytes());
                buf.extend_from_slice(&amount_out.to_le_bytes());
            }
            Self::SimulateInfo(SimulateInstruction {
                param,
                swap_base_in_value,
                swap_base_out_value,
            }) => {
                buf.push(12);
                buf.push(*param);
                match SimulateParams::from_u64(*param as u64) {
                    SimulateParams::PoolInfo | SimulateParams::RunCrankInfo => {}
                    SimulateParams::SwapBaseInInfo => {
                        let swap_base_in = match swap_base_in_value {
                            Some(a) => a,
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&swap_base_in.amount_in.to_le_bytes());
                        buf.extend_from_slice(&swap_base_in.minimum_amount_out.to_le_bytes());
                    }
                    SimulateParams::SwapBaseOutInfo => {
                        let swap_base_out = match swap_base_out_value {
                            Some(a) => a,
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&swap_base_out.max_amount_in.to_le_bytes());
                        buf.extend_from_slice(&swap_base_out.amount_out.to_le_bytes());
                    }
                }
            }
            Self::AdminCancelOrders(AdminCancelOrdersInstruction { limit }) => {
                buf.push(13);
                buf.extend_from_slice(&limit.to_le_bytes());
            }
            Self::CreateConfigAccount => {
                buf.push(14);
            }
            Self::UpdateConfigAccount(ConfigArgs {
                param,
                owner,
                create_pool_fee,
            }) => {
                buf.push(15);
                buf.push(*param);
                match param {
                    0 | 1 => {
                        let owner = match owner {
                            Some(owner) => {
                                if *owner == Pubkey::default() {
                                    return Err(ProgramError::InvalidInstructionData.into());
                                } else {
                                    owner
                                }
                            }
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&owner.to_bytes());
                    }
                    2 => {
                        let create_pool_fee = match create_pool_fee {
                            Some(create_pool_fee) => create_pool_fee,
                            None => return Err(ProgramError::InvalidInstructionData.into()),
                        };
                        buf.extend_from_slice(&create_pool_fee.to_le_bytes());
                    }
                    _ => return Err(ProgramError::InvalidInstructionData.into()),
                }
            }
        }
        Ok(buf)
    }
}
