# Local Safe Tx Verifier

> [!CAUTION]
> It is under development and is not fully tested. Please do not use it in production environments.

## Usage

There are typically 2 things we do in safe wallets. Signing messages and Executing transactions (when signer threshold is met).

This tool helps verify those actions locally and hopefully protect against possible phishing or compromised UI attacks. 

```bash
Usage: safe-hash [OPTIONS] --chain <CHAIN> --safe-address <safe_address>

Options:
  -c, --chain <CHAIN>                  Chain - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea, mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia, gnosis-chiado, polygon-zkevm
  -n, --nonce <NONCE>                  Transaction nonce of the safe address
  -s, --safe-address <safe_address>    Address of the safe address
  -t, --tx-file <TX_FILE>             Path to JSON file containing the input from Tenderly's simulation summary
  -m, --message-file <MESSAGE_FILE>    Path to message file for offchain message hashes
      --safe-version <SAFE_VERSION>    Safe Contract version [default: 1.3.0]
      --tx-signing                     Check transaction signing (default mode if none specified)
      --tx-executing                   Check transaction execution
      --msg-signing                    Check message signing
  -h, --help                          Print help
  -V, --version                       Print version
```

## Example 

### Verify before signing transactions (default mode)

```bash
# These two commands are equivalent
./safe-hash -s 0xMultiSigAddress -c arbitrum -n 5 -t tx-file.json
./safe-hash -s 0xMultiSigAddress -c arbitrum -n 5 -t tx-file.json --tx-signing
```
Before signing the transaction in the ledger, make sure the safe tx hash matches 

### Verify before executing transactions

```bash
./safe-hash -t tx-file.json --tx-executing
```
Before signing to execute the transaction in the ledger, make sure the calldata matches 

### Verify message signing
```bash
./safe-hash -s 0xMultiSigAddress -c arbitrum -m message-file.txt --msg-signing
```

### NOTE

> tx-file.json comes from the input section in Tenderly simulation of the repsective action (sponsored by Safe)

## Roadmap

You can find a more detailed list in the [pinned GitHub issue](https://github.com/cyfrin/safe-tx-verifier/issues/1).

## Trust Assumptions
* Safe Smart contracts are flawless
* Tenderly simulation is flawless
* DNS is not hijacked
