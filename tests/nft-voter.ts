import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftVoter } from "../target/types/nft_voter";

describe("nft-voter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.NftVoter as Program<NftVoter>;

  
 

  it("Is initialized!", async () => {

    const records = program.account.nftVoteRecord.all();
    // Add your test here.
    //const tx = await program.rpc.createRegistrar({});
    console.log("Your transaction signature", records);
  });
});
