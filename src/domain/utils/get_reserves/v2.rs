// use std::env;

// use ethers::{
//     prelude::abigen,
//     providers::{Http, Provider},
//     types::{Bytes, H160, U128},
// };

// use std::sync::Arc;

// pub async fn get_reserves_v2(contract_address: H160, order: bool) -> Option<(u128, u128)> {
//     abigen!(UniswapV3Quoter, "./interfaces/pair_v2.json");
//     let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");

//     let provider = Provider::<Http>::try_from(rpc_url).expect("invalid url");
//     let client = Arc::new(provider);
//     let address: H160 = contract_address;

//     let contract = UniswapV3Quoter::new(address, client);

//     let res: Result<Bytes, _> = contract.get_reserves().call_raw_bytes().await;
//     match res {
//         Ok(data) => {
//             println!("{:?}", data);
//             return Some((0, 0));
//             // let reserve0_bytes: &[u8] = &data[0..14];
//             // let reserve0 = U128::from_big_endian(reserve0_bytes).as_u128();

//             // // Extract _reserve1 (next 14 bytes, which is 112 bits)
//             // let reserve1_bytes: &[u8] = &data[14..28];
//             // let reserve1 = U128::from_big_endian(reserve1_bytes).as_u128();
//             // match order {
//             //     true => Some((reserve0, reserve1)),
//             //     false => Some((reserve1, reserve0)),
//             // }
//         }
//         _ => None,
//     }
// }
