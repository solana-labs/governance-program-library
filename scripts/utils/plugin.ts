import { Connection, Keypair, PublicKey, TransactionInstruction, TransactionMessage, VersionedTransaction } from '@solana/web3.js';

export const createAndSendV0Tx = async (connection: Connection, signer: Keypair, txInstructions: TransactionInstruction[]) => {
  const latestBlockhash = await connection.getLatestBlockhash('confirmed');
  const messageV0 = new TransactionMessage({
    payerKey: signer.publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions: txInstructions
  }).compileToV0Message();
  const transaction = new VersionedTransaction(messageV0);
  transaction.sign([signer]);
  const txid = await connection.sendTransaction(transaction, { maxRetries: 5 });
  console.log("Sent transaction", txid);
  await connection.confirmTransaction({ signature: txid, ...latestBlockhash }, 'confirmed');
  console.log("Confirmed transaction", txid);
  return txid;
};

export const getMaxVoterWeightRecord = async (
  realmPk: PublicKey,
  mint: PublicKey,
  clientProgramId: PublicKey
) => {
  const [
    maxVoterWeightRecord,
    maxVoterWeightRecordBump,
  ] = await PublicKey.findProgramAddress(
    [
      Buffer.from('max-voter-weight-record'),
      realmPk.toBuffer(),
      mint.toBuffer(),
    ],
    clientProgramId
  )
  return {
    maxVoterWeightRecord,
    maxVoterWeightRecordBump,
  }
}
