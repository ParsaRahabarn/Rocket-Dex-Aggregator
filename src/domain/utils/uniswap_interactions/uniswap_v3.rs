use crate::domain::states::QuoteOutput;

pub use crate::utils::traits::ToAddress;

use std::env;

use ethers::{
    abi::{Address, Token},
    core::abi::encode_packed,
    prelude::abigen,
    providers::{Http, Provider, ProviderError},
    types::{Bytes, H160, U256},
};

use std::{ops::Deref, sync::Arc};
const FEES: [u32; 4] = [100, 500, 3000, 10000];
pub async fn uniswap_v3_quoter(
    token_in: H160,
    mid_token: H160,
    token_out: H160,
    fee1: u32,
    fee2: u32,
    amount_in: U256,
) -> Option<QuoteOutput> {
    abigen!(UniswapV3Quoter, "./interfaces/uniswap_v3_quoter.json");
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let uniswap_v3_quoter: &str = "0x61fFE014bA17989E743c5F6cB21bF9697530B21e";

    let provider = Provider::<Http>::try_from(rpc_url).expect("invalid url");
    let client = Arc::new(provider);
    let address: H160 = uniswap_v3_quoter.parse().expect("invalid checksum address");
    let encoded_fee_1: [u8; 4] = fee1.to_be_bytes();
    let encoded_fee_2: [u8; 4] = fee2.to_be_bytes();
    let truncated_encoded_fee_1 = &encoded_fee_1[1..4].to_vec();
    let truncated_encoded_fee_2 = &encoded_fee_2[1..4].to_vec();

    let contract = UniswapV3Quoter::new(address, client);
    let token_input = Token::Address(token_in);
    let token_middle = Token::Address(mid_token);
    let token_output = Token::Address(token_out);
    let fee_1 = Token::Bytes(truncated_encoded_fee_1.to_owned());
    let fee_2 = Token::Bytes(truncated_encoded_fee_2.to_owned());
    let tokens = &[token_input, fee_1, token_middle, fee_2, token_output];
    let packed_data = encode_packed(tokens).expect("invalid argument packed");
    let path = Bytes::from(packed_data);
    let res: Result<Bytes, _> = contract
        .quote_exact_input(path, amount_in)
        .call_raw_bytes()
        .await;
    match res {
        Ok(response) => {
            let response_iter = response.iter().take(32).cloned().collect::<Vec<u8>>();
            let amount = Bytes::from_iter(response_iter);

            let slice_amount_out = amount.deref();

            let mut amount_out: u128 = 0;
            for &num in slice_amount_out.iter() {
                amount_out = (&amount_out << 8) | num as u128;
            }
            // 0.00017169
            let quote = QuoteOutput::new(
                token_in,
                mid_token,
                token_out,
                amount_in,
                U256::from(amount_out),
                "UNISwapV3".to_string(),
                "V3".to_string(),
                fee1,
                fee2,
                U256::zero(),
                U256::zero(),
                U256::zero(),
                U256::zero(),
            );
            Some(quote)
        }

        _ => None,
    }
}
pub async fn uniswap_v3_quoter_single(
    token_in: H160,
    token_out: H160,
    amount_in: U256,
    fee: u32,
) -> Option<QuoteOutput> {
    abigen!(UniswapV3Quoter, "./interfaces/uniswap_v3_quoter.json");
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let uniswap_v3_quoter: &str = "0x61fFE014bA17989E743c5F6cB21bF9697530B21e";

    let provider = Provider::<Http>::try_from(rpc_url).expect("invalid url");
    let client = Arc::new(provider);
    let address: H160 = uniswap_v3_quoter.parse().expect("invalid checksum address");
    let contract = UniswapV3Quoter::new(address, client);

    impl QuoteExactInputSingleParams {
        fn new(
            token_in: H160,
            token_out: H160,
            amount_in: U256,
            fee: u32,
            sqrt_price_limit_x96: U256,
        ) -> Self {
            Self {
                token_in,
                token_out,
                amount_in,
                fee,
                sqrt_price_limit_x96,
            }
        }
    }
    let sqrt_price_limit_x96 = U256::default();

    let params =
        QuoteExactInputSingleParams::new(token_in, token_out, amount_in, fee, sqrt_price_limit_x96);
    let result: Result<Bytes, ProviderError> = contract
        .quote_exact_input_single(params)
        .call_raw_bytes()
        .await;
    let mut exact_amount_out: u128 = 0;
    match result {
        Ok(res) => {
            let address_checksum = res.iter().take(32).cloned().collect::<Vec<u8>>();
            let amount = Bytes::from_iter(address_checksum);

            let slice_amount_out = amount.deref();

            let mut amount_out: u128 = 0;
            for &num in slice_amount_out.iter() {
                amount_out = (&amount_out << 8) | num as u128;
            }
            exact_amount_out = amount_out;
        }
        Err(_) => {}
    };

    match exact_amount_out {
        0 => None,
        _ => {
            let quote = QuoteOutput::new(
                token_in,
                Address::zero(),
                token_out,
                amount_in,
                U256::from(exact_amount_out),
                "UNISwapV3".to_string(),
                "V3".to_string(),
                fee,
                0,
                U256::zero(),
                U256::zero(),
                U256::zero(),
                U256::zero(),
            );
            Some(quote)
        }
    }
}
pub async fn get_uniswap_v3(
    token_in: H160,
    token_out: H160,
    amount_in: U256,
) -> Vec<tokio::task::JoinHandle<Option<QuoteOutput>>> {
    let middleware_tokens: String =
        env::var("MIDDLE_WARE_TOKENS").expect("MIDDLE_WARE_TOKENS must be set");

    let tokens_vec: Vec<String> = middleware_tokens
        .split(',')
        .map(|s| s.to_string())
        .collect();

    let mut tasks: Vec<tokio::task::JoinHandle<Option<QuoteOutput>>> = vec![];
    for fee in FEES {
        let task_single = tokio::spawn(uniswap_v3_quoter_single(
            token_in, token_out, amount_in, fee,
        ));
        tasks.push(task_single);

        for mid_token in &tokens_vec {
            for fee_1 in FEES {
                let task_middle = tokio::spawn(uniswap_v3_quoter(
                    token_in,
                    mid_token.to_address().expect("invalid checksum"),
                    token_out,
                    fee,
                    fee_1,
                    amount_in,
                ));
                tasks.push(task_middle);
            }
        }
    }

    return tasks;
}
// fn get_data(token_in: H160, token_out: H160, amount_in: U256) {
//     // Fetching environment variable
//     let middleware_tokens: String =
//         env::var("MIDDLE_WARE_TOKENS").expect("MIDDLE_WARE_TOKENS must be set");

//     // Split the middleware tokens into a vector
//     let tokens_vec: Vec<String> = middleware_tokens
//         .split(',')
//         .map(|s| s.to_string())
//         .collect();

//     // Create the channel for sending and receiving `Option<QuoteOutput>`
//     let (tx, rx): (Sender<Option<QuoteOutput>>, Receiver<Option<QuoteOutput>>) = mpsc::channel();
//     let mut children = Vec::new();

//     // Loop over the fees and spawn threads
//     for fee in FEES {
//         let thread_tx = tx.clone();
//         let token_in = token_in.clone(); // Cloning tokens for thread safety
//         let token_out = token_out.clone(); // Cloning tokens for thread safety
//         let amount_in = amount_in.clone(); // Cloning the amount for thread safety
//         let tokens_vec = tokens_vec.clone(); // Cloning the vector for thread safety

//         // Spawn a new thread
//         let child = thread::spawn(move || {
//             // Create a new Tokio runtime within each thread
//             let rt = Runtime::new().unwrap();

//             // Run the async function inside the runtime
//             rt.block_on(async {
//                 // Call `uniswap_v3_quoter_single` and send the result
//                 let result = uniswap_v3_quoter_single(token_in, token_out, amount_in, fee).await;

//                 if let Err(e) = thread_tx.send(result) {
//                     eprintln!("Failed to send result: {}", e);
//                 }

//                 // Vector to hold the middle token tasks
//                 let mut tasks = Vec::new();

//                 // Process `mid_token` and its corresponding fees
//                 for mid_token in &tokens_vec {
//                     for fee_1 in FEES {
//                         // Spawn the async task for the middle token processing
//                         let task_middle = tokio::spawn(uniswap_v3_quoter(
//                             token_in,
//                             mid_token.to_address(),
//                             token_out,
//                             fee,
//                             fee_1,
//                             amount_in,
//                         ));

//                         // Add the task to the `tasks` vector
//                         tasks.push(task_middle);
//                     }
//                 }

//                 // Await all the tasks for middle token processing
//                 for task in tasks {
//                     if let Err(e) = task.await {
//                         eprintln!("Error in task: {}", e);
//                     }
//                 }
//             });
//         });

//         // Store the thread handle to join later
//         children.push(child);
//     }

//     // Drop the original sender to close the channel
//     drop(tx);

//     // Collect the results from the receiver
//     let mut quotes: Vec<Option<QuoteOutput>> = Vec::new();
//     for _ in 0..FEES.len() {
//         match rx.recv() {
//             Ok(quote) => quotes.push(quote),
//             Err(e) => eprintln!("Failed to receive result: {}", e),
//         }
//     }

//     // Join all the threads to ensure they complete
//     for child in children {
//         child.join().expect("Thread panicked");
//     }

//     // Now `quotes` contains all the results from `uniswap_v3_quoter_single`
//     println!("{:?}", quotes);
// }
