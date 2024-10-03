pub use crate::domain::uniswap_interactions;
use crate::QuoteOutput;
use ethers::types::{H160, U256};
use futures::future::join_all;
pub async fn get_all_quotes(token_in: H160, token_out: H160, amount_in: U256) -> Vec<QuoteOutput> {
    let mut results: Vec<QuoteOutput> = vec![];
    let mut tasks: Vec<tokio::task::JoinHandle<Option<QuoteOutput>>> = vec![];
    let swap_task: &mut Vec<tokio::task::JoinHandle<Option<QuoteOutput>>> =
        &mut uniswap_interactions::get_uniswap_v3(token_in, token_out, amount_in).await;
    let quick_task: &mut Vec<tokio::task::JoinHandle<Option<QuoteOutput>>> =
        &mut uniswap_interactions::quick_v2_quote(token_in, token_out, amount_in).await;
    let sushi_task: &mut Vec<tokio::task::JoinHandle<Option<QuoteOutput>>> =
        &mut uniswap_interactions::sushi_quote(token_in, token_out, amount_in).await;
    tasks.append(swap_task);
    tasks.append(quick_task);
    tasks.append(sushi_task);

    let res: Vec<Option<QuoteOutput>> = join_all(tasks)
        .await
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();
    for i in res {
        if let Some(data) = i {
            results.push(data);
        }
    }

    return results;
}
