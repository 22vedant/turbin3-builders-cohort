import { Connection, PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import idl from "./lib/turbin3-idl.json"
import wallet from "./lib/secret-solflare.json"

const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const user = keypair.publicKey
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed"
})

const program = new Program(idl, provider)

const account_seeds = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer()
]

const [account_key, _account_bump] =
PublicKey.findProgramAddressSync(account_seeds, program.programId);

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");

const authority_seeds = [
    Buffer.from("collection"),
    mintCollection.toBuffer()
]

const [authority_key, _authority_bump] =
PublicKey.findProgramAddressSync(authority_seeds, program.programId);


const mintTs = Keypair.generate()

const initialize = async ({github}: {github: string}) => {
    try {
        const tx = await program
        .methods
        .initialize(github)
        .accounts({
            user: user,
            account: account_key,
            systemProgram: SystemProgram.programId
        })
        .signers([keypair])
        .rpc()

        console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${tx}?cluster=devnet`);
        return tx
    } catch (error) {
        console.error(error);
        throw error
    }
}

const submit_ts = async () => {
    try {
        const tx = await program.methods
        .submitTs()
        .accounts({
            user: user,
            account: account_key,
            mint: mintTs.publicKey,
            collection: mintCollection,
            authority: authority_key,
            mpl_core_program: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId
        })
        .signers([keypair, mintTs])
        .rpc()
        console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${tx}?cluster=devnet`);
        return tx
    } catch (error) {
        console.error(error);
    }
}

const main = async () => {
    try {
        await initialize({github: "22vedant"});
        await submit_ts()
    } catch (error) {
        console.error("Main execution error:", error);
    }
}

main()