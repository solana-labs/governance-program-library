// Given a governance token wallet, a realm and a plugin name, send a transaction to update the voter weight record for the given wallet address.
import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { GatewayClient, QuadraticClient } from '../src';
import { Provider } from '@coral-xyz/anchor';
import { createAndSendV0Tx } from './utils/plugin';
import { DEFAULT_RPC_URL } from './utils/constants';
import { getProvider, payer } from './utils/common';

// Parse the command line arguments
const [voterString, realmString, communityMintString, rpcUrl = DEFAULT_RPC_URL] = process.argv.slice(2);
if (!voterString || !realmString) {
  console.error('Usage: getQuadraticVoterWeight <voter> <realm> <mint> [rpcUrl]');
  process.exit(1);
}

const voterPk = new PublicKey(voterString);
const realmPk = new PublicKey(realmString);
const communityMintPk = new PublicKey(communityMintString);

// Connect to the cluster
const provider = getProvider(rpcUrl);

const loadClient = (provider: Provider) => QuadraticClient.connect(provider);

(async () => {
  // Get the plugin client
  const client = await loadClient(provider);

  const registrar = await client.getRegistrarAccount(realmPk, communityMintPk);
  const voterWeightRecord = await client.getVoterWeightRecord(realmPk, communityMintPk, voterPk);
  if (!voterWeightRecord) {
    console.error("Voter weight record not found");
    process.exit(1);
  }

  console.log("Quadratic coefficients: ", registrar?.quadraticCoefficients);
  console.log("Voter weight:" + voterWeightRecord.voterWeight.toString());
})().catch((err) => {
  console.error(err);
  process.exit(1);
});