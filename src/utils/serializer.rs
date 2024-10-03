use ethers::types::U256;
use serde::{self, Deserialize, Deserializer, Serializer};
pub fn u256_to_u128<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let as_u128 = value.low_u128();
    serializer.serialize_u128(as_u128)
}

pub fn u128_from_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let as_u128 = u128::deserialize(deserializer)?;
    Ok(U256::from(as_u128))
}
