import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    sendAndConfirmTransaction,
    Transaction
  } from "@solana/web3.js";
  import wallet from "./lib/keypair.json";
  
  const from = Keypair.fromSecretKey(new Uint8Array(wallet));
  const to = new PublicKey("HuWUSzBMEfCphybY9CMgEHLRT5LWrvcQbeoYb2Y6M1Es");
  
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  
  async function transferSOL() {
    const balance = await connection.getBalance(from.publicKey);
  
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: 1 // Temporary value just to estimate fee
      })
    );
  
    transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    transaction.feePayer = from.publicKey;
  
    const message = transaction.compileMessage();
    const fee = (await connection.getFeeForMessage(message)).value || 0;
  
    if (balance <= fee) {
      console.error("Insufficient balance to cover transaction fee.");
      return;
    }
  
    const amountToSend = balance - fee;
  
    const finalTransaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: amountToSend
      })
    );
  
    finalTransaction.recentBlockhash = transaction.recentBlockhash;
    finalTransaction.feePayer = from.publicKey;
  
    const signature = await sendAndConfirmTransaction(connection, finalTransaction, [from]);
  
    console.log("Transaction signature:", signature);
  }
  
  transferSOL();
  