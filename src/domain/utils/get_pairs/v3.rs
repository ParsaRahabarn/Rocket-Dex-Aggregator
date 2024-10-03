// use std::env;

// use ethers::{
//     prelude::abigen,
//     providers::{Http, Provider},
//     types::{Bytes, H160},
// };

// use std::sync::Arc;

// pub async fn get_pair_v3(
//     token_in: H160,
//     token_out: H160,
//     contract_address: H160,
//     fee: u32,
// ) -> Option<H160> {
//     abigen!(UniswapV3Quoter, "./interfaces/factory_v3.json");
//     let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");

//     let provider = Provider::<Http>::try_from(rpc_url).expect("invalid url");
//     let client = Arc::new(provider);
//     let address: H160 = contract_address;

//     let contract = UniswapV3Quoter::new(address, client);

//     let res: Result<Bytes, _> = contract
//         .get_pool(token_in, token_out, fee)
//         .call_raw_bytes()
//         .await;
//     match res {
//         Ok(address) => Some(H160::from_slice(address.to_vec().as_slice())),
//         _ => None,
//     }
// }
