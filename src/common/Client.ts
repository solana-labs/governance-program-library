import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { BN, Idl, Program } from '@coral-xyz/anchor';
import { PluginProgramAccounts } from './types';
import { IdlAccounts } from '@coral-xyz/anchor/dist/cjs/program/namespace/types';
import { getTokenOwnerRecordAddress, VoterWeightAction } from '@solana/spl-governance';

export const DEFAULT_GOVERNANCE_PROGRAM_ID = new PublicKey("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");

export abstract class Client<T extends Idl> {
  abstract readonly requiresInputVoterWeight: boolean;
  protected constructor(public program: Program<T>, public devnet?: boolean) {}

  abstract createVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey): Promise<TransactionInstruction | null>;
  abstract createMaxVoterWeightRecord(realm: PublicKey, mint: PublicKey): Promise<TransactionInstruction | null>;
  abstract updateVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey, action?: VoterWeightAction, inputRecordCallback?: () => Promise<PublicKey>, target?: PublicKey): Promise<{ pre: TransactionInstruction[], post?: TransactionInstruction[] }>;
  abstract updateMaxVoterWeightRecord(realm: PublicKey, mint: PublicKey, action?: VoterWeightAction, inputRecordCallback?: () => Promise<PublicKey>): Promise<TransactionInstruction | null>;

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
    const registrarObject = await (this.program.account as PluginProgramAccounts<T>).registrar.fetchNullable(
      registrar
    )
    // previousVoterWeightPluginProgramId should be added to the object automatically by the type inference from IsPluginIdl
    // but Typescript does not seem to be that clever yet
    return registrarObject as IdlAccounts<T>['registrar'] & { previousVoterWeightPluginProgramId: PublicKey } | null;
  }

  /**
   * If this plugin uses a persistent voter weight record, get it
   * @param realm
   * @param mint
   * @param walletPk
   */
  async getVoterWeightRecord (realm: PublicKey, mint: PublicKey, walletPk: PublicKey): Promise<IdlAccounts<T>['voterWeightRecord'] | null>  {
    const { voterWeightPk } = await this.getVoterWeightRecordPDA(realm, mint, walletPk);
    const voterWeightRecordAccount = (this.program.account as PluginProgramAccounts<T>).voterWeightRecord;

    // TODO handle this at the type-level with a better PluginProgramAccounts type.
    if (!voterWeightRecordAccount) return null;

    const voterWeightRecord = await voterWeightRecordAccount.fetchNullable(voterWeightPk);

    return voterWeightRecord as IdlAccounts<T>['voterWeightRecord'];
  }

  async getMaxVoterWeightRecord (realm: PublicKey, mint: PublicKey) {
    const { maxVoterWeightPk } =  await this.getMaxVoterWeightRecordPDA(realm, mint) ?? {};
    if (!maxVoterWeightPk) return null; // this plugin does not have a max voter weight record

    const maxVoterWeightRecordAccount = (this.program.account as PluginProgramAccounts<T>).maxVoterWeightRecord;

    // TODO handle this at the type-level with a better PluginProgramAccounts type.
    if (!maxVoterWeightRecordAccount) return null;

    const maxVoterWeightRecord = await maxVoterWeightRecordAccount.fetchNullable(maxVoterWeightPk);

    return maxVoterWeightRecord as IdlAccounts<T>['maxVoterWeightRecord'];
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

  // Should be overridden by the plugin if the plugin has a max voter weight record (most plugins do not)
  // Boilerplate for those plugins:
  // return Client.getMaxVoterWeightRecordPDAForProgram(realm, mint, this.program.programId)
  async getMaxVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey): Promise<{
    maxVoterWeightPk: PublicKey;
    maxVoterWeightRecordBump: number;
  } | null> {
    return null;
  }

  protected async getPredecessorVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey, voter: PublicKey, inputRecordCallback?: () => Promise<PublicKey>) : Promise<PublicKey> {
    // if the previous plugin has a specific way of deriving the input voter weight, use it
    // otherwise derive it the default way.
    if (inputRecordCallback) {
      return inputRecordCallback();
    }
    const inputVoterWeight = await this.derivePredecessorVoterWeightRecordPDA(realm, mint, voter);

    if (inputVoterWeight) return inputVoterWeight.voterWeightPk;

    // no predecessor voter weight record found - pass the token owner record
    return getTokenOwnerRecordAddress(DEFAULT_GOVERNANCE_PROGRAM_ID, realm, mint, voter);
  }

  protected static deriveVoterWeightRecordPDAForProgram(realm: PublicKey, mint: PublicKey, walletPk: PublicKey, programId: PublicKey): {
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

  async getVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey, walletPk: PublicKey): Promise<{
    voterWeightPk: PublicKey;
    voterWeightRecordBump: number;
  }> {
    return Client.deriveVoterWeightRecordPDAForProgram(realm, mint, walletPk, this.program.programId)
  }

  async derivePredecessorVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey, walletPk: PublicKey): Promise<{
    voterWeightPk: PublicKey;
    voterWeightRecordBump: number;
  } | undefined> {
    const predecessorProgramId = await this.getPredecessorProgramId(realm, mint);

    if (!predecessorProgramId) return undefined;
    return Client.deriveVoterWeightRecordPDAForProgram(realm, mint, walletPk, predecessorProgramId);
  }

  /**
   * Returns the PDA of the max voter weight record for the predecessor program.
   * WARNING: This function just derives the PDA, but the max voter weight record itself may not exist.
   * @param realm
   * @param mint
   */
  async derivePredecessorMaxVoterWeightRecordPDA(realm: PublicKey, mint: PublicKey): Promise<{
    maxVoterWeightPk: PublicKey;
    maxVoterWeightRecordBump: number;
  } | undefined> {
    const predecessorProgramId = await this.getPredecessorProgramId(realm, mint);

    if (!predecessorProgramId) return undefined;
    return Client.getMaxVoterWeightRecordPDAForProgram(realm, mint, predecessorProgramId);
  }

  async getPredecessorProgramId(realm: PublicKey, mint: PublicKey): Promise<PublicKey | undefined> {
    // Get the registrar for the realm
    const registrarObject = await this.getRegistrarAccount(realm, mint);

    // Find the gatekeeper network from the registrar
    return registrarObject?.previousVoterWeightPluginProgramId
  }
}