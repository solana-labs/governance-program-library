import { BN, Program, Provider } from '@coral-xyz/anchor';
import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { Quadratic } from './quadratic';
import QuadraticIDL from './quadratic.json';
import { Client, DEFAULT_GOVERNANCE_PROGRAM_ID } from '../common/Client';
import { getTokenOwnerRecordAddress, VoterWeightAction } from '@solana/spl-governance';

export const QUADRATIC_PLUGIN_ID = new PublicKey(
  'quadCSapU8nTdLg73KHDnmdxKnJQsh7GUbu5tZfnRRr'
);

export type Coefficients = [ a: number, b: number, c: number ];

const toAnchorType = (coefficients: Coefficients) => ({
  a: coefficients[0],
  b: coefficients[1],
  c: coefficients[2],
});


// By default, the quadratic plugin will use a function ax-2 + bx - c
// resulting in a vote weight that is the square root of the token balance
export const DEFAULT_COEFFICIENTS: Coefficients = [ 1, 0, 0 ];

export class QuadraticClient extends Client<Quadratic> {
  readonly requiresInputVoterWeight = true;
  constructor(public program: Program<Quadratic>, public devnet?: boolean, readonly governanceProgramId = DEFAULT_GOVERNANCE_PROGRAM_ID) {
    super(program, devnet);
  }

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<QuadraticClient> {
    return new QuadraticClient(
      new Program<Quadratic>(QuadraticIDL as Quadratic, provider),
      devnet,
    );
  }

  async configureRegistrar(realm: PublicKey, mint: PublicKey, previousVoterWeightPluginProgramId?: PublicKey, coefficients = DEFAULT_COEFFICIENTS) {
    const { registrar, registrarBump } = this.getRegistrarPDA(realm, mint);

    const methodsBuilder = this.program.methods
      .configureRegistrar(toAnchorType(coefficients), !!previousVoterWeightPluginProgramId)
      .accounts({
        registrar,
        realm,
        realmAuthority: this.program.provider.publicKey!,
      });

    if (previousVoterWeightPluginProgramId) {
      methodsBuilder.remainingAccounts([{
        pubkey: previousVoterWeightPluginProgramId,
        isSigner: false,
        isWritable: false
      }])
    }

    return methodsBuilder
      .instruction();
  }

  async calculateVoterWeight(voter: PublicKey, realm: PublicKey, mint: PublicKey, inputVoterWeight: BN): Promise<BN | null> {
    const registrar = await this.getRegistrarAccount(realm, mint);

    // No registrar yet, QV weight cannot be calculated
    if (!registrar) return null;
    // @ts-ignore: below should return ok
    const coefficients = registrar.quadraticCoefficients;

    // otherwise, the input voter weight is passed through
    return QuadraticClient.applyCoefficients(
      inputVoterWeight,
      QuadraticClient.convertCoefficientsFromAnchorType(coefficients)
    );
  }

  public static convertCoefficientsFromAnchorType(coefficients: { a: number, b: number, c: number }): Coefficients {
    return [ coefficients.a, coefficients.b, coefficients.c ];
  }

  public static applyCoefficients(inputVoterWeight: BN, coefficients: Coefficients): BN {
    const [ a, b, c ] = coefficients

    const number = inputVoterWeight.toNumber();
    const rootX = Math.sqrt(inputVoterWeight.toNumber());

    return new BN(
      Math.floor(
        a * rootX + b * number + c
      )
    )
  }

  async createVoterWeightRecord(voter: PublicKey, realm: PublicKey, mint: PublicKey): Promise<TransactionInstruction> {
    const { registrar } = this.getRegistrarPDA(realm, mint);

    return this.program.methods
      .createVoterWeightRecord(voter)
      .accounts({
        registrar,
        payer: voter,
      })
      .instruction();
  }

  async createMaxVoterWeightRecord(realm: PublicKey, mint: PublicKey) {
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
    const inputVoterWeightPk = await this.getPredecessorVoterWeightRecordPDA(realm, mint, voter, inputRecordCallback);

    const ix = await this.program.methods
      .updateVoterWeightRecord()
      .accounts({
        registrar,
        inputVoterWeight: inputVoterWeightPk,
        voterWeightRecord: voterWeightPk,
      })
      .instruction();

    return { pre: [ix] }
  }

  async updateMaxVoterWeightRecord(realm: PublicKey, mint: PublicKey) {
    return null;
  }
}
