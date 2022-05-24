import { Program, Provider } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { Gateway, IDL } from './gateway';

export const GATEWAY_PLUGIN_ID = new PublicKey(
  'Ggatr3wgDLySEwA2qEjt1oiw4BUzp5yMLJyz21919dq6'
);

export class GatewayClient {
  constructor(public program: Program<Gateway>, public devnet?: boolean) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<GatewayClient> {
    // alternatively we could fetch from chain
    // const idl = await Program.fetchIdl(GATEWAY_PLUGIN_ID, provider);
    const idl = IDL;

    return new GatewayClient(
      new Program<Gateway>(idl as Gateway, GATEWAY_PLUGIN_ID, provider),
      devnet,
    );
  }
}
