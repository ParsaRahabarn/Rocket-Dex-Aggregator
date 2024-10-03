use crate::{domain::QuoteOutput, ToAddress, ToUint};

use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Bytes, H160, U256},
};
use std::env;
use std::{ops::Deref, sync::Arc};

pub async fn quick_v2_quote_single(
    token_in: H160,
    token_out: H160,
    amount_in: U256,
) -> Option<QuoteOutput> {
    abigen!(SushiV2Qotuter, "./interfaces/quick_v2.json");
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let quick_v2_qotuter: &str = "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff";
    // let quick_v2_factory = env::var("QUICKSWAP_FACTORY_V2").expect("RPC_URL must be set");
    let provider = Provider::<Http>::try_from(rpc_url).expect("invalid url");
    let client = Arc::new(provider);
    let address: H160 = quick_v2_qotuter.parse().expect("invalid checksum address");
    let contract: SushiV2Qotuter<Provider<Http>> = SushiV2Qotuter::new(address, client);
    let path = vec![token_in, token_out];
    let result: Result<Bytes, _> = contract
        .get_amounts_out(amount_in, path)
        .call_raw_bytes()
        .await;
    // let order = token_in.lt(&token_out);

    match result {
        Ok(res) => {
            // let pair = get_pair_v2(token_in, token_out, quick_v2_factory.to_address())
            //     .await
            //     .expect("couldn't get pair");

            let address_checksum = res.iter().skip(96).take(32);
            let amount = Bytes::from_iter(address_checksum);
            // let amounts = get_reserves_v2(pair, order)
            //     .await
            //     .expect("couldn't get reserves");
            // dbg!(amount);
            let slice_amount_out = amount.deref();

            let mut amount_out: u128 = 0;
            for &num in slice_amount_out.iter() {
                amount_out = (&amount_out << 8) | num as u128;
            }
            let result = QuoteOutput::new(
                token_in,
                H160::zero(),
                token_out,
                amount_in,
                U256::from(amount_out),
                "QuickV2".to_string(),
                "V2".to_string(),
                3000,
                0,
                0.to_uint().unwrap(),
                0.to_uint().unwrap(),
                0.to_uint().unwrap(),
                0.to_uint().unwrap(),
            );
            return Some(result);
        }
        Err(_) => {
            return None;
        }
    };

    // dbg!(res);
}

pub async fn quick_v2_middle_quote(
    token_in: H160,
    token_mid: H160,
    token_out: H160,
    amount_in: U256,
) -> Option<QuoteOutput> {
    abigen!(SushiV2Qotuter, "./interfaces/quick_v2.json");
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let quick_v2_qotuter: &str = "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff";
    // let quick_v2_factory = env::var("QUICKSWAP_FACTORY_V2").expect("RPC_URL must be set");
    let provider = Provider::<Http>::try_from(rpc_url).expect("invalid url");
    let client = Arc::new(provider);
    let address: H160 = quick_v2_qotuter.parse().expect("invalid checksum address");
    let contract: SushiV2Qotuter<Provider<Http>> = SushiV2Qotuter::new(address, client);
    let path = vec![token_in, token_out, token_mid];
    let result: Result<Bytes, _> = contract
        .get_amounts_out(amount_in, path)
        .call_raw_bytes()
        .await;

    match result {
        Ok(res) => {
            let address_checksum = res.iter().skip(128).take(32);
            let amount = Bytes::from_iter(address_checksum);
            // let pair1 = get_pair_v2(token_in, token_mid, quick_v2_factory.to_address())
            //     .await
            //     .expect("couldn't get reserves");
            // let order1 = token_in.lt(&token_mid);
            // let pair2 = get_pair_v2(token_mid, token_out, quick_v2_factory.to_address())
            //     .await
            //     .expect("couldn't get reserves");
            // let order2 = token_mid.lt(&token_out);
            let slice_amount_out = amount.deref();
            // let amounts_0 = get_reserves_v2(pair1, order1)
            //     .await
            //     .expect("couldn't get reserves");
            // let amounts_1 = get_reserves_v2(pair2, order2)
            //     .await
            //     .expect("couldn't get reserves");

            let mut amount_out: u128 = 0;
            for &num in slice_amount_out.iter() {
                amount_out = (&amount_out << 8) | num as u128;
            }
            let result = QuoteOutput::new(
                token_in,
                token_mid,
                token_out,
                amount_in,
                U256::from(amount_out),
                "QuickV2".to_string(),
                "V2".to_string(),
                3000,
                0,
                0.to_uint().unwrap(),
                0.to_uint().unwrap(),
                0.to_uint().unwrap(),
                0.to_uint().unwrap(),
            );
            return Some(result);
        }
        Err(_) => {
            return None;
        }
    };

    // dbg!(res);
}

pub async fn quick_v2_quote(
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

    let task_single = tokio::spawn(quick_v2_quote_single(token_in, token_out, amount_in));
    tasks.push(task_single);

    for mid_token in &tokens_vec {
        let task_middle = tokio::spawn(quick_v2_middle_quote(
            token_in,
            mid_token.to_address().expect("invalid checksum"),
            token_out,
            amount_in,
        ));

        tasks.push(task_middle);
    }

    return tasks;
}
