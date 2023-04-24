import { Program, Provider } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { NftVoter, IDL } from './nft_voter';

export const NFT_VOTER_ID = new PublicKey(
  'GnftV5kLjd67tvHpNGyodwWveEKivz3ZWvvE3Z4xi2iw',
);

export class NftVoterClient {
  constructor(public program: Program<NftVoter>, public devnet?: boolean) {}

  static connect(
    provider: Provider,
    devnet?: boolean,
    programId = NFT_VOTER_ID,
  ): NftVoterClient {
    return new NftVoterClient(
      new Program<NftVoter>(IDL, programId, provider),
      devnet,
    );
  }
}
