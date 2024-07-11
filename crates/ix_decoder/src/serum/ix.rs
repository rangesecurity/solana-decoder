use super::matching::{OrderType, Side};
use bytemuck::cast;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use arrayref::{array_ref, array_refs};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::num::NonZeroU64;


pub mod srm_token {
    pub const ID: solana_sdk::pubkey::Pubkey = solana_sdk::pubkey!("SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt");
}

pub mod msrm_token {
    pub const ID: solana_sdk::pubkey::Pubkey = solana_sdk::pubkey!("MSRMcoVyrFxnSgo5uXwone5SKcGhT1KEJMFEkMEWf9L");
}

pub mod disable_authority {
    pub const ID: solana_sdk::pubkey::Pubkey = solana_sdk::pubkey!("5ZVJgwWxMsqXxRMYHXqMwH2hd4myX5Ef4Au2iUsuNQ7V");
}

pub mod fee_sweeper {
    pub const ID: solana_sdk::pubkey::Pubkey = solana_sdk::pubkey!("DeqYsmBd9BnrbgUwQjVH4sQWK71dEgE6eoZFw3Rp4ftE");
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct InitializeMarketInstruction {
    // In the matching engine, all prices and balances are integers.
    // This only works if the smallest representable quantity of the coin
    // is at least a few orders of magnitude larger than the smallest representable
    // quantity of the price currency. The internal representation also relies on
    // on the assumption that every order will have a (quantity x price) value that
    // fits into a u64.
    //
    // If these assumptions are problematic, rejigger the lot sizes.
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub fee_rate_bps: u16,
    pub vault_signer_nonce: u64,
    pub pc_dust_threshold: u64,
}

#[derive(
    PartialEq, Eq, Copy, Clone, Debug, TryFromPrimitive, IntoPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum SelfTradeBehavior {
    DecrementTake = 0,
    CancelProvide = 1,
    AbortTransaction = 2,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct SendTakeInstruction {
    pub side: Side,

    pub limit_price: NonZeroU64,

    pub max_coin_qty: NonZeroU64,

    pub max_native_pc_qty_including_fees: NonZeroU64,

    pub min_coin_qty: u64,
    pub min_native_pc_qty: u64,

    pub limit: u16,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderInstructionV3 {
    pub side: Side,


    pub limit_price: NonZeroU64,

 
    pub max_coin_qty: NonZeroU64,

    pub max_native_pc_qty_including_fees: NonZeroU64,

    pub self_trade_behavior: SelfTradeBehavior,

    pub order_type: OrderType,
    pub client_order_id: u64,
    pub limit: u16,
    pub max_ts: i64,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderInstructionV2 {
    pub side: Side,

    pub limit_price: NonZeroU64,

    pub max_qty: NonZeroU64,
    pub order_type: OrderType,
    pub client_id: u64,
    pub self_trade_behavior: SelfTradeBehavior,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderInstructionV1 {
    pub side: Side,

    pub limit_price: NonZeroU64,

    pub max_qty: NonZeroU64,
    pub order_type: OrderType,
    pub client_id: u64,
}

impl NewOrderInstructionV1 {
    pub fn add_self_trade_behavior(
        self,
        self_trade_behavior: SelfTradeBehavior,
    ) -> NewOrderInstructionV2 {
        let NewOrderInstructionV1 {
            side,
            limit_price,
            max_qty,
            order_type,
            client_id,
        } = self;
        NewOrderInstructionV2 {
            side,
            limit_price,
            max_qty,
            order_type,
            client_id,
            self_trade_behavior,
        }
    }
}

impl SendTakeInstruction {
    fn unpack(data: &[u8; 46]) -> Option<Self> {
        let (
            &side_arr,
            &price_arr,
            &max_coin_qty_arr,
            &max_native_pc_qty_arr,
            &min_coin_qty_arr,
            &min_native_pc_qty_arr,
            &limit_arr,
        ) = array_refs![data, 4, 8, 8, 8, 8, 8, 2];

        let side = Side::try_from_primitive(u32::from_le_bytes(side_arr).try_into().ok()?).ok()?;
        let limit_price = NonZeroU64::new(u64::from_le_bytes(price_arr))?;
        let max_coin_qty = NonZeroU64::new(u64::from_le_bytes(max_coin_qty_arr))?;
        let max_native_pc_qty_including_fees =
            NonZeroU64::new(u64::from_le_bytes(max_native_pc_qty_arr))?;
        let min_coin_qty = u64::from_le_bytes(min_coin_qty_arr);
        let min_native_pc_qty = u64::from_le_bytes(min_native_pc_qty_arr);
        let limit = u16::from_le_bytes(limit_arr);

        Some(SendTakeInstruction {
            side,
            limit_price,
            max_coin_qty,
            max_native_pc_qty_including_fees,
            min_coin_qty,
            min_native_pc_qty,
            limit,
        })
    }
}

impl NewOrderInstructionV3 {
    fn unpack(data: &[u8; 54]) -> Option<Self> {
        let (
            &side_arr,
            &price_arr,
            &max_coin_qty_arr,
            &max_native_pc_qty_arr,
            &self_trade_behavior_arr,
            &otype_arr,
            &client_order_id_bytes,
            &limit_arr,
            &max_ts,
        ) = array_refs![data, 4, 8, 8, 8, 4, 4, 8, 2, 8];

        let side = Side::try_from_primitive(u32::from_le_bytes(side_arr).try_into().ok()?).ok()?;
        let limit_price = NonZeroU64::new(u64::from_le_bytes(price_arr))?;
        let max_coin_qty = NonZeroU64::new(u64::from_le_bytes(max_coin_qty_arr))?;
        let max_native_pc_qty_including_fees =
            NonZeroU64::new(u64::from_le_bytes(max_native_pc_qty_arr))?;
        let self_trade_behavior = SelfTradeBehavior::try_from_primitive(
            u32::from_le_bytes(self_trade_behavior_arr)
                .try_into()
                .ok()?,
        )
        .ok()?;
        let order_type =
            OrderType::try_from_primitive(u32::from_le_bytes(otype_arr).try_into().ok()?).ok()?;
        let client_order_id = u64::from_le_bytes(client_order_id_bytes);
        let limit = u16::from_le_bytes(limit_arr);
        let max_ts = i64::from_le_bytes(max_ts);

        Some(NewOrderInstructionV3 {
            side,
            limit_price,
            max_coin_qty,
            max_native_pc_qty_including_fees,
            self_trade_behavior,
            order_type,
            client_order_id,
            limit,
            max_ts,
        })
    }
}

impl NewOrderInstructionV1 {
    fn unpack(data: &[u8; 32]) -> Option<Self> {
        let (&side_arr, &price_arr, &max_qty_arr, &otype_arr, &client_id_bytes) =
            array_refs![data, 4, 8, 8, 4, 8];
        let client_id = u64::from_le_bytes(client_id_bytes);
        let side = match u32::from_le_bytes(side_arr) {
            0 => Side::Bid,
            1 => Side::Ask,
            _ => return None,
        };
        let limit_price = NonZeroU64::new(u64::from_le_bytes(price_arr))?;
        let max_qty = NonZeroU64::new(u64::from_le_bytes(max_qty_arr))?;
        let order_type = match u32::from_le_bytes(otype_arr) {
            0 => OrderType::Limit,
            1 => OrderType::ImmediateOrCancel,
            2 => OrderType::PostOnly,
            _ => return None,
        };
        Some(NewOrderInstructionV1 {
            side,
            limit_price,
            max_qty,
            order_type,
            client_id,
        })
    }
}
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
pub struct CancelOrderInstructionV2 {
    pub side: Side,
    pub order_id: u128,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
pub struct CancelOrderInstruction {
    pub side: Side,
    pub order_id: u128,
    pub owner: [u64; 4], // Unused
    pub owner_slot: u8,
}

impl CancelOrderInstructionV2 {
    fn unpack(data: &[u8; 20]) -> Option<Self> {
        let (&side_arr, &oid_arr) = array_refs![data, 4, 16];
        let side = Side::try_from_primitive(u32::from_le_bytes(side_arr).try_into().ok()?).ok()?;
        let order_id = u128::from_le_bytes(oid_arr);
        Some(CancelOrderInstructionV2 { side, order_id })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
pub enum MarketInstruction {
    /// 0. `[writable]` the market to initialize
    /// 1. `[writable]` zeroed out request queue
    /// 2. `[writable]` zeroed out event queue
    /// 3. `[writable]` zeroed out bids
    /// 4. `[writable]` zeroed out asks
    /// 5. `[writable]` spl-token account for the coin currency
    /// 6. `[writable]` spl-token account for the price currency
    /// 7. `[]` coin currency Mint
    /// 8. `[]` price currency Mint
    /// 9. `[]` the rent sysvar
    /// 10. `[]` open orders market authority (optional)
    /// 11. `[]` prune authority (optional, requires open orders market authority)
    /// 12. `[]` crank authority (optional, requires prune authority)
    InitializeMarket(InitializeMarketInstruction),
    /// 0. `[writable]` the market
    /// 1. `[writable]` the OpenOrders account to use
    /// 2. `[writable]` the request queue
    /// 3. `[writable]` the (coin or price currency) account paying for the order
    /// 4. `[signer]` owner of the OpenOrders account
    /// 5. `[writable]` coin vault
    /// 6. `[writable]` pc vault
    /// 7. `[]` spl token program
    /// 8. `[]` the rent sysvar
    /// 9. `[]` (optional) the (M)SRM account used for fee discounts
    NewOrder(NewOrderInstructionV1),
    /// 0. `[writable]` market
    /// 1. `[writable]` req_q
    /// 2. `[writable]` event_q
    /// 3. `[writable]` bids
    /// 4. `[writable]` asks
    MatchOrders(u16),
    /// ... `[writable]` OpenOrders
    /// accounts.len() - 4 `[writable]` market
    /// accounts.len() - 3 `[writable]` event queue
    /// accounts.len() - 2 `[]`
    /// accounts.len() - 1 `[]`
    ConsumeEvents(u16),
    /// 0. `[]` market
    /// 1. `[writable]` OpenOrders
    /// 2. `[writable]` the request queue
    /// 3. `[signer]` the OpenOrders owner
    CancelOrder(CancelOrderInstruction),
    /// 0. `[writable]` market
    /// 1. `[writable]` OpenOrders
    /// 2. `[signer]` the OpenOrders owner
    /// 3. `[writable]` coin vault
    /// 4. `[writable]` pc vault
    /// 5. `[writable]` coin wallet
    /// 6. `[writable]` pc wallet
    /// 7. `[]` vault signer
    /// 8. `[]` spl token program
    /// 9. `[writable]` (optional) referrer pc wallet
    SettleFunds,
    /// 0. `[]` market
    /// 1. `[writable]` OpenOrders
    /// 2. `[writable]` the request queue
    /// 3. `[signer]` the OpenOrders owner
    CancelOrderByClientId(u64),
    /// 0. `[writable]` market
    /// 1. `[signer]` disable authority
    DisableMarket,
    /// 0. `[writable]` market
    /// 1. `[writable]` pc vault
    /// 2. `[signer]` fee sweeping authority
    /// 3. `[writable]` fee receivable account
    /// 4. `[]` vault signer
    /// 5. `[]` spl token program
    SweepFees,
    /// 0. `[writable]` the market
    /// 1. `[writable]` the OpenOrders account to use
    /// 2. `[writable]` the request queue
    /// 3. `[writable]` the (coin or price currency) account paying for the order
    /// 4. `[signer]` owner of the OpenOrders account
    /// 5. `[writable]` coin vault
    /// 6. `[writable]` pc vault
    /// 7. `[]` spl token program
    /// 8. `[]` the rent sysvar
    /// 9. `[]` (optional) the (M)SRM account used for fee discounts
    NewOrderV2(NewOrderInstructionV2),
    /// 0. `[writable]` the market
    /// 1. `[writable]` the OpenOrders account to use
    /// 2. `[writable]` the request queue
    /// 3. `[writable]` the event queue
    /// 4. `[writable]` bids
    /// 5. `[writable]` asks
    /// 6. `[writable]` the (coin or price currency) account paying for the order
    /// 7. `[signer]` owner of the OpenOrders account
    /// 8. `[writable]` coin vault
    /// 9. `[writable]` pc vault
    /// 10. `[]` spl token program
    /// 11. `[]` the rent sysvar
    /// 12. `[]` (optional) the (M)SRM account used for fee discounts
    NewOrderV3(NewOrderInstructionV3),
    /// 0. `[writable]` market
    /// 1. `[writable]` bids
    /// 2. `[writable]` asks
    /// 3. `[writable]` OpenOrders
    /// 4. `[signer]` the OpenOrders owner
    /// 5. `[writable]` event_q
    CancelOrderV2(CancelOrderInstructionV2),
    /// 0. `[writable]` market
    /// 1. `[writable]` bids
    /// 2. `[writable]` asks
    /// 3. `[writable]` OpenOrders
    /// 4. `[signer]` the OpenOrders owner
    /// 5. `[writable]` event_q
    CancelOrderByClientIdV2(u64),
    /// 0. `[writable]` market
    /// 1. `[writable]` the request queue
    /// 2. `[writable]` the event queue
    /// 3. `[writable]` bids
    /// 4. `[writable]` asks
    /// 5. `[writable]` the coin currency wallet account
    /// 6. `[writable]` the price currency wallet account
    /// 7. `[]` signer
    /// 8. `[writable]` coin vault
    /// 9. `[writable]` pc vault
    /// 10. `[]` spl token program
    /// 11. `[]` vault signer
    /// 12. `[]` (optional) the (M)SRM account used for fee discounts
    SendTake(SendTakeInstruction),
    /// 0. `[writable]` OpenOrders
    /// 1. `[signer]` the OpenOrders owner
    /// 2. `[writable]` the destination account to send rent exemption SOL to
    /// 3. `[]` market
    CloseOpenOrders,
    /// 0. `[writable]` OpenOrders
    /// 1. `[signer]` the OpenOrders owner
    /// 2. `[]` market
    /// 3. `[]`
    /// 4. `[signer]` open orders market authority (optional).
    InitOpenOrders,
    /// Removes all orders for a given open orders account from the orderbook.
    ///
    /// 0. `[writable]` market
    /// 1. `[writable]` bids
    /// 2. `[writable]` asks
    /// 3. `[signer]` prune authority
    /// 4. `[]` open orders.
    /// 5. `[]` open orders owner.
    /// 6. `[writable]` event queue.
    Prune(u16),
    /// ... `[writable]` OpenOrders
    /// accounts.len() - 3 `[writable]` market
    /// accounts.len() - 2 `[writable]` event queue
    /// accounts.len() - 1 `[signer]` crank authority
    ConsumeEventsPermissioned(u16),
    /// 0. `[writable]` market
    /// 1. `[writable]` bids
    /// 2. `[writable]` asks
    /// 3. `[writable]` OpenOrders
    /// 4. `[signer]` the OpenOrders owner
    /// 5. `[writable]` event_q
    CancelOrdersByClientIds([u64; 8]),
    /// 0. `[writable]` the market
    /// 1. `[writable]` the OpenOrders account to use
    /// 2. `[writable]` the request queue
    /// 3. `[writable]` the event queue
    /// 4. `[writable]` bids
    /// 5. `[writable]` asks
    /// 6. `[writable]` the (coin or price currency) account paying for the order
    /// 7. `[signer]` owner of the OpenOrders account
    /// 8. `[writable]` coin vault
    /// 9. `[writable]` pc vault
    /// 10. `[]` spl token program
    /// 11. `[]` the rent sysvar
    /// 12. `[]` (optional) the (M)SRM account used for fee discounts
    ReplaceOrderByClientId(NewOrderInstructionV3),
    /// 0. `[writable]` the market
    /// 1. `[writable]` the OpenOrders account to use
    /// 2. `[writable]` the request queue
    /// 3. `[writable]` the event queue
    /// 4. `[writable]` bids
    /// 5. `[writable]` asks
    /// 6. `[writable]` the (coin or price currency) account paying for the order
    /// 7. `[signer]` owner of the OpenOrders account
    /// 8. `[writable]` coin vault
    /// 9. `[writable]` pc vault
    /// 10. `[]` spl token program
    /// 11. `[]` the rent sysvar
    /// 12. `[]` (optional) the (M)SRM account used for fee discounts
    ReplaceOrdersByClientIds(Vec<NewOrderInstructionV3>),
}

impl MarketInstruction {
    pub fn pack(&self) -> Vec<u8> {
        bincode::serialize(&(0u8, self)).unwrap()
    }

    pub fn unpack(versioned_bytes: &[u8]) -> Option<Self> {
        if versioned_bytes.len() < 5 || versioned_bytes.len() > 5 + 8 + 54 * 8 {
            return None;
        }
        let (&[version], &discrim, data) = array_refs![versioned_bytes, 1, 4; ..;];
        if version != 0 {
            return None;
        }
        let discrim = u32::from_le_bytes(discrim);
        Some(match (discrim, data.len()) {
            (0, 34) => MarketInstruction::InitializeMarket({
                let data_array = array_ref![data, 0, 34];
                let fields = array_refs![data_array, 8, 8, 2, 8, 8];
                InitializeMarketInstruction {
                    coin_lot_size: u64::from_le_bytes(*fields.0),
                    pc_lot_size: u64::from_le_bytes(*fields.1),
                    fee_rate_bps: u16::from_le_bytes(*fields.2),
                    vault_signer_nonce: u64::from_le_bytes(*fields.3),
                    pc_dust_threshold: u64::from_le_bytes(*fields.4),
                }
            }),
            (1, 32) => MarketInstruction::NewOrder({
                let data_arr = array_ref![data, 0, 32];
                NewOrderInstructionV1::unpack(data_arr)?
            }),
            (2, 2) => {
                let limit = array_ref![data, 0, 2];
                MarketInstruction::MatchOrders(u16::from_le_bytes(*limit))
            }
            (3, 2) => {
                let limit = array_ref![data, 0, 2];
                MarketInstruction::ConsumeEvents(u16::from_le_bytes(*limit))
            }
            (4, 53) => MarketInstruction::CancelOrder({
                let data_array = array_ref![data, 0, 53];
                let fields = array_refs![data_array, 4, 16, 32, 1];
                let side = match u32::from_le_bytes(*fields.0) {
                    0 => Side::Bid,
                    1 => Side::Ask,
                    _ => return None,
                };
                let order_id = u128::from_le_bytes(*fields.1);
                let owner = cast(*fields.2);
                let &[owner_slot] = fields.3;
                CancelOrderInstruction {
                    side,
                    order_id,
                    owner,
                    owner_slot,
                }
            }),
            (5, 0) => MarketInstruction::SettleFunds,
            (6, 8) => {
                let client_id = array_ref![data, 0, 8];
                MarketInstruction::CancelOrderByClientId(u64::from_le_bytes(*client_id))
            }
            (7, 0) => MarketInstruction::DisableMarket,
            (8, 0) => MarketInstruction::SweepFees,
            (9, 36) => MarketInstruction::NewOrderV2({
                let data_arr = array_ref![data, 0, 36];
                let (v1_data_arr, v2_data_arr) = array_refs![data_arr, 32, 4];
                let v1_instr = NewOrderInstructionV1::unpack(v1_data_arr)?;
                let self_trade_behavior = SelfTradeBehavior::try_from_primitive(
                    u32::from_le_bytes(*v2_data_arr).try_into().ok()?,
                )
                .ok()?;
                v1_instr.add_self_trade_behavior(self_trade_behavior)
            }),
            (10, len) if len == 46 || len == 54 => MarketInstruction::NewOrderV3({
                let extended_data = match len {
                    46 => Some([data, &i64::MAX.to_le_bytes()].concat()),
                    54 => Some(data.to_vec()),
                    _ => None,
                }?;
                let data_arr = array_ref![extended_data, 0, 54];
                NewOrderInstructionV3::unpack(data_arr)?
            }),
            (11, 20) => MarketInstruction::CancelOrderV2({
                let data_arr = array_ref![data, 0, 20];
                CancelOrderInstructionV2::unpack(data_arr)?
            }),
            (12, 8) => {
                let client_id = array_ref![data, 0, 8];
                MarketInstruction::CancelOrderByClientIdV2(u64::from_le_bytes(*client_id))
            }
            (13, 46) => MarketInstruction::SendTake({
                let data_arr = array_ref![data, 0, 46];
                SendTakeInstruction::unpack(data_arr)?
            }),
            (14, 0) => MarketInstruction::CloseOpenOrders,
            (15, 0) => MarketInstruction::InitOpenOrders,
            (16, 2) => {
                let limit = array_ref![data, 0, 2];
                MarketInstruction::Prune(u16::from_le_bytes(*limit))
            }
            (17, 2) => {
                let limit = array_ref![data, 0, 2];
                MarketInstruction::ConsumeEventsPermissioned(u16::from_le_bytes(*limit))
            }
            // At most 8 client ids, each of which is 8 bytes
            (18, len) if len % 8 == 0 && len <= 8 * 8 => {
                let mut client_ids = [0; 8];
                // convert chunks of 8 bytes to client ids
                for (chunk, client_id) in data.chunks_exact(8).zip(client_ids.iter_mut()) {
                    *client_id = u64::from_le_bytes(chunk.try_into().unwrap());
                }
                MarketInstruction::CancelOrdersByClientIds(client_ids)
            }
            (19, 54) => MarketInstruction::ReplaceOrderByClientId({
                let data_arr = array_ref![data, 0, 54];
                NewOrderInstructionV3::unpack(data_arr)?
            }),
            (20, len) if len % 54 == 8 && len <= 8 + 8 * 54 => {
                if u64::from_le_bytes(data[0..8].try_into().unwrap())
                    != (data.len() as u64 - 8) / 54
                {
                    return None;
                }

                let new_orders = data[8..]
                    .chunks_exact(54)
                    .map(|chunk| {
                        let chunk_arr = array_ref![chunk, 0, 54];
                        NewOrderInstructionV3::unpack(chunk_arr)
                    })
                    .collect::<Option<Vec<_>>>()?;
                MarketInstruction::ReplaceOrdersByClientIds(new_orders)
            }
            _ => return None,
        })
    }

}
