// Given a governance token wallet, a realm and a plugin name, send a transaction to update the voter weight record for the given wallet address.
import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { GatewayClient, QuadraticClient } from '../src';
import { Provider } from '@coral-xyz/anchor';
import { createAndSendV0Tx } from './utils/plugin';
import { DEFAULT_RPC_URL } from './utils/constants';
import { getProvider, payer } from './utils/common';

const PLUGIN_NAMES = ['quadratic', 'gateway'];

// Parse the command line arguments
const [voterString, realmString, communityMintString, pluginName, rpcUrl = DEFAULT_RPC_URL] = process.argv.slice(2);
if (!voterString || !realmString || !pluginName) {
  console.error('Usage: updateVoterWeightRecord <voter> <realm> <mint> <pluginName> [rpcUrl]');
  process.exit(1);
}
if (!PLUGIN_NAMES.includes(pluginName)) {
  console.error(`Plugin name must be one of ${PLUGIN_NAMES}`);
  process.exit(1);
}

const voterPk = new PublicKey(voterString);
const realmPk = new PublicKey(realmString);
const communityMintPk = new PublicKey(communityMintString);

// Connect to the cluster
const provider = getProvider(rpcUrl);

const loadClient = (plugin: typeof PLUGIN_NAMES[number], provider: Provider) => {
  switch (plugin) {
    case 'quadratic':
      return QuadraticClient.connect(provider)
    case 'gateway':
      return GatewayClient.connect(provider)
    default:
      throw new Error(`Unsupported plugin ${plugin}`);
  }
}

(async () => {
  // Get the plugin client
  const client = await loadClient(pluginName, provider);

  // check if the voter weight record exists already. If not, create it.
  const ixes: TransactionInstruction[] = [];

  const voterWeightRecord = await client.getVoterWeightRecord(realmPk, communityMintPk, voterPk);
  if (!voterWeightRecord) {
    console.log("creating voter weight record");
    const ix = await client.createVoterWeightRecord(voterPk, realmPk, communityMintPk);
    ixes.push(ix);
  }

  const maxVoterWeightRecord = await client.getMaxVoterWeightRecord(realmPk, communityMintPk);
  if (!maxVoterWeightRecord) {
    console.log("creating max voter weight record");
    const ix = await client.createMaxVoterWeightRecord(realmPk, communityMintPk);
    if (ix) ixes.push(ix);
  }

  // update the voter weight record
  const updateVoterWeightRecordIx = await client.updateVoterWeightRecord(voterPk, realmPk, communityMintPk);
  ixes.push(...updateVoterWeightRecordIx.pre);

  const updateMaxVoterWeightRecordIx = await client.updateMaxVoterWeightRecord(realmPk, communityMintPk);
  if (updateMaxVoterWeightRecordIx) ixes.push(updateMaxVoterWeightRecordIx);

  await createAndSendV0Tx(provider.connection, payer, ixes);

  const { voterWeightPk } = client.getVoterWeightRecordPDA(realmPk, communityMintPk, voterPk);
  console.log("Voter weight record", voterWeightPk.toBase58());

  const maxVoterWeight = client.getMaxVoterWeightRecordPDA(realmPk, communityMintPk);
  if (!maxVoterWeight) {
    console.log("No max voter weight record found")
    return
  }
  console.log("Max voter weight record", maxVoterWeight.maxVoterWeightPk.toBase58());
})().catch((err) => {
  console.error(err);
  process.exit(1);
});