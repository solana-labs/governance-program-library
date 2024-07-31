import { Program, Provider } from '@coral-xyz/anchor';
import { RealmVoter } from './realm_voter';
import RealmVoterIDL  from './realm_voter.json';

export class RealmVoterClient {
  constructor(public program: Program<RealmVoter>, public devnet?: boolean) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<RealmVoterClient> {
    return new RealmVoterClient(
      new Program(RealmVoterIDL as RealmVoter, provider),
      devnet,
    );
  }
}
