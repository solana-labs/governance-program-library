import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftVoter } from "../target/types/nft_voter";
import { NftVoterClient } from "../src/client"
import camelcase from "camelcase";
import { sha256 } from "js-sha256";

describe("nft-voter", () => {




  // Configure the client to use the local cluster.
 // anchor.setProvider(anchor.Provider.env());

 // const program = anchor.workspace.NftVoter as Program<NftVoter>;

 

  it("Is initialized!", async () => {
    const client =  await NftVoterClient.connect(anchor.Provider.env());

    const name = (client.program.account.nftVoteRecord as any)._idlAccount.name;
    const digestName = `account:${camelcase(name, { pascalCase: true })}`
    const sha = sha256.digest(digestName);
    const buffFromSha  =  Buffer.from(
      sha
    ).slice(0, 8);

    const dd2 = Buffer.from(
      sha256.digest(`account:${camelcase(name, { pascalCase: true })}`)
    ).slice(0, 8);

    let arrayFromSha = new Uint8Array([137,   6,  55, 139, 251, 126, 254,  99]);
    const buffFromArray = Buffer.from(arrayFromSha);

    const discriminator = anchor.AccountsCoder.accountDiscriminator(name);


    console.log("INFO", {discriminator,dd2, sha, buffFromSha,arrayFromSha,buffFromArray});
    //console.log("INFO", {digestName,discriminator, sha,  buffFromSha, array,buff2});



    // // Add your test here.
     const all = await client.program.account.nftVoteRecord.fetch("5SuT66KYjfWmURKpamgWcn5BHmiisHFHBTXfD7zZGWBt");
    // // // const tx = await program.rpc.createRegistrar({});
      console.log("ALL", all);
  });
});
