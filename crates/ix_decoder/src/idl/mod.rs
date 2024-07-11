mod types;
pub mod idl;

use {
    std::collections::HashMap,
    serde::{Serialize, Deserialize},
    solana_sdk::pubkey::Pubkey,
    types::ParsedIdl,
    anyhow::{Result, Context},
};

#[derive(Clone, Default)]
pub struct IdlParser {
    pub idls: HashMap<Pubkey, idl::Idl >
}

impl IdlParser {
    pub fn load_idl(&mut self, program_id: Pubkey, idl: &str) -> Result<()> {
        let parsed_idl: anchor_client_gen_utils::idl::IdlJsonDefinition = serde_json::from_str(idl).with_context(|| "failed to deserialize idl")?;

        for ix in parsed_idl.instructions {
            if ix.name == "setParams" {
                ix
                println!("{ix:#?}");
            }
        }
        //self.idls.insert(program_id, parsed_idl);
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_parsed_idl() {
        
        let pid: Pubkey = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".parse().unwrap();
        let mut parser = IdlParser::default();
        let idl = tokio::fs::read_to_string("../../raydium_idl.json").await.unwrap();
        parser.load_idl(pid, &idl).unwrap();

    }
}