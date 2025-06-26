import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

import wallet from "./lib/keypair.json"

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet))

const connection = new Connection("https://api.devnet.solana.com")

async function claimAirdrop() {
    try {
        const txHash = await connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL)
        console.log(txHash);
        
    } catch (error) {
        console.error("Something is wrong",error);
        
    }
}

claimAirdrop()