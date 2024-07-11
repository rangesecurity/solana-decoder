import * as anchor from "@coral-xyz/anchor";
import { web3, Idl } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import fs from "fs";

export interface Decoder {
  idl: Idl;
}

const loadIdl = (idlPath: string): Idl => {
    const idlStr = fs.readFileSync(idlPath, 'utf8');
    return JSON.parse(idlStr);
  };

const newConnection = (endpoint: string): web3.Connection => {
  return new web3.Connection(endpoint);
};


const initAnchorDecoder = (idl: Idl) => {

}