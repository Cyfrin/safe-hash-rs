# safe-hash - Verify Safe Wallet Transactions and Messages

> [!CAUTION]
> It is under development and is not fully tested. Please do not use it in production environments.

## Installation 

### Cyfrinup

**Cyfrinup** simplifies the installation and management of Cyfrin tools.

Follow the instructions to install [here](https://github.com/Cyfrin/up).

Run `safe-hash --version` to check the installation.


### Shell (For Linux, Mac, WSL)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Cyfrin/safe-hash-rs/releases/latest/download/safe-hash-installer.sh | sh
```

### Powershell (For Windows)

```
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Cyfrin/safe-hash-rs/releases/latest/download/safe-hash-installer.ps1 | iex"
```

### Homebrew 

```
brew install cyfrin/tap/safe-hash
```

### npm

```
npm install -g @cyfrin/safe-hash
```

## Usage

This tool helps protect against possible phishing or compromised UI attacks by allowing local verification of transaction and message hashes before signing.

```bash
Usage: safe-hash [OPTIONS] <COMMAND>

Commands:
  tx  Transaction signing mode
  msg Message signing mode

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Transaction Signing Mode

```bash
Usage: safe-hash tx [OPTIONS] --chain <CHAIN> --nonce <NONCE> --safe-address <safe_address> --to <ADDRESS>

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
```

### Message Signing Mode

```bash
Usage: safe-hash msg [OPTIONS] --chain <CHAIN> --safe-address <safe_address> --input-file <FILE>

Options:
  -c, --chain <CHAIN>                  Chain - arbitrum, aurora, avalanche, base, blast, bsc, celo, ethereum, gnosis, linea, mantle, optimism, polygon, scroll, sepolia, worldchain, xlayer, zksync, base-sepolia, gnosis-chiado, polygon-zkevm
  -s, --safe-address <safe_address>    Address of the safe address
  -i, --input-file <FILE>             Path to the message file to be signed
      --safe-version <VERSION>        Safe Contract version [default: 1.3.0]
```

## Examples

### Transaction Signing Examples

```bash
# Basic transaction with just a target address and value
safe-hash tx -s 0xSafeAddress -c ethereum -n 42 -t 0xTargetContract --value 1000000000000000000

# Transaction with calldata (e.g., token transfer)
safe-hash tx -s 0xSafeAddress -c ethereum -n 42 -t 0xTokenContract -d 0xdatadatadata

# Transaction with custom gas parameters
safe-hash tx -s 0xSafeAddress -c ethereum -n 42 -t 0xTargetContract --safe-tx-gas 100000 --base-gas 21000 --gas-price 50000000000

# Real-world example
safe-hash tx \
  --chain ethereum \
  --nonce 63 \
  --safe-address 0x1c694Fc3006D81ff4a56F97E1b99529066a23725 \
  --to 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \
  --data 0xa9059cbb00000000000000000000000092d0ebaf7eb707f0650f9471e61348f4656c29bc00000000000000000000000000000000000000000000000000000005d21dba00
```

Expected output:
```
+-----------------------+------------------------------------------------------------------+
| TYPE                  | CALCULATED HASHES                                                |
+-----------------------+------------------------------------------------------------------+
| Domain Hash           | 1655e94a9bcc5a957daa1acae692b4c22e7aaf146b4deb9194f8221d2f09d8c3 |
| Message Hash          | f22754eba5a2b230714534b4657195268f00dc0031296de4b835d82e7aa1e574 |
| Safe Transaction Hash | ad06b099fca34e51e4886643d95d9a19ace2cd024065efb66662a876e8c40343 |
+-----------------------+------------------------------------------------------------------+
```

### Message Signing Examples

```bash
# Sign a message from a file
safe-hash msg \
  --chain sepolia \
  --safe-address 0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1 \
  --input-file test/test_message.txt
```

Expected output:
```
+-----------------------+------------------------------------------------------------------+
| TYPE                  | CALCULATED HASHES                                                |
+-----------------------+------------------------------------------------------------------+
| Raw Message Hash      | cb1a9208c1a7c191185938c7d304ed01db68677eea4e689d688469aa72e34236 |
| Domain Hash           | 611379c19940caee095cdb12bebe6a9fa9abb74cdb1fbd7377c49a1f198dc24f |
| Message Hash          | a5d2f507a16279357446768db4bd47a03bca0b6acac4632a4c2c96af20d6f6e5 |
| Safe Transaction Hash | 1866b559f56261ada63528391b93a1fe8e2e33baf7cace94fc6b42202d16ea08 |
+-----------------------+------------------------------------------------------------------+
```

## Roadmap

You can find a more detailed list in the [pinned GitHub issue](https://github.com/cyfrin/safe-tx-verifier/issues/1).

## Trust Assumptions
* You trust this codebase
* You trust your hardware wallet
* You trust the Safe Wallet contracts
