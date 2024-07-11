import * as anchor from "@coral-xyz/anchor";
import { web3, Idl } from "@coral-xyz/anchor";
import { IdlAccounts, Program } from "@coral-xyz/anchor";
import * as dotenv from 'dotenv';
import bs58 from 'bs58';
import fs from "fs";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";


const loadIdl = (idlPath: string): Idl => {
  const idlStr = fs.readFileSync(idlPath, 'utf8');
  return JSON.parse(idlStr);
};

async function main() {
  dotenv.config();

  // workaround for `ANCHOR_WALLET` is not set.
  // https://github.com/coral-xyz/anchor/issues/2791
  const dummyProvider = new anchor.AnchorProvider(
    new web3.Connection(process.env.ANCHOR_PROVIDER_URL),
    new NodeWallet(web3.Keypair.generate()), {}
  );

  const jupIdl = loadIdl("jupiter_v6_converted_idl.json");

  const jupProgramId = new anchor.web3.PublicKey("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

  const program = new Program(jupIdl, dummyProvider);

  const decoder = program.coder.instruction as anchor.BorshInstructionCoder;

  const decoded = decoder.decode("e517cb977ae3ad2a010000001200640001656f200000000000e69f040000000000c80000");
  
  console.log(decoded);
}

main();