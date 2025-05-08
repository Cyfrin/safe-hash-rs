# safe-hash - Verify Safe Wallet Transactions and Messages

> [!CAUTION]
> It is under development. Please use it at your own risk.

## Installation 

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

## Usage

This tool helps protect against possible phishing or compromised UI attacks by allowing local verification of transaction and message hashes before signing.

To see more, run:

### Help

```bash
safe-hash --help
```

To see help in more detail for any subcommand, run:

```bash
safe-hash tx --help
```

```bash
safe-hash typed --help
```

## Examples

### Transaction Signing

```bash
safe-hash tx \
  --chain ethereum \
  --nonce 63 \
  --safe-address 0x1c694Fc3006D81ff4a56F97E1b99529066a23725 \
  --data 0xa9059cbb00000000000000000000000092d0ebaf7eb707f0650f9471e61348f4656c29bc00000000000000000000000000000000000000000000000000000005d21dba00 \
  --safe-version 1.4.1
```
> By default, it runs in online mode. To force offline, you must pass `--offline` flag and supply the `--to` address.

### Nested Safe Address Example

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

```bash
safe-hash typed \
  --file sepolia \
  --safe-address test/pat_eip712_message.json
```

## Trust Assumptions
* You trust this codebase
* You trust your hardware wallet
* You trust the Safe Wallet contracts
