use crate::Result;
use eip712_enc::{EIP712, hash_structured_data};
use serde_json::from_str;

pub struct Eip712Hasher {
    typed_message_string: String,
}

impl Eip712Hasher {
    fn new(typed_message_string: String) -> Self {
        Self { typed_message_string }
    }

    fn hash(&self) -> Result<String> {
        let hased_data = hash_structured_data(from_str::<EIP712>(&self.typed_message_string)?)
            .map_err(|_| "Failed to hash".to_string())?;
        let hashed_string = &format!("{:x}", hased_data)[..];
        Ok(hashed_string.to_owned())
    }
}

#[cfg(test)]
mod eip712_hash {
    use crate::Eip712Hasher;

    #[test]
    fn test_encoding_1() {
        let string = r#"{
            "types": {
                "EIP712Domain": [
                    {
                      "name": "name",
                      "type": "string"
                    },
                    {
                      "name": "version",
                      "type": "string"
                    },
                    {
                      "name": "chainId",
                      "type": "uint256"
                    },
                    {
                      "name": "verifyingContract",
                      "type": "address"
                    }
                ],
                "Person": [
                    {
                      "name": "name",
                      "type": "string"
                    },
                    {
                      "name": "wallets",
                      "type": "address[]"
                    }
                ],
                "Mail": [
                    {
                      "name": "from",
                      "type": "Person"
                    },
                    {
                      "name": "to",
                      "type": "Person[]"
                    },
                    {
                      "name": "contents",
                      "type": "string"
                    }
                ],
                "Group": [
                    {
                      "name": "name",
                      "type": "string"
                    },
                    {
                      "name": "members",
                      "type": "Person[]"
                    }
                ]
            },
            "domain": {
                "name": "Ether Mail",
                "version": "1",
                "chainId": "0x1",
                "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
            },
            "primaryType": "Mail",
            "message": {
                "from": {
                    "name": "Cow",
                    "wallets": [
                      "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826",
                      "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF"
                    ]
                },
                "to": [
                    {
                        "name": "Bob",
                        "wallets": [
                            "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
                            "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57",
                            "0xB0B0b0b0b0b0B000000000000000000000000000"
                        ]
                    }
                ],
                "contents": "Hello, Bob!"
            }
        }"#;

        let hasher = Eip712Hasher::new(string.to_string());
        let hash = hasher.hash().expect("failed to create eip 712 hash");

        assert_eq!(hash, "a85c2e2b118698e88db68a8105b794a8cc7cec074e89ef991cb4f5f533819cc2");
    }

    #[test]
    fn test_encoding_2() {
        let string = r#"{
            "types": {
                "EIP712Domain": [
                    {
                        "name": "name",
                        "type": "string"
                    },
                    {
                        "name": "version",
                        "type": "string"
                    },
                    {
                        "name": "chainId",
                        "type": "uint256"
                    },
                    {
                        "name": "verifyingContract",
                        "type": "address"
                    }
                ],
              "Person": [
                {
                  "name": "name",
                  "type": "string"
                },
                {
                  "name": "wallets",
                  "type": "address[]"
                }
              ],
              "Mail": [
                {
                  "name": "from",
                  "type": "Person"
                },
                {
                  "name": "to",
                  "type": "Group"
                },
                {
                  "name": "contents",
                  "type": "string"
                }
              ],
              "Group": [
                {
                  "name": "name",
                  "type": "string"
                },
                {
                  "name": "members",
                  "type": "Person[]"
                }
              ]
            },
            "domain": {
              "name": "Ether Mail",
              "version": "1",
              "chainId": "0x1",
              "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
            },
            "primaryType": "Mail",
            "message": {
              "from": {
                "name": "Cow",
                "wallets": [
                  "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826",
                  "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF"
                ]
              },
              "to": {
                "name": "Farmers",
                "members": [
                  {
                    "name": "Bob",
                    "wallets": [
                      "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
                      "0xB0BdaBea57B0BDABeA57b0bdABEA57b0BDabEa57",
                      "0xB0B0b0b0b0b0B000000000000000000000000000"
                    ]
                  }
                ]
              },
              "contents": "Hello, Bob!"
            }
          }"#;

        let hasher = Eip712Hasher::new(string.to_string());
        let hash = hasher.hash().expect("failed to create eip 712 hash");
        assert_eq!(hash, "cd8b34cd09c541cfc0a2fcd147e47809b98b335649c2aa700db0b0c4501a02a0");
    }
}
