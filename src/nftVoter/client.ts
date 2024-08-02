import { Program, Provider } from '@coral-xyz/anchor';
import { NftVoter } from './nft_voter';
import NftVoterIDL  from './nft_voter.json';

export class NftVoterClient {
  constructor(public program: Program<NftVoter>, public devnet?: boolean) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<NftVoterClient> {
    return new NftVoterClient(
      new Program(NftVoterIDL as NftVoter, provider),
      devnet,
    );
  }
}
