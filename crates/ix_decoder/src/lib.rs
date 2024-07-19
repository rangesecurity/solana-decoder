use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{
    UiInstruction, UiParsedInstruction, UiPartiallyDecodedInstruction,
};
use types::{
    DecodedInstruction, PartiallyDecodedInstruction, ProgramDecoder, ProgramDecoderMatcher,
    PROGRAM_DECODER_MATCHERS,
};
pub mod raydium;
pub mod types;

#[derive(Clone, Copy)]
pub struct DecodeMatcher {}

impl DecodeMatcher {
    /// Attemps to decode the given instruction, returning an error if we failed to decode
    pub fn try_new_decoder(self, ix: UiInstruction) -> Result<Box<dyn ProgramDecoder>> {
        match ix {
            //
            UiInstruction::Compiled(ix) => {
                return Err(anyhow!("compiled instructions not supported"))
            }
            UiInstruction::Parsed(ix) => match ix {
                UiParsedInstruction::PartiallyDecoded(ix) => {
                    return try_program_decoder(ix).with_context(|| "unrecognized instruction");
                    /*
                                        {
                      "accounts": [
                        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                        "3cdeiXyxedfzki8HTZ1DomU1HU1SbGFnv5Pttkmc8n8E",
                        "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
                        "DkhYtHGiBFBRK9priwezu16JbJjnVZGFMKxkgq69advx",
                        "2Vb8om5Ewed5LJ1X2KsRtd8p3mZX5ocBhZMJ2VDDzRmv",
                        "ESmdjKtmsUbkKV7XcNz6TrZBFKeaa5K9sVRVbGBYywVy",
                        "HyH2qkQn5fLndZvzUE3mizN99WH3rJPQCVPiJNQBMDcq",
                        "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
                        "EzEiX9G3oZrWMHp7m4aKqGNCGx77RPXMt9C7BCxfsPhR",
                        "qPFqZduU2PE9pDkcK6f4Dkr76uVSsoe8BPL7Jm9h2u4",
                        "3GTbXkHYFH4bP1SffyM1QH4HkwEnfcVz6HezA8sjRj2z",
                        "9wk9xcQhVTaZ4BQ2xRMbvhx7upXyeDgJQrvhg5nEUkBA",
                        "GQ3K2SPnznyhE4H7G6Anp5CW67UhLnCZt9XXCiog4rhL",
                        "9zh3hw8khtcgzvTrvnGp6kkJ7QYXTLdrkC75i7z13tpQ",
                        "Fz6FwoEErD69qv5Jo1wPsLuLMroG2DoAPHGyRoixDg2W",
                        "FKbm7rasypkm8YZAqMZmWD3GcbFYYU5eEnddM13tmZoj",
                        "JDzGczMYV3338frU4GGusTKbsRD4qvCiCsD7ax5LMtnU",
                        "AjVKSwyZGeGeyvmCr1FBciR6pLcaErdLEEiJGAnbq1ct"
                      ],
                      "data": "5ub2y2Px1fytRf4QS8Qy3VH",
                      "programId": "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
                      "stackHeight": null
                    }
                                         */
                }
                UiParsedInstruction::Parsed(ix) => return Err(anyhow!("unsupported instruction format"))
            },
        }
    }
}

pub fn try_program_decoder(
    ix: UiPartiallyDecodedInstruction,
) -> anyhow::Result<Box<dyn ProgramDecoder>> {
    let ix: PartiallyDecodedInstruction = TryFrom::try_from(ix)?;
    for decoder_matcher in &PROGRAM_DECODER_MATCHERS {
        match decoder_matcher.try_new(&ix) {
            Some(decoder) => {
                return Ok(decoder);
            }
            None => (),
        }
    }
    Err(anyhow!("failed to fnid a decoder"))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_program_decoder() {
        let ix_json = serde_json::json!({
          "accounts": [
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "3cdeiXyxedfzki8HTZ1DomU1HU1SbGFnv5Pttkmc8n8E",
            "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
            "DkhYtHGiBFBRK9priwezu16JbJjnVZGFMKxkgq69advx",
            "2Vb8om5Ewed5LJ1X2KsRtd8p3mZX5ocBhZMJ2VDDzRmv",
            "ESmdjKtmsUbkKV7XcNz6TrZBFKeaa5K9sVRVbGBYywVy",
            "HyH2qkQn5fLndZvzUE3mizN99WH3rJPQCVPiJNQBMDcq",
            "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
            "EzEiX9G3oZrWMHp7m4aKqGNCGx77RPXMt9C7BCxfsPhR",
            "qPFqZduU2PE9pDkcK6f4Dkr76uVSsoe8BPL7Jm9h2u4",
            "3GTbXkHYFH4bP1SffyM1QH4HkwEnfcVz6HezA8sjRj2z",
            "9wk9xcQhVTaZ4BQ2xRMbvhx7upXyeDgJQrvhg5nEUkBA",
            "GQ3K2SPnznyhE4H7G6Anp5CW67UhLnCZt9XXCiog4rhL",
            "9zh3hw8khtcgzvTrvnGp6kkJ7QYXTLdrkC75i7z13tpQ",
            "Fz6FwoEErD69qv5Jo1wPsLuLMroG2DoAPHGyRoixDg2W",
            "FKbm7rasypkm8YZAqMZmWD3GcbFYYU5eEnddM13tmZoj",
            "JDzGczMYV3338frU4GGusTKbsRD4qvCiCsD7ax5LMtnU",
            "AjVKSwyZGeGeyvmCr1FBciR6pLcaErdLEEiJGAnbq1ct"
          ],
          "data": "5ub2y2Px1fytRf4QS8Qy3VH",
          "programId": "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
          "stackHeight": null
        });

        let ix: UiPartiallyDecodedInstruction = serde_json::from_value(ix_json).unwrap();

        let decoder = try_program_decoder(ix).unwrap();
        let decoded = decoder.decode().unwrap();
        println!("{decoded:#?}");
    }
}
