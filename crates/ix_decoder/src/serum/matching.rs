use {
    num_enum::{TryFromPrimitive, IntoPrimitive},
    serde::{Serialize, Deserialize},
};

#[derive(
    Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Debug, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

#[derive(
    Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Debug, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum OrderType {
    Limit = 0,
    ImmediateOrCancel = 1,
    PostOnly = 2,
}
