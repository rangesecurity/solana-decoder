//! Instructions decoding functions for raydium

use crate::types::{PartiallyDecodedInstruction, ProgramDecoder, ProgramDecoderMatcher};
use anyhow::{anyhow, Context};
use cpmm::CpmmDecoder as CpmmV1Decoder;
use once_cell::sync::Lazy;
use serde::de;
use std::sync::Arc;

pub mod cpmm;

pub static RAYDIUM_DECODER: Lazy<Arc<dyn ProgramDecoderMatcher>> =
    Lazy::new(|| Arc::new(RaydiumProgramDecoderMatcher {}) as Arc<dyn ProgramDecoderMatcher>);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Programs {
    CpmmV1(CpmmV1Decoder),
}

/// used to handle iplementation of the ProgramDecoderMatcher trait
/// which determines if the instruction can be decoded
pub struct RaydiumProgramDecoderMatcher {}

impl ProgramDecoder for Programs {
    fn decode(&self) -> anyhow::Result<crate::types::DecodedInstruction> {
        match self {
            Self::CpmmV1(decoder) => {
                return decoder
                    .decode()
                    .with_context(|| "failed to decde Raydium::CpmmV1");
            }
        }
    }
    fn debug(&self) -> String {
        match self {
            Self::CpmmV1(decoder) => format!("{:#?}", decoder),
        }
    }
}

impl ProgramDecoderMatcher for RaydiumProgramDecoderMatcher {
    fn try_new(&self, ix: &PartiallyDecodedInstruction) -> Option<Box<dyn ProgramDecoder>> {
        match ix.program_id {
            cpmm::PROGRAM_ID => Some(Box::new(Programs::CpmmV1(CpmmV1Decoder::new(ix.clone())))),
            _ => None,
        }
    }
}
