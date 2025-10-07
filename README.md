# safe-hash - Verify Safe Wallet Transactions and Messages

A tool to verify Safe{Wallet} transaction data and EIP-712 messages before signing.

- [safe-hash - Verify Safe Wallet Transactions and Messages](#safe-hash---verify-safe-wallet-transactions-and-messages)
- [Security practices for using `safe-hash`](#security-practices-for-using-safe-hash)
- [Getting Started](#getting-started)
  - [Installation](#installation)
    - [Cyfrinup](#cyfrinup)
    - [Shell (For Linux, Mac, WSL)](#shell-for-linux-mac-wsl)
    - [Homebrew](#homebrew)
    - [npm](#npm)
- [Usage](#usage)
  - [Help](#help)
  - [Live Examples](#live-examples)
    - [Transaction Signing](#transaction-signing)
    - [Transaction Signing With Nested Safe Address, offline mode](#transaction-signing-with-nested-safe-address-offline-mode)
    - [Message Signing](#message-signing)
    - [EIP-712 encoding](#eip-712-encoding)
    - [Example outputs](#example-outputs)
  - [Trust Assumptions](#trust-assumptions)
- [Community-Maintained User Interface Implementations](#community-maintained-user-interface-implementations)
- [Acknowledgements](#acknowledgements)


# Security practices for using `safe-hash`

1. **Use a separate device for running this script**, totally different from what you use to send/sign your Safe{Wallet} transactions. This is to add some resilience in case your main device is compromised.
   1. For extra security, run this inside a docker container or a secure operating system like Tails or Qubes OS. **This project includes Podman/Docker support for enhanced security isolation.**
2. **Manually verify what you expect, and then compare to what you get**. This tool shows you what you get from you wallet based on your input, you should compare it to what you expect to get
3. **Understand the parameters you're signing**. This tool is useless if you don't know how to read the output! To get familiar, you can play the [wise-signer](https://wise-signer.cyfrin.io/) game to learn how calldata should be interpreted (with this tool too).
4. **Read/watch these resources**:
   1. [Video: Verifying Safe Wallet transactions](https://updraft.cyfrin.io/courses/advanced-web3-wallet-security/signer-advanced/verify-multi-sig-signatures)
   2. [How to perform basic transaction checks](https://help.safe.global/en/articles/276343-how-to-perform-basic-transactions-checks-on-safe-wallet)
   3. [How to verify a Safe TX on a hardware wallet](https://help.safe.global/en/articles/276344-how-to-verify-safe-wallet-transactions-on-a-hardware-wallet)
5. **Understand the trust assumptions**. Even using this tool has trust assumptions, no matter what tool you use, be sure you understand the differences in trust. 

# Getting Started

## Installation

You can install this tool in several ways.

### Cyfrinup

This method simplifies the installation and management of Cyfrin tools.

Instructions [here](https://github.com/Cyfrin/up).

Run `safe-hash --version` to check the installation.


### Shell (For Linux, Mac, WSL)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Cyfrin/safe-hash-rs/releases/latest/download/safe-hash-installer.sh | sh
```

### Homebrew 

```
brew install cyfrin/tap/safe-hash
```

### npm

```
npm install -g @cyfrin/safe-hash
```

### Podman/Docker

For enhanced security, you can run `safe-hash` in a container using Podman or Docker. This provides additional isolation and is especially recommended when running on potentially compromised systems.

#### Quick Start with Podman

1. Build the container image:
```bash
./scripts/podman-build.sh
```

2. Run safe-hash in the container:
```bash
# Show help
./scripts/podman-run.sh --help

# Verify a transaction
./scripts/podman-run.sh tx --chain ethereum --nonce 63 --safe-address 0x1c694Fc3006D81ff4a56F97E1b99529066a23725 --safe-version 1.4.1

# Mount a directory to access local files
./scripts/podman-run.sh -v ./test msg --chain sepolia --safe-address 0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1 --input-file /app/input/test_message.txt --safe-version 1.4.1
```

#### Manual Podman Commands

If you prefer to run commands manually:

```bash
# Build the image
podman build -t localhost/safe-hash:latest .

# Run with basic security
podman run --rm --cap-drop ALL --read-only --tmpfs /tmp --network host localhost/safe-hash:latest --help

# Mount files for processing
podman run --rm --cap-drop ALL --read-only --tmpfs /tmp --network host -v ./test:/app/input:ro,Z localhost/safe-hash:latest msg --chain sepolia --safe-address 0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1 --input-file /app/input/test_message.txt --safe-version 1.4.1
```

#### Container Security Features

The container setup includes several security enhancements:
- **Minimal base image**: Uses Debian slim for reduced attack surface
- **Non-root user**: Runs as unprivileged user inside container
- **Read-only filesystem**: Container filesystem is mounted read-only
- **Dropped capabilities**: All Linux capabilities are dropped
- **Network isolation**: Can be run with restricted networking
- **Resource limits**: Easily configurable resource constraints

#### Docker Compatibility

The same Dockerfile works with Docker:

```bash
# Build with Docker
docker build -t safe-hash:latest .

# Run with Docker
docker run --rm --cap-drop ALL --read-only --tmpfs /tmp --network host safe-hash:latest --help
```

# Usage

This tool helps protect against possible phishing or compromised UI attacks by allowing local verification of transaction and message hashes before signing.

To see more, run the help command on the main or sub command:

## Help

```bash
safe-hash --help
safe-hash tx --help
safe-hash msg --help
safe-hash typed --help
```

## Live Examples

You can run all of these examples here!

### Transaction Signing

- Safe API, already created transaction:

```bash
safe-hash tx \
  --chain ethereum \
  --nonce 63 \
  --safe-address 0x1c694Fc3006D81ff4a56F97E1b99529066a23725 \
  --safe-version 1.4.1
```

### Transaction Signing With Nested Safe Address, offline mode

```bash
safe-hash tx \
   --safe-address 0xbC7977C6694Ae2Ae8Ad96bb1C100a281D928b7DB \
   --nonce 0 \
   --safe-version 1.4.1 \
   --chain sepolia \
   --to 0xdd13E55209Fd76AfE204dBda4007C227904f0a81 \
   --data 0xa9059cbb00000000000000000000000036bffa3048d89fad48509c83fdb6a3410232f3d300000000000000000000000000000000000000000000000000038d7ea4c68000 \
   --nested-safe-address 0x5031f5E2ed384978dca63306dc28A68a6Fc33e81 \
   --nested-safe-nonce 1 \
   --offline
```

### Message Signing

```bash
# Sign a message from a file
safe-hash msg \
  --chain sepolia \
  --safe-address 0x657ff0D4eC65D82b2bC1247b0a558bcd2f80A0f1 \
  --input-file test/test_message.txt \
  --safe-version 1.4.1
```

### EIP-712 encoding

Add this to a file called `file.json`:

```json
{"types":{"SafeTx":[{"type":"address","name":"to"},{"type":"uint256","name":"value"},{"type":"bytes","name":"data"},{"type":"uint8","name":"operation"},{"type":"uint256","name":"safeTxGas"},{"type":"uint256","name":"baseGas"},{"type":"uint256","name":"gasPrice"},{"type":"address","name":"gasToken"},{"type":"address","name":"refundReceiver"},{"type":"uint256","name":"nonce"}],"EIP712Domain":[{"name":"chainId","type":"uint256"},{"name":"verifyingContract","type":"address"}]},"domain":{"chainId":"42161","verifyingContract":"0x4087d2046A7435911fC26DCFac1c2Db26957Ab72"},"primaryType":"SafeTx","message":{"to":"0x82af49447d8a07e3bd95bd0d56f35241523fbab1","value":"0","data":"0xa9059cbb000000000000000000000000d8da6bf26964af9d7eed9e03e53415d37aa960450000000000000000000000000000000000000000000000000de0b6b3a7640000","operation":"0","safeTxGas":"0","baseGas":"0","gasPrice":"0","gasToken":"0x0000000000000000000000000000000000000000","refundReceiver":"0x0000000000000000000000000000000000000000","nonce":"29"}}
```

Then run:
```bash
safe-hash typed \
  --chain sepolia \
  --file file.json
```

### Example outputs

```bash
# safe-hash tx --chain ethereum --nonce 63  --safe-address 0x1c694Fc3006D81ff4a56F97E1b99529066a23725 --safe-version 1.4.1
Fetching transaction from API: https://safe-transaction-mainnet.safe.global/api/v1/safes/0x1c694Fc3006D81ff4a56F97E1b99529066a23725/multisig-transactions/?nonce=63
Full Tx Calldata:        6a761202000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001c00000000000000000000000000000000000000000000000000000000000000044a9059cbb00000000000000000000000092d0ebaf7eb707f0650f9471e61348f4656c29bc00000000000000000000000000000000000000000000000000000005d21dba00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082250b73a9ef806089dc55c1ac978a55fcf8a580cfc9984e41d10e5e0990cade2f217213bd70d69856b99a64cebd47bf552144000246aa604791adaa514734268720e5b5c20fa48b0633e35b044da59b43d86a20083f050307195f416ec7de4cae2728eb2773194b2fe39806d78f1398d2d775837cad077055d8bd60947b51aef3531f000000000000000000000000000000000000000000000000000000000000
Full Tx Calldata Hash:   0xde0b38a03da8f1888c2ed19578fc49526b13ee03a705d58a7cb4e32f76fa60ec
Safe Address:            0x1c694Fc3006D81ff4a56F97E1b99529066a23725
To:                      0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
Value:                   0
Data:                    0xa9059cbb00000000000000000000000092d0ebaf7eb707f0650f9471e61348f4656c29bc00000000000000000000000000000000000000000000000000000005d21dba00
Operation:               0
Nonce:                   63
Safe Tx Gas:             0
Base Gas:                0
Gas Price:               0
Gas Token:               0x0000000000000000000000000000000000000000
Refund Receiver:         0x0000000000000000000000000000000000000000
Confirmations Required:  2
Confirmations Count:     2

Decoded Call:
Method:      transfer
Parameter:   address: 0x92D0eBAF7Eb707F0650F9471E61348f4656c29bC
Parameter:   uint256: 25000000000

Main transaction
Domain Hash:             1655e94a9bcc5a957daa1acae692b4c22e7aaf146b4deb9194f8221d2f09d8c3
Message Hash:            f22754eba5a2b230714534b4657195268f00dc0031296de4b835d82e7aa1e574
Safe Transaction Hash:   ad06b099fca34e51e4886643d95d9a19ace2cd024065efb66662a876e8c40343
Verify the above value as the Safe Tx Hash when signing the message from the ledger.
```

## Trust Assumptions
* You trust this codebase
* You trust the Safe Wallet contracts
* You trust your OS (or container runtime when using Podman/Docker)
* You trust the [transaction service API](https://docs.safe.global/core-api/transaction-service-overview)
  * Unless you use the `--offline` flag.

When using containers:
* You trust the container runtime (Podman/Docker)
* You trust the base container images (Rust and Debian)
* The container provides additional isolation but doesn't eliminate all trust assumptions

# Community-Maintained User Interface Implementations

> [!IMPORTANT]
> Please be aware that user interface implementations may introduce additional trust assumptions, such as relying on `npm` dependencies that have not undergone thorough review. Always verify and cross-reference with the main [script](./safe_hashes.sh).

- [`Cyfrin Chain Tools`](https://tools.cyfrin.io/safe-hash)
- [`safeutils.openzeppelin.com`](https://safeutils.openzeppelin.com/)

# Acknowledgements

- [safe-tx-hashes-utils](https://github.com/pcaversaccio/safe-tx-hashes-util)
