import { Program, Provider } from '@coral-xyz/anchor';
import { TokenVoter } from './token_voter';
import TokenVoterIDL  from './token_voter.json';

export class TokenVoterClient {
  constructor(public program: Program<TokenVoter>, public devnet?: boolean) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<TokenVoterClient> {
    return new TokenVoterClient(
      new Program(TokenVoterIDL as TokenVoter, provider),
      devnet,
    );
  }
}
