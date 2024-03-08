// Load the payer keypair
import { Connection, Keypair } from '@solana/web3.js';
import { DEFAULT_KEYPAIR_PATH } from './constants';
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';

const keypairPath = process.env.KEYPAIR_PATH || DEFAULT_KEYPAIR_PATH;
let keypair = Keypair.fromSecretKey(Buffer.from(require(keypairPath), 'hex'));
try {
  keypair = Keypair.fromSecretKey(Buffer.from(require(keypairPath), 'hex'));
} catch (e) {
  console.error(`Unable to read keypair file at ${keypairPath}: ${e}`);
  process.exit(1);
}

const getConnection = (rpcUrl: string) => new Connection(rpcUrl, 'confirmed');
export const getProvider = (rpcUrl: string) => new AnchorProvider(
  getConnection(rpcUrl),
  new Wallet(payer), {});

export const payer = keypair;