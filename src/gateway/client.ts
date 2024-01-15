import { Program, Provider } from '@coral-xyz/anchor';
import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { Gateway, IDL } from './gateway';
import { Client, DEFAULT_GOVERNANCE_PROGRAM_ID } from '../common/Client';
import { getGatewayTokenAddressForOwnerAndGatekeeperNetwork } from '@identity.com/solana-gateway-ts';
import { getTokenOwnerRecordAddress } from '@solana/spl-governance';

export const GATEWAY_PLUGIN_ID = new PublicKey(
  'GgathUhdrCWRHowoRKACjgWhYHfxCEdBi5ViqYN6HVxk'
);

export class GatewayClient extends Client<Gateway> {
  constructor(public program: Program<Gateway>, public devnet?: boolean, readonly governanceProgramId = DEFAULT_GOVERNANCE_PROGRAM_ID) {
    super(program, devnet);
  }

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<GatewayClient> {
    // alternatively we could fetch from chain
    // const idl = await Program.fetchIdl(GATEWAY_PLUGIN_ID, provider);

    return new GatewayClient(
      new Program<Gateway>(IDL, GATEWAY_PLUGIN_ID, provider),
      devnet,
    );
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
    const { voterWeightPk } = this.getVoterWeightRecordPDA(realm, mint, voter);

    console.log({
      registrar: registrar.toBase58(),
      voterWeightRecord: voterWeightPk.toBase58(),
    });

    return this.program.methods
      .createVoterWeightRecord(voter)
      .accounts({
        registrar,
        voterWeightRecord: voterWeightPk,
      })
      .instruction();
  }

  async createMaxVoterWeightRecord() {
    return null;
  }

  async updateVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey) {
    const { registrar } = this.getRegistrarPDA(realm, mint);
    const { voterWeightPk } = this.getVoterWeightRecordPDA(realm, mint, voter);

    const [inputVoterWeight, gatewayToken] = await Promise.all([
      this.getPredecessorVoterWeightRecordPDA(realm, mint, voter),
      this.getGatewayTokenPDA(voter, realm, mint)
    ]);

    let inputVoterWeightPk = inputVoterWeight?.voterWeightPk;
    if (!inputVoterWeightPk) {
      // no predecessor voter weight record found - pass the token owner record
      inputVoterWeightPk = await getTokenOwnerRecordAddress(this.governanceProgramId, realm, mint, voter);
    }

    return this.program.methods
      .updateVoterWeightRecord()
      .accounts({
        registrar,
        inputVoterWeight: inputVoterWeightPk,
        gatewayToken,
        voterWeightRecord: voterWeightPk,
      })
      .instruction();
  }

  async updateMaxVoterWeightRecord() {
    return null;
  }
}
