import { Program, Provider } from '@coral-xyz/anchor';
import { Gateway } from './gateway';
import GatewayIDL  from './gateway.json';

export class GatewayClient {
  constructor(public program: Program<Gateway>, public devnet?: boolean) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<GatewayClient> {
    return new GatewayClient(
      new Program(GatewayIDL as Gateway, provider),
      devnet,
    );
  }
}
