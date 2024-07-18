import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftVoter } from "../target/types/nft_voter";

describe("nft-voter", () => {

  const program = anchor.workspace.NftVoter as Program<NftVoter>;

  it("Is initialized!", async () => {

    const records = program.account.voterWeightRecord.all();
    // Add your test here.
    //const tx = await program.rpc.createRegistrar({});
    console.log("Your transaction signature", records);
  });
});
