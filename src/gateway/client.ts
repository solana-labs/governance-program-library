import { BN, Program, Provider } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { Gateway } from './gateway';
import GatewayIDL  from './gateway.json';
import { Client, DEFAULT_GOVERNANCE_PROGRAM_ID } from '../common/Client';
import { getGatewayTokenAddressForOwnerAndGatekeeperNetwork, getGatewayToken } from '@identity.com/solana-gateway-ts';
import { getTokenOwnerRecordAddress, VoterWeightAction } from '@solana/spl-governance';

export const GATEWAY_PLUGIN_ID = new PublicKey(
  'GgathUhdrCWRHowoRKACjgWhYHfxCEdBi5ViqYN6HVxk'
);

export class GatewayClient extends Client<Gateway> {
  readonly requiresInputVoterWeight = true;
  constructor(public program: Program<Gateway>, public devnet?: boolean, readonly governanceProgramId = DEFAULT_GOVERNANCE_PROGRAM_ID) {
    super(program, devnet);
  }

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<GatewayClient> {
    return new GatewayClient(
      new Program(GatewayIDL as Gateway, provider),
      devnet,
    );
  }

  async calculateVoterWeight(voter: PublicKey, realm: PublicKey, mint: PublicKey, inputVoterWeight: BN): Promise<BN | null> {
    try {
      const gatewayTokenPDA = await this.getGatewayTokenPDA(voter, realm, mint);

      const gatewayToken = await getGatewayToken(this.program.provider.connection, gatewayTokenPDA);

      // fail out if the token is not valid
      if (!gatewayToken || !gatewayToken.isValid()) return null;

      // otherwise, the input voter weight is passed through
      return inputVoterWeight;
    } catch (e) {
      console.log('Error fetching gateway token PDA', e);
      return null; // fail out if we can't get the registrar or gateway token PDA
    }
  }

  async getGatewayTokenPDA(voter: PublicKey, realm: PublicKey, mint: PublicKey) {
    const registrar = await this.getRegistrarAccount(realm, mint);
    if (!registrar) {
      throw new Error('No registrar found');
    }

    const { gatekeeperNetwork } = registrar;
    return getGatewayTokenAddressForOwnerAndGatekeeperNetwork(voter, gatekeeperNetwork);
  }

  async createVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey) {
    const { registrar } = this.getRegistrarPDA(realm, mint);

    return this.program.methods
      .createVoterWeightRecord(voter)
      .accounts({
        registrar,
      })
      .instruction();
  }

  async createMaxVoterWeightRecord() {
    return null;
  }

  async updateVoterWeightRecord(
    voter: PublicKey,
    realm: PublicKey,
    mint: PublicKey,
    action?: VoterWeightAction,
    inputRecordCallback?: () => Promise<PublicKey>
  ) {
    const { registrar } = this.getRegistrarPDA(realm, mint);
    const { voterWeightPk } = await this.getVoterWeightRecordPDA(realm, mint, voter);

    // if the previous plugin has a specific way of deriving the input voter weight, use it
    // otherwise derive it the default way.
    const [inputVoterWeightPk, gatewayToken] = await Promise.all([
      this.getPredecessorVoterWeightRecordPDA(realm, mint, voter, inputRecordCallback),
      this.getGatewayTokenPDA(voter, realm, mint)
    ])

    const ix = await this.program.methods
      .updateVoterWeightRecord()
      .accounts({
        registrar,
        inputVoterWeight: inputVoterWeightPk,
        gatewayToken,
        voterWeightRecord: voterWeightPk,
      })
      .instruction();

    return { pre: [ix] }
  }

  async updateMaxVoterWeightRecord() {
    return null;
  }
}
