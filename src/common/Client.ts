import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { BN, Idl, Program } from '@coral-xyz/anchor';
import { IsPluginIdl, PluginProgramAccounts } from './types';
import { IdlAccounts } from '@coral-xyz/anchor/dist/cjs/program/namespace/types';

export const DEFAULT_GOVERNANCE_PROGRAM_ID = new PublicKey("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");

export abstract class Client<T extends Idl, U extends IsPluginIdl<T> = IsPluginIdl<T>> {
  protected constructor(public program: Program<U>, public devnet?: boolean) {}

  abstract createVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey): Promise<TransactionInstruction>;
  abstract createMaxVoterWeightRecord(realm: PublicKey, mint: PublicKey): Promise<TransactionInstruction | null>;
  abstract updateVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey): Promise<TransactionInstruction>;
  abstract updateMaxVoterWeightRecord(realm: PublicKey, mint: PublicKey): Promise<TransactionInstruction | null>;

  /**
   * Preview what this voter weight plugin does to a voter's vote weight.
   * This can be used by clients to show the voter weight on the UI before a vote.
   * This function should try its best not to throw an error. A return value of null (as opposed to zero)
   * means "something is preventing this user from voting". This could be a missing registrar or some
   * other invalid status.
   *
   * Since the function does not expect any voter weights to be registered on chain yet (or up-to-date)
   * @param voter
   * @param realm
   * @param mint
   * @param inputVoterWeight
   */
  abstract calculateVoterWeight(voter: PublicKey, realm: PublicKey, mint: PublicKey, inputVoterWeight: BN): Promise<BN | null>;

  /**
   * Preview what this voter weight plugin does to the max voter's vote weight.
   * This is equivalent to calculateVoterWeight, but it has a default implementation,
   * that just returns the inputMaxVoterWeight, because plugins that set the max voter weight
   * are rarer.
   * @param _realm
   * @param _mint
   * @param inputMaxVoterWeight
   */
  async calculateMaxVoterWeight(_realm: PublicKey, _mint: PublicKey, inputMaxVoterWeight: BN): Promise<BN | null> {
    return inputMaxVoterWeight;
  }

  async getRegistrarAccount(realm: PublicKey, mint: PublicKey) {
    const { registrar } = this.getRegistrarPDA(
      realm,
      mint,
    );
    const registrarObject = await (this.program.account as PluginProgramAccounts<U>).registrar.fetchNullable(
      registrar
    )
    // previousVoterWeightPluginProgramId should be added to the object automatically by the type inference from IsPluginIdl
    // but Typescript does not seem to be that clever yet
    return registrarObject as IdlAccounts<U>['registrar'] & { previousVoterWeightPluginProgramId: PublicKey } | null;
  }

  async getVoterWeightRecord (realm: PublicKey, mint: PublicKey, walletPk: PublicKey) {
    const { voterWeightPk } = this.getVoterWeightRecordPDA(realm, mint, walletPk);
    const voterWeightRecord = await (this.program.account as PluginProgramAccounts<U>).voterWeightRecord.fetchNullable(voterWeightPk);

    return voterWeightRecord as IdlAccounts<U>['voterWeightRecord'];
  }

  async getMaxVoterWeightRecord (realm: PublicKey, mint: PublicKey) {
    const { maxVoterWeightPk } = this.getMaxVoterWeightRecordPDA(realm, mint);
    const maxVoterWeightRecordAccount = (this.program.account as PluginProgramAccounts<U>).maxVoterWeightRecord;

    // TODO handle this at the type-level with a better PluginProgramAccounts type.
    if (!maxVoterWeightRecordAccount) return null;

    const maxVoterWeightRecord = await maxVoterWeightRecordAccount.fetchNullable(maxVoterWeightPk);

    return maxVoterWeightRecord as IdlAccounts<U>['maxVoterWeightRecord'];
  }

  getRegistrarPDA(realm: PublicKey, mint: PublicKey):{
    registrar: PublicKey;
    registrarBump: number;
  } {
    const [registrar, registrarBump] = PublicKey.findProgramAddressSync(
      [Buffer.from('registrar'), realm.toBuffer(), mint.toBuffer()],
      this.program.programId
    )
    return {
      registrar,
      registrarBump,
    }
  }

  getMaxVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey): {
    maxVoterWeightPk: PublicKey;
    maxVoterWeightRecordBump: number;
  } {
    return Client.getMaxVoterWeightRecordPDAForProgram(realm, mint, this.program.programId)
  }

  protected static getVoterWeightRecordPDAForProgram(realm: PublicKey, mint: PublicKey, walletPk: PublicKey, programId: PublicKey): {
    voterWeightPk: PublicKey;
    voterWeightRecordBump: number;
  } {
    const [
      voterWeightPk,
      voterWeightRecordBump,
    ] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('voter-weight-record'),
        realm.toBuffer(),
        mint.toBuffer(),
        walletPk.toBuffer(),
      ],
      programId
    )

    return {
      voterWeightPk,
      voterWeightRecordBump,
    }
  }

  protected static getMaxVoterWeightRecordPDAForProgram(realm: PublicKey, mint: PublicKey, programId: PublicKey): {
    maxVoterWeightPk: PublicKey;
    maxVoterWeightRecordBump: number;
  } {
    const [
      maxVoterWeightPk,
      maxVoterWeightRecordBump,
    ] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('max-voter-weight-record'),
        realm.toBuffer(),
        mint.toBuffer(),
      ],
      programId
    )
    return {
      maxVoterWeightPk,
      maxVoterWeightRecordBump,
    }
  }

  getVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey, walletPk: PublicKey): {
    voterWeightPk: PublicKey;
    voterWeightRecordBump: number;
  } {
    return Client.getVoterWeightRecordPDAForProgram(realm, mint, walletPk, this.program.programId)
  }

  async getPredecessorVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey, walletPk: PublicKey): Promise<{
    voterWeightPk: PublicKey;
    voterWeightRecordBump: number;
  } | undefined> {
    const predecessorProgramId = await this.getPredecessorProgramId(realm, mint);

    if (!predecessorProgramId) return undefined;
    return Client.getVoterWeightRecordPDAForProgram(realm, mint, walletPk, predecessorProgramId);
  }

  /**
   * Returns the PDA of the max voter weight record for the predecessor program.
   * WARNING: This function just derives the PDA, but the max voter weight record itself may not exist.
   * @param realm
   * @param mint
   */
  async getPredecessorMaxVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey): Promise<{
    maxVoterWeightPk: PublicKey;
    maxVoterWeightRecordBump: number;
  } | undefined> {
    const predecessorProgramId = await this.getPredecessorProgramId(realm, mint);

    if (!predecessorProgramId) return undefined;
    return Client.getMaxVoterWeightRecordPDAForProgram(realm, mint, predecessorProgramId);
  }

  async getPredecessorProgramId(realm: PublicKey, mint: PublicKey): Promise<PublicKey | undefined> {
    // Get the registrar for the realm
    const { registrar } = this.getRegistrarPDA(
      realm,
      mint,
    );
    const registrarObject = await this.getRegistrarAccount(realm, mint);

    // Find the gatekeeper network from the registrar
    return registrarObject?.previousVoterWeightPluginProgramId
  }
}