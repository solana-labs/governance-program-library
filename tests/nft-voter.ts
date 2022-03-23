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

    const name = (client.program.account.maxVoterWeightRecord as any)._idlAccount.name;
  //  const name = (client.program.account.nftVoteRecord as any)._idlAccount.name;
    const digestName = `account:${camelcase(name, { pascalCase: true })}`
    const sha = sha256.digest(digestName);
    const buff  =  Buffer.from(
      sha
    ).slice(0, 8);

    let array = new Uint8Array([157, 95, 242, 151, 16, 98, 26, 118]);
    const buff2 = Buffer.from(array);

    const discriminator = anchor.AccountsCoder.accountDiscriminator(name);


    console.log("INFO", {digestName,discriminator, sha, buff, array,buff2});



    // // Add your test here.
    // const all = await client.program.account.nftVoteRecord.fetch("2iVquXaAfUyB8o3umejtVM18SoV7r8VU7KXehcAPYKg3");
    // // const tx = await program.rpc.createRegistrar({});
    //  console.log("ALL", all);
  });
});
