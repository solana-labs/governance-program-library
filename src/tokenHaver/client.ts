import { Program, Provider } from '@coral-xyz/anchor';
import { TokenHaver } from './token_haver';
import TokenHaverIDL  from './token_haver.json';

export class TokenHaverClient {
  constructor(public program: Program<TokenHaver>, public devnet?: boolean) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<TokenHaverClient> {
    return new TokenHaverClient(
      new Program(TokenHaverIDL as TokenHaver, provider),
      devnet,
    );
  }
}
