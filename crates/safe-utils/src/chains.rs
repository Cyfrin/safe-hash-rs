use crate::Result;

use alloy::primitives::ChainId;

const SAFE_SUPPORTED_CHAINS: &[(ChainId, &str)] = &[
    (42161, "arbitrum"),
    (1313161554, "aurora"),
    (43114, "avalanche"),
    (8453, "base"),
    (81457, "blast"),
    (56, "bsc"),
    (42220, "celo"),
    (1, "ethereum"),
    (100, "gnosis"),
    (59144, "linea"),
    (5000, "mantle"),
    (10, "optimism"),
    (137, "polygon"),
    (534352, "scroll"),
    (11155111, "sepolia"),
    (480, "worldchain"),
    (196, "xlayer"),
    (324, "zksync"),
    (84532, "base-sepolia"),
    (10200, "gnosis-chiado"),
    (1101, "polygon-zkevm"),
];

pub fn get_all_supported_chain_names() -> Vec<String> {
    SAFE_SUPPORTED_CHAINS.iter().map(|(_, chain_name)| chain_name.to_string()).collect()
}

pub trait Of<T> {
    fn of(chain_name: &str) -> Result<T>;
}

impl Of<ChainId> for ChainId {
    fn of(chain_name: &str) -> Result<ChainId> {
        SAFE_SUPPORTED_CHAINS
            .iter()
            .find(|&&(_, name)| name == chain_name)
            .map(|&(id, _)| id)
            .ok_or_else(|| format!("unsupported safe chain - {chain_name}").into())
    }
}
