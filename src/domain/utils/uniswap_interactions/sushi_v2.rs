pub use crate::utils::traits::ToAddress;

use crate::{
    domain::{
        // utils::{get_pairs::get_pair_v2, get_reserves::get_reserves_v2},
        QuoteOutput,
    },
    ToUint,
};

use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Bytes, H160, U256},
};
use std::env;
use std::{ops::Deref, sync::Arc};
pub async fn sushi_quote_single(
    token_in: H160,
    token_out: H160,
    amount_in: U256,
) -> Option<QuoteOutput> {
    abigen!(SushiV2Quoter, "./interfaces/sushi_router_v2.json");
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let sushi_v2_quoter: &str = "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506";

    let provider = Provider::<Http>::try_from(rpc_url).expect("invalid url");
    let client = Arc::new(provider);
    let address: H160 = sushi_v2_quoter.parse().expect("invalid checksum address");
    let contract: SushiV2Quoter<Provider<Http>> = SushiV2Quoter::new(address, client);

    let path = vec![token_in, token_out];
    let result = contract
        .get_amounts_out(amount_in, path)
        .call_raw_bytes()
        .await;

    // let order = token_in.lt(&token_out);

    match result {
        Ok(res) => {
            let amount_checksum = res.iter().skip(96 + 32).take(32);
            let amount = Bytes::from_iter(amount_checksum);
            // dbg!(amount);
            let slice_amount_out = amount.deref();

            let mut amount_out: u128 = 0;
            for &num in slice_amount_out.iter() {
                amount_out = (&amount_out << 8) | num as u128;
            }
            // let pair = get_pair_v2(token_in, token_out, factory.to_address()).await?;

            // let amounts = get_reserves_v2(pair, order).await?;
            let result = QuoteOutput::new(
                token_in,
                H160::zero(),
                token_out,
                amount_in,
                U256::from(amount_out),
                "SUSHI_V2".to_string(),
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

pub async fn sushi_quote_middle(
    token_in: H160,
    token_mid: H160,
    token_out: H160,
    amount_in: U256,
) -> Option<QuoteOutput> {
    abigen!(SushiV2Quoter, "./interfaces/sushi_router_v2.json");
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let sushi_v2_quoter: &str = "0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506";

    let provider = Provider::<Http>::try_from(rpc_url).expect("invalid url");
    let client = Arc::new(provider);
    let address: H160 = sushi_v2_quoter.parse().expect("invalid checksum address");

    let contract: SushiV2Quoter<Provider<Http>> = SushiV2Quoter::new(address, client);
    let path = vec![token_in, token_mid, token_out];
    let result = contract
        .get_amounts_out(amount_in, path)
        .call_raw_bytes()
        .await;
    match result {
        Ok(res) => {
            let address_checksum = res.iter().skip(128).take(32);
            let amount = Bytes::from_iter(address_checksum);
            // dbg!(amount);
            let slice_amount_out = amount.deref();
            // let pair1 = get_pair_v2(token_in, token_mid, factory.to_address()).await?;
            // let order1 = token_in.lt(&token_mid);
            // let pair2 = get_pair_v2(token_mid, token_out, factory.to_address()).await?;
            // let order2 = token_mid.lt(&token_out);

            let mut amount_out: u128 = 0;
            for &num in slice_amount_out.iter() {
                amount_out = (&amount_out << 8) | num as u128;
            }
            // let amounts_0 = get_reserves_v2(pair1, order1).await?;
            // let amounts_1 = get_reserves_v2(pair2, order2).await?;

            let result = QuoteOutput::new(
                token_in,
                token_mid,
                token_out,
                amount_in,
                U256::from(amount_out),
                "SUSHI_V2".to_string(),
                "V2".to_string(),
                3000,
                3000,
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

pub async fn sushi_quote(
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
    let task_single = tokio::spawn(sushi_quote_single(token_in, token_out, amount_in));

    tasks.push(task_single);

    for mid_token in &tokens_vec {
        let task_middle = tokio::spawn(sushi_quote_middle(
            token_in,
            mid_token.to_address().expect("invalid checksum"),
            token_out,
            amount_in,
        ));

        tasks.push(task_middle);
    }

    return tasks;
}
