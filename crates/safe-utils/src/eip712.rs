use alloy::dyn_abi::TypedData;
use serde::Deserialize;

use crate::Result;

#[derive(Clone)]
pub struct Eip712Hasher {
    typed_message_string: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EIP7127HashDetails {
    pub eip_712_hash: String,
    pub domain_hash: String,
    pub message_hash: String,
}

impl Eip712Hasher {
    pub fn new(typed_message_string: String) -> Self {
        Self { typed_message_string }
    }

    pub fn hash(&self) -> Result<EIP7127HashDetails> {
        let typed_data: TypedData = serde_json::from_str(&self.typed_message_string)?;
        Ok(EIP7127HashDetails {
            eip_712_hash: typed_data.eip712_signing_hash()?.to_string(),
            domain_hash: typed_data.domain.hash_struct().to_string(),
            message_hash: typed_data.hash_struct()?.to_string(),
        })
    }
}
