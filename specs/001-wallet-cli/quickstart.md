# Quickstart Guide: Web3 Wallet CLI

## Prerequisites
- Rust 2021 Edition toolchain installed
- Basic understanding of Ethereum wallets and private key security
- Terminal/command line access

## Installation

### From Source
```bash
git clone https://github.com/user/web3wallet-cli
cd web3wallet-cli
cargo build --release
cargo install --path .
```

### Verify Installation
```bash
wallet --version
# Expected output: wallet-cli 1.0.0
```

## Quick Start Scenarios

### Scenario 1: Create Your First Wallet

**Step 1**: Generate a new wallet
```bash
wallet create --words 24 --save my-first-wallet
```

**Expected Output**:
```
🔐 Generating new wallet...

✅ Wallet created successfully!

Mnemonic (WRITE THIS DOWN SECURELY):
abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art

Address: 0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99
Derivation Path: m/44'/60'/0'/0/0
Network: mainnet

Enter password to encrypt wallet: [hidden input]
Confirm password: [hidden input]

💾 Wallet saved to: my-first-wallet.json
```

**Step 2**: Verify the wallet was saved
```bash
wallet list
```

**Expected Output**:
```
📋 Local Wallets:

┌─────────────────────────────────────────────┬─────────────────┬─────────────┬─────────────────────┐
│ Address                                     │ Alias           │ Network     │ Created             │
├─────────────────────────────────────────────┼─────────────────┼─────────────┼─────────────────────┤
│ 0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99 │ my-first-wallet │ mainnet     │ 2025-01-21 10:30:00 │
└─────────────────────────────────────────────┴─────────────────┴─────────────┴─────────────────────┘
```

**Validation**: Address matches the one shown during creation

### Scenario 2: Import Existing MetaMask Wallet

**Step 1**: Import wallet using MetaMask mnemonic
```bash
wallet import --mnemonic "word1 word2 word3 ... word12" --save imported-metamask
```

**Expected Behavior**:
- Prompts for password to encrypt the imported wallet
- Shows address that matches your MetaMask wallet
- Saves encrypted keystore file

**Step 2**: Verify MetaMask compatibility
```bash
wallet load imported-metamask.json --address-only
```

**Expected Result**: Address matches exactly with MetaMask for the same mnemonic

### Scenario 3: Derive Additional Addresses

**Step 1**: Load wallet and derive address index 5
```bash
wallet derive 5 --from-file my-first-wallet.json
```

**Expected Output**:
```
🔓 Enter password for my-first-wallet.json: [hidden input]

📍 Derived Address:

Index: 5
Derivation Path: m/44'/60'/0'/0/5
Address: 0x8ba1f109551bD432803012645Hac136c53962394
Private Key: 0x4c0883a69102937d6231471b5dbb6204fe512961708279c1e3ae83da5e56...
```

**Step 2**: Derive multiple addresses at once
```bash
wallet derive 0 --from-file my-first-wallet.json --count 5
```

**Expected Behavior**: Shows addresses 0-4 in sequence

### Scenario 4: Address-Only Operations

**Step 1**: View wallet address without password
```bash
wallet load my-first-wallet.json --address-only
```

**Expected Output**:
```
📍 Wallet Information:

Alias: my-first-wallet
Address: 0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99
Network: mainnet
Created: 2025-01-21 10:30:00

🔒 Private data not decrypted (--address-only mode)
```

**Validation**: No password prompt, no sensitive data displayed

### Scenario 5: JSON Output for Automation

**Step 1**: Create wallet with JSON output
```bash
wallet create --words 12 --output json
```

**Expected Output**:
```json
{
  "success": true,
  "data": {
    "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    "address": "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99",
    "private_key": "0x4c0883a69102937d6231471b5dbb6204fe512961708279c1e3ae83da5e56df1a",
    "derivation_path": "m/44'/60'/0'/0/0",
    "network": "mainnet"
  }
}
```

**Validation**: Valid JSON format, parseable by automation tools

## Performance Validation

### Response Time Testing
All commands should complete within 1 second:

```bash
time wallet create --words 12
# real    0m0.847s

time wallet list
# real    0m0.123s

time wallet load my-wallet.json --address-only
# real    0m0.089s
```

### Memory Usage Testing
```bash
# Monitor memory during wallet creation
/usr/bin/time -v wallet create --words 24
# Maximum resident set size: should be < 50MB
```

## Security Validation

### Encryption Verification
**Test 1**: Verify keystore files are encrypted
```bash
cat my-first-wallet.json | grep -o '"ciphertext"'
# Should find encrypted data, not plaintext keys
```

**Test 2**: Verify wrong password handling
```bash
wallet load my-first-wallet.json
# Enter incorrect password
# Expected: Error AUTH_001 with clear message
```

### Memory Cleanup Verification
**Test**: Check that sensitive data is cleared from memory
- Create wallet
- Use memory analysis tools to verify private keys are not in process memory after operation completes

## Troubleshooting Common Issues

### Issue 1: "Error CRYPTO_001: Insufficient entropy"
**Solution**: Ensure system has adequate entropy sources
```bash
# Linux: Check entropy
cat /proc/sys/kernel/random/entropy_avail
# Should be > 256

# Generate additional entropy if needed
sudo apt-get install rng-tools
```

### Issue 2: "Error FS_001: Permission denied"
**Solution**: Check file permissions and directory access
```bash
# Check current directory permissions
ls -la
# Ensure write permissions for current user

# Use alternative directory
wallet create --save ~/wallets/my-wallet
```

### Issue 3: "Error CRYPTO_002: Invalid BIP39 mnemonic"
**Solution**: Verify mnemonic phrase format
- Ensure correct number of words (12 or 24)
- Check for typos in individual words
- Verify words are from BIP39 wordlist

## Success Criteria Validation

After completing this quickstart:

✅ **Generated Address Compatibility**: Addresses match MetaMask for same mnemonic
✅ **Encryption Security**: Private keys never stored in plaintext
✅ **Performance Requirements**: All operations complete < 1 second
✅ **CLI Usability**: Commands are intuitive and provide clear feedback
✅ **Error Handling**: Errors provide actionable guidance
✅ **Cross-Platform**: Works on Windows, macOS, and Linux

## Next Steps

1. **Backup Security**: Store mnemonic phrases in secure offline location
2. **Network Configuration**: Explore testnet options for development
3. **Integration**: Use JSON output mode for automation scripts
4. **Advanced Features**: Explore custom derivation paths and bulk operations

For advanced usage and API documentation, see the full project documentation at `docs/api.md`.