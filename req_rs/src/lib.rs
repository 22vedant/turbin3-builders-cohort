#[allow(unused)]
#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::nonblocking::rpc_client;
    use solana_sdk::{
        hash::hash,
        instruction::AccountMeta,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    };
    use std::io::{self, BufRead};
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You have created a new Solana Wallet {}",
            kp.pubkey().to_string()
        );
        println!("{:?}", kp.to_bytes())
    }

    #[test]

    fn base58_to_json_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    // #[test]
    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");

        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() {
        use solana_client::rpc_client::RpcClient;
        use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
        let RPC_URL = "https://api.devnet.solana.com";

        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        let client = RpcClient::new(RPC_URL);
        println!("{}", &keypair.pubkey());

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }

            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }
    #[test]
    fn transfer_sol() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;
        let RPC_URL = "https://api.devnet.solana.com";

        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());

        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        let to_pubkey = Pubkey::from_str("HuWUSzBMEfCphybY9CMgEHLRT5LWrvcQbeoYb2Y6M1Es").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
    #[test]
    fn transfer_all() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            message::Message,
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };
        use std::str::FromStr;
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let pubkey = keypair.pubkey();

        let rpc_url = "https://api.devnet.solana.com";
        let client = RpcClient::new(rpc_url.to_string());

        let balance = client.get_balance(&pubkey).expect("failed to get balance");
        let to_pubkey = Pubkey::from_str("HuWUSzBMEfCphybY9CMgEHLRT5LWrvcQbeoYb2Y6M1Es").unwrap();
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");

        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn mint_nft() {
        use solana_client::rpc_client::RpcClient;
        use solana_program::{pubkey::Pubkey, system_instruction::transfer};
        use solana_sdk::{
            hash::hash,
            instruction::{AccountMeta, Instruction},
            // pubkey::Pubkey,
            signature::{Keypair, Signer, read_keypair_file},
            transaction::Transaction,
        };

        use std::str::FromStr;

        let rpc_url = "https://api.devnet.solana.com";
        let client = RpcClient::new(rpc_url.to_string());

        let signer = read_keypair_file("secret.json").expect("Couldn't find wallet");
        let signer_pubkey = signer.pubkey();

        let mint = Keypair::new();

        let turbin3_prereq_program =
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();

        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();

        let mpl_core_program =
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();

        let system_program_id = Pubkey::from_str("11111111111111111111111111111111").unwrap();

        let account_seeds = &[b"prereqs", signer_pubkey.as_ref()];

        let (account_key, _account_bump) =
            Pubkey::find_program_address(account_seeds, &turbin3_prereq_program);

        let authority_seeds = &[b"collection", collection.as_ref()];

        let (authority_key, _authority_bump) =
            Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        let accounts = vec![
            AccountMeta::new(signer_pubkey, true), // Signer
            AccountMeta::new(account_key, false),  // The PDA derived above
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(collection, false), // Example account
            AccountMeta::new_readonly(authority_key, false),
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program_id, false),
        ];

        let blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );

        let signature = client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
