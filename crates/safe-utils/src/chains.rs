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
    (43111, "hemi"),
];

const SAFE_CHAIN_APIS: &[(&str, &str)] = &[
    ("arbitrum", "https://safe-transaction-arbitrum.safe.global"),
    ("aurora", "https://safe-transaction-aurora.safe.global"),
    ("avalanche", "https://safe-transaction-avalanche.safe.global"),
    ("base", "https://safe-transaction-base.safe.global"),
    ("blast", "https://safe-transaction-blast.safe.global"),
    ("bsc", "https://safe-transaction-bsc.safe.global"),
    ("celo", "https://safe-transaction-celo.safe.global"),
    ("ethereum", "https://safe-transaction-mainnet.safe.global"),
    ("gnosis", "https://safe-transaction-gnosis-chain.safe.global"),
    ("linea", "https://safe-transaction-linea.safe.global"),
    ("mantle", "https://safe-transaction-mantle.safe.global"),
    ("optimism", "https://safe-transaction-optimism.safe.global"),
    ("polygon", "https://safe-transaction-polygon.safe.global"),
    ("scroll", "https://safe-transaction-scroll.safe.global"),
    ("sepolia", "https://safe-transaction-sepolia.safe.global"),
    ("worldchain", "https://safe-transaction-worldchain.safe.global"),
    ("xlayer", "https://safe-transaction-xlayer.safe.global"),
    ("zksync", "https://safe-transaction-zksync.safe.global"),
    ("base-sepolia", "https://safe-transaction-base-sepolia.safe.global"),
    ("gnosis-chiado", "https://safe-transaction-chiado.safe.global"),
    ("polygon-zkevm", "https://safe-transaction-zkevm.safe.global"),
    ("hemi", "https://safe-transaction-hemi.safe.global"),
];

pub fn get_all_supported_chain_names() -> Vec<String> {
    SAFE_SUPPORTED_CHAINS.iter().map(|(_, chain_name)| chain_name.to_string()).collect()
}

pub fn get_safe_api(chain_id: ChainId) -> Result<String> {
    let chain_name = {
        let chain_names =
            SAFE_SUPPORTED_CHAINS
                .into_iter()
                .filter_map(|(_chain_id, chain_name)| {
                    if *_chain_id == chain_id { Some(chain_name.to_string()) } else { None }
                })
                .collect::<Vec<_>>();
        chain_names.get(0).ok_or("no chain found")?.to_string()
    };

    let api = {
        let apis =
            SAFE_CHAIN_APIS
                .into_iter()
                .filter_map(|(_chain_name, chain_api)| {
                    if _chain_name == &chain_name { Some(chain_api.to_string()) } else { None }
                })
                .collect::<Vec<_>>();
        apis.get(0).ok_or("chain not found")?.to_string()
    };

    Ok(api)
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
