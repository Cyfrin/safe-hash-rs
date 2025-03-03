use alloy::primitives::ChainId;
use reqwest::blocking::Client;
use std::env;

pub fn is_contract_verfied(
    address: &str,
    chain_id: ChainId,
) -> Result<bool, Box<dyn std::error::Error>> {
    let api_key = env::var("ETHERSCAN_API_KEY")?;
    let url = format!(
        "https://api.etherscan.io/v2/api?chainid={}&module=contract&action=getsourcecode&address={}&apikey={}",
        chain_id, address, api_key
    );

    let client = Client::new();
    let response = client.get(&url).send()?.json::<serde_json::Value>()?;

    let results = response.get("result").ok_or("bad reponse")?.as_array().ok_or("bad response")?;
    let entry = results.get(0).ok_or("bad response")?;

    let source_code =
        entry.get("SourceCode").ok_or("bad response")?.as_str().ok_or("bad response")?;

    Ok(!source_code.is_empty())
}

#[cfg(test)]
mod check_contract_verification {

    use std::convert::identity;

    use super::*;
    use safe_utils::Of;

    #[test]
    #[ignore = "repetitively running test can cause API Key to be blacklisted"]
    fn test_veridied_contracts() {
        // DOGECOIN (Ethereum)
        assert!(
            is_contract_verfied("0xc336f8408557272646d192628dc3f554b654b21a", 1)
                .is_ok_and(identity)
        );
        // USDC (Arbitrum)
        assert!(
            is_contract_verfied(
                "0xaf88d065e77c8cC2239327C5EDb3A432268e5831",
                ChainId::of("arbitrum").unwrap()
            )
            .is_ok_and(identity)
        );
    }
}
