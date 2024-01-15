// Given a governance token wallet, a realm and a plugin name, send a transaction to update the voter weight record for the given wallet address.

import { Connection, Keypair, PublicKey, TransactionInstruction, Transaction } from '@solana/web3.js';
import * as os from 'os';
import { GatewayClient, QuadraticClient } from '../src';
import { AnchorProvider, Provider, Wallet } from '@coral-xyz/anchor';
import { createAndSendV0Tx } from './utils/plugin';

const DEFAULT_KEYPAIR_PATH = os.homedir() + '/.config/solana/id.json';
const DEFAULT_RPC_URL = 'https://api.devnet.solana.com';
// Only the quadratic plugin is supported at present
const PLUGIN_NAMES = ['quadratic', 'gateway'];

// Parse the command line arguments
const [voterString, realmString, communityMintString, pluginName, rpcUrl = DEFAULT_RPC_URL] = process.argv.slice(2);
if (!voterString || !realmString || !pluginName) {
  console.error('Usage: updateVoterWeightRecord <voter> </voter><realm> <mint> <pluginName> [rpcUrl] [keypairPath]');
  process.exit(1);
}
if (!PLUGIN_NAMES.includes(pluginName)) {
  console.error(`Plugin name must be one of ${PLUGIN_NAMES}`);
  process.exit(1);
}

const keypairPath = process.env.KEYPAIR_PATH || DEFAULT_KEYPAIR_PATH;
const voterPk = new PublicKey(voterString);
const realmPk = new PublicKey(realmString);
const communityMintPk = new PublicKey(communityMintString);

// Load the payer keypair
let payer = Keypair.fromSecretKey(Buffer.from(require(keypairPath), 'hex'));
try {
  payer = Keypair.fromSecretKey(Buffer.from(require(keypairPath), 'hex'));
} catch (e) {
  console.error(`Unable to read keypair file at ${keypairPath}: ${e}`);
  process.exit(1);
}

// Connect to the cluster
const connection = new Connection(rpcUrl, 'confirmed');
const provider = new AnchorProvider(
  connection,
  new Wallet(payer), {});

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

    console.log(ix);
  }

  const maxVoterWeightRecord = await client.getMaxVoterWeightRecord(realmPk, communityMintPk);
  if (!maxVoterWeightRecord) {
    console.log("creating max voter weight record");
    const ix = await client.createMaxVoterWeightRecord(realmPk, communityMintPk);
    if (ix) ixes.push(ix);

    console.log(ix);
  }

  // update the voter weight record
  const updateVoterWeightRecordIx = await client.updateVoterWeightRecord(voterPk, realmPk, communityMintPk);
  ixes.push(updateVoterWeightRecordIx);

  const updateMaxVoterWeightRecordIx = await client.updateMaxVoterWeightRecord(realmPk, communityMintPk);
  if (updateMaxVoterWeightRecordIx) ixes.push(updateMaxVoterWeightRecordIx);

  await createAndSendV0Tx(connection, payer, ixes);

  const { voterWeightPk } = client.getVoterWeightRecordPDA(realmPk, communityMintPk, voterPk);
  console.log("Voter weight record", voterWeightPk.toBase58());
})().catch((err) => {
  console.error(err);
  process.exit(1);
});