use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_sdk::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LastOrderDistance {
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,
}

#[derive(Copy, Clone)]
#[repr(u64)]
pub enum SimulateParams {
    PoolInfo = 0u64,
    SwapBaseInInfo = 1u64,
    SwapBaseOutInfo = 2u64,
    RunCrankInfo = 3u64,
}

#[derive(Copy, Clone)]
#[repr(u64)]
pub enum AmmParams {
    Status = 0u64,
    State = 1u64,
    OrderNum = 2u64,
    Depth = 3u64,
    AmountWave = 4u64,
    MinPriceMultiplier = 5u64,
    MaxPriceMultiplier = 6u64,
    MinSize = 7u64,
    VolMaxCutRatio = 8u64,
    Fees = 9u64,
    AmmOwner = 10u64,
    SetOpenTime = 11u64,
    LastOrderDistance = 12u64,
    InitOrderDepth = 13u64,
    SetSwitchTime = 14u64,
    ClearOpenTime = 15u64,
    Seperate = 16u64,
    UpdateOpenOrder = 17u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fees {
    /// numerator of the min_separate
    pub min_separate_numerator: u64,
    /// denominator of the min_separate
    pub min_separate_denominator: u64,

    /// numerator of the fee
    pub trade_fee_numerator: u64,
    /// denominator of the fee
    /// and 'trade_fee_denominator' must be equal to 'min_separate_denominator'
    pub trade_fee_denominator: u64,

    /// numerator of the pnl
    pub pnl_numerator: u64,
    /// denominator of the pnl
    pub pnl_denominator: u64,

    /// numerator of the swap_fee
    pub swap_fee_numerator: u64,
    /// denominator of the swap_fee
    pub swap_fee_denominator: u64,
}
impl SimulateParams {
    pub fn from_u64(flag: u64) -> Self {
        match flag {
            0u64 => SimulateParams::PoolInfo,
            1u64 => SimulateParams::SwapBaseInInfo,
            2u64 => SimulateParams::SwapBaseOutInfo,
            3u64 => SimulateParams::RunCrankInfo,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            SimulateParams::PoolInfo => 0u64,
            SimulateParams::SwapBaseInInfo => 1u64,
            SimulateParams::SwapBaseOutInfo => 2u64,
            SimulateParams::RunCrankInfo => 3u64,
        }
    }
}

/// IsInitialized is required to use `Pack::pack` and `Pack::unpack`
impl IsInitialized for Fees {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for Fees {}
impl Pack for Fees {
    const LEN: usize = 64;
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 64];
        let (
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
        ) = mut_array_refs![output, 8, 8, 8, 8, 8, 8, 8, 8];
        *min_separate_numerator = self.min_separate_numerator.to_le_bytes();
        *min_separate_denominator = self.min_separate_denominator.to_le_bytes();
        *trade_fee_numerator = self.trade_fee_numerator.to_le_bytes();
        *trade_fee_denominator = self.trade_fee_denominator.to_le_bytes();
        *pnl_numerator = self.pnl_numerator.to_le_bytes();
        *pnl_denominator = self.pnl_denominator.to_le_bytes();
        *swap_fee_numerator = self.swap_fee_numerator.to_le_bytes();
        *swap_fee_denominator = self.swap_fee_denominator.to_le_bytes();
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Fees, ProgramError> {
        let input = array_ref![input, 0, 64];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
        ) = array_refs![input, 8, 8, 8, 8, 8, 8, 8, 8];
        Ok(Self {
            min_separate_numerator: u64::from_le_bytes(*min_separate_numerator),
            min_separate_denominator: u64::from_le_bytes(*min_separate_denominator),
            trade_fee_numerator: u64::from_le_bytes(*trade_fee_numerator),
            trade_fee_denominator: u64::from_le_bytes(*trade_fee_denominator),
            pnl_numerator: u64::from_le_bytes(*pnl_numerator),
            pnl_denominator: u64::from_le_bytes(*pnl_denominator),
            swap_fee_numerator: u64::from_le_bytes(*swap_fee_numerator),
            swap_fee_denominator: u64::from_le_bytes(*swap_fee_denominator),
        })
    }
}

impl AmmParams {
    pub fn from_u64(state: u64) -> Self {
        match state {
            0u64 => AmmParams::Status,
            1u64 => AmmParams::State,
            2u64 => AmmParams::OrderNum,
            3u64 => AmmParams::Depth,
            4u64 => AmmParams::AmountWave,
            5u64 => AmmParams::MinPriceMultiplier,
            6u64 => AmmParams::MaxPriceMultiplier,
            7u64 => AmmParams::MinSize,
            8u64 => AmmParams::VolMaxCutRatio,
            9u64 => AmmParams::Fees,
            10u64 => AmmParams::AmmOwner,
            11u64 => AmmParams::SetOpenTime,
            12u64 => AmmParams::LastOrderDistance,
            13u64 => AmmParams::InitOrderDepth,
            14u64 => AmmParams::SetSwitchTime,
            15u64 => AmmParams::ClearOpenTime,
            16u64 => AmmParams::Seperate,
            17u64 => AmmParams::UpdateOpenOrder,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmParams::Status => 0u64,
            AmmParams::State => 1u64,
            AmmParams::OrderNum => 2u64,
            AmmParams::Depth => 3u64,
            AmmParams::AmountWave => 4u64,
            AmmParams::MinPriceMultiplier => 5u64,
            AmmParams::MaxPriceMultiplier => 6u64,
            AmmParams::MinSize => 7u64,
            AmmParams::VolMaxCutRatio => 8u64,
            AmmParams::Fees => 9u64,
            AmmParams::AmmOwner => 10u64,
            AmmParams::SetOpenTime => 11u64,
            AmmParams::LastOrderDistance => 12u64,
            AmmParams::InitOrderDepth => 13u64,
            AmmParams::SetSwitchTime => 14u64,
            AmmParams::ClearOpenTime => 15u64,
            AmmParams::Seperate => 16u64,
            AmmParams::UpdateOpenOrder => 17u64,
        }
    }
}
