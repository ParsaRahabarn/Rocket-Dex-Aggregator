use crate::domain::errors::DexParseError;
use crate::utils::serializer::*;
use ethers::types::{H160, U256};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]

pub struct QuoteOutput {
    pub token_in: H160,
    pub token_mid: H160,
    pub token_out: H160,
    #[serde(serialize_with = "u256_to_u128", deserialize_with = "u128_from_u256")]
    pub amount_in: U256,
    #[serde(serialize_with = "u256_to_u128", deserialize_with = "u128_from_u256")]
    pub amount_out: U256,
    pub dex_name: String,
    pub version: String,
    pub dex_fee1: u32,
    pub dex_fee2: u32,
    #[serde(serialize_with = "u256_to_u128", deserialize_with = "u128_from_u256")]
    pub in_liquidity: U256,
    #[serde(serialize_with = "u256_to_u128", deserialize_with = "u128_from_u256")]
    pub out_liquidity: U256,
    #[serde(serialize_with = "u256_to_u128", deserialize_with = "u128_from_u256")]
    pub in_to_mid_liquidity: U256,
    #[serde(serialize_with = "u256_to_u128", deserialize_with = "u128_from_u256")]
    pub mid_to_out_liquidity: U256,
}
impl QuoteOutput {
    pub fn new(
        token_in: H160,
        token_mid: H160,
        token_out: H160,
        amount_in: U256,
        amount_out: U256,
        dex_name: String,
        version: String,
        dex_fee1: u32,
        dex_fee2: u32,
        in_liquidity: U256,
        out_liquidity: U256,
        in_to_mid_liquidity: U256,
        mid_to_out_liquidity: U256,
    ) -> Self {
        Self {
            token_in,
            token_mid,
            token_out,
            amount_in,
            amount_out,
            dex_name,
            version,
            dex_fee1,
            dex_fee2,
            in_liquidity,
            out_liquidity,
            in_to_mid_liquidity,
            mid_to_out_liquidity,
        }
    }
}

pub enum Dexes {
    UniSwap,
    UNiswap,
    Sushi,
}

impl TryFrom<&str> for Dexes {
    type Error = DexParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "UniSwap" => Ok(Dexes::UniSwap),
            "UNiswap" => Ok(Dexes::UNiswap),
            "Sushi" => Ok(Dexes::Sushi),
            _ => Err(DexParseError::new(value)),
        }
    }
}

impl TryFrom<String> for Dexes {
    type Error = DexParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Reuse the implementation for &str
        Dexes::try_from(value.as_str())
    }
}
