# Local Safe Tx Verifier

> [!CAUTION]
> It is under development and is not fully tested. Please do not use it in production environments.

## Usage

There are typically 2 things we do in safe wallets. Signing messages and Executing transactions (when signer threshold is met).

This tool helps verify those actions locally and hopefully protect against possible phishing or compromised UI attacks. 

```bash
Usage: verifier [OPTIONS] --chain <CHAIN> --nonce <NONCE> --safe-contract <SAFE_CONTRACT> --tx-file <TX_FILE>

Options:
  -c, --chain <CHAIN>                  Chain - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea, mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia, gnosis-chiado, polygon-zkevm
  -n, --nonce <NONCE>                  Transaction nonce of the safe contract
  -s, --safe-contract <SAFE_CONTRACT>  Address of the safe contract
  -t, --tx-file <TX_FILE>              Path to JSON file containing the input from Tenderly's simulation summary
      --safe-version <SAFE_VERSION>    Safe Contract version [default: 1.3.0]
      --check-for-signing              Check for signing the transaction
      --check-for-executing            Check for executing the transaction
  -h, --help                           Print help
  -V, --version                        Print version
```

## Example 

```bash
./verifier -s 0x1111100000000000000000000000000011111111 -c arbitrum -n 1 -t tx.json --check-for-signing
```
Before signing the transaction in the ledger, make sure the safe tx hash matches based on the simulation input in tenderly.

## Roadmap

You can find a more detailed list in the [pinned GitHub issue](https://github.com/cyfrin/safe-tx-verifier/issues/1).

## Trust Assumptions
* Safe Smart contracts are flawless
* Tenderly simulation is flawless
* DNS is not hijacked
