# Local Safe Tx Verifier

> [!CAUTION]
> It is under development and is not fully tested. Please do not use it in production environments.

## Usage

This tool helps protect against possible phishing or compromised UI attacks by allowing local verification of transaction hashes before signing.

```bash
Usage: safe-hash [OPTIONS] --chain <CHAIN> --nonce <NONCE> --safe-address <safe_address> --to <ADDRESS>

Options:
  -c, --chain <CHAIN>                  Chain - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea, mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia, gnosis-chiado, polygon-zkevm
  -n, --nonce <NONCE>                  Transaction nonce of the safe address
  -s, --safe-address <safe_address>    Address of the safe address
  -t, --to <ADDRESS>                   Address of the contract to send calldata to
  -d, --data <HEX>                     Raw calldata encoded in hex [default: "0x"]
      --value <AMOUNT>                 Value in wei to send [default: 0]
      --operation <0|1>                Call (0) or delegatecall (1) [default: 0]
      --safe-tx-gas <AMOUNT>           Gas limit for the safe transaction [default: 0]
      --base-gas <AMOUNT>              Base gas amount [default: 0]
      --gas-price <AMOUNT>             Gas price in wei [default: 0]
      --gas-token <ADDRESS>            Address of gas payment token [default: 0x0]
      --refund-receiver <ADDRESS>      Address to receive gas payment [default: 0x0]
      --safe-version <VERSION>        Safe Contract version [default: 1.3.0]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Examples

### Verify transaction before signing

```bash
# Basic transaction with just a target address and value
./safe-hash -s 0xSafeAddress -c ethereum -n 42 -t 0xTargetContract --value 1000000000000000000

# Transaction with calldata (e.g., token transfer)
./safe-hash -s 0xSafeAddress -c ethereum -n 42 -t 0xTokenContract -d 0xdatadatadata

# Transaction with custom gas parameters
./safe-hash -s 0xSafeAddress -c ethereum -n 42 -t 0xTargetContract --safe-tx-gas 100000 --base-gas 21000 --gas-price 50000000000
```

## Roadmap

You can find a more detailed list in the [pinned GitHub issue](https://github.com/cyfrin/safe-tx-verifier/issues/1).

## Trust Assumptions
* Safe Smart contracts are flawless
* Tenderly simulation is flawless
* DNS is not hijacked
