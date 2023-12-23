import { Program, Provider } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { Quadratic, IDL } from './quadratic';

export const QUADRATIC_PLUGIN_ID = new PublicKey(
  'quadCSapU8nTdLg73KHDnmdxKnJQsh7GUbu5tZfnRRr'
);

export class QuadraticClient {
  constructor(public program: Program<Quadratic>, public devnet?: boolean) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<QuadraticClient> {
    const idl = IDL;

    return new QuadraticClient(
      new Program<Quadratic>(idl, QUADRATIC_PLUGIN_ID, provider),
      devnet,
    );
  }
}
