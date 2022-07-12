import { Program, Provider } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { RealmVoter, IDL } from './realm_voter';

export const REALM_VOTER_ID = new PublicKey(
  'GRmVtfLq2BPeWs5EDoQoZc787VYkhdkA11k63QM1Xemz',
);

export class RealmVoterClient {
  constructor(public program: Program<RealmVoter>, public devnet?: boolean) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<RealmVoterClient> {
    // alternatively we could fetch from chain
    // const idl = await Program.fetchIdl(VSR_ID, provider);
    const idl = IDL;

    return new RealmVoterClient(
      new Program<RealmVoter>(idl as RealmVoter, REALM_VOTER_ID, provider),
      devnet,
    );
  }
}
