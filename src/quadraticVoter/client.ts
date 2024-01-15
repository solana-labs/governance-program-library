import { Program, Provider } from '@coral-xyz/anchor';
import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { Quadratic, IDL } from './quadratic';
import { Client, DEFAULT_GOVERNANCE_PROGRAM_ID } from '../common/Client';
import { getTokenOwnerRecordAddress } from '@solana/spl-governance';

export const QUADRATIC_PLUGIN_ID = new PublicKey(
  'quadCSapU8nTdLg73KHDnmdxKnJQsh7GUbu5tZfnRRr'
);

export class QuadraticClient extends Client<Quadratic> {
  constructor(public program: Program<Quadratic>, public devnet?: boolean, readonly governanceProgramId = DEFAULT_GOVERNANCE_PROGRAM_ID) {
    super(program, devnet);
  }

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<QuadraticClient> {
    return new QuadraticClient(
      new Program<Quadratic>(IDL, QUADRATIC_PLUGIN_ID, provider),
      devnet,
    );
  }

  async createVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey): Promise<TransactionInstruction> {
    const { registrar } = this.getRegistrarPDA(realm, mint);
    const { voterWeightPk } = this.getVoterWeightRecordPDA(realm, mint, voter);

    return this.program.methods
      .createVoterWeightRecord(voter)
      .accounts({
        registrar,
        voterWeightRecord: voterWeightPk,
      })
      .instruction();
  }

  async createMaxVoterWeightRecord(realm: PublicKey, mint: PublicKey) {
    const { registrar } = this.getRegistrarPDA(realm, mint);
    const { maxVoterWeightPk } = this.getMaxVoterWeightRecordPDA(realm, mint);

    return this.program.methods
      .createMaxVoterWeightRecord()
      .accounts({
        maxVoterWeightRecord: maxVoterWeightPk,
        governanceProgramId: this.governanceProgramId,
        realm,
        realmGoverningTokenMint: mint,
      })
      .instruction();
  }

  async updateVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey) {
    const { registrar } = this.getRegistrarPDA(realm, mint);
    const { voterWeightPk } = this.getVoterWeightRecordPDA(realm, mint, voter);
    const inputVoterWeight = await this.getPredecessorVoterWeightRecordPDA(realm, mint, voter);

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
        voterWeightRecord: voterWeightPk,
      })
      .instruction();
  }

  async updateMaxVoterWeightRecord(realm: PublicKey, mint: PublicKey) {
    const { registrar } = this.getRegistrarPDA(realm, mint);
    const { maxVoterWeightPk } = this.getMaxVoterWeightRecordPDA(realm, mint);
    const inputMaxVoterWeight = await this.getPredecessorMaxVoterWeightRecordPDA(realm, mint);

    let inputMaxVoterWeightPk = inputMaxVoterWeight?.maxVoterWeightPk;

    // Check if that predecessor max voter weight actually exists
    const hasPredecessorMaxVoterWeightRecord = !!inputMaxVoterWeightPk && !!(await this.program.provider.connection.getAccountInfo(inputMaxVoterWeightPk));

    if (!hasPredecessorMaxVoterWeightRecord || !inputMaxVoterWeightPk) {
      // if there is no max voter weight from the predecessor (or no predecessor), then the mint is passed,
      // as the total supply (stored on the mint account) is the maximum voter weight by default.
      inputMaxVoterWeightPk = mint;
    }

    console.log("inputMaxVoterWeightPk", inputMaxVoterWeightPk.toBase58());

    return this.program.methods
      .updateMaxVoterWeightRecord()
      .accounts({
        registrar,
        inputMaxVoterWeight: inputMaxVoterWeightPk,
        maxVoterWeightRecord: maxVoterWeightPk,
      })
      .instruction();
  }
}
