# CLI Interface Contract

## Command Structure

### Primary Commands

#### `wallet create`
**Purpose**: Generate a new HD wallet with BIP39 mnemonic
**Usage**: `wallet create [OPTIONS]`

**Options**:
- `--words <12|24>`: Mnemonic word count (default: 12)
- `--save <filename>`: Save encrypted keystore to file
- `--output <json|table>`: Output format (default: table)
- `--network <mainnet|sepolia|...>`: Target network (default: mainnet)

**Output**:
```json
{
  "success": true,
  "data": {
    "mnemonic": "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    "address": "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99",
    "private_key": "0x...",
    "derivation_path": "m/44'/60'/0'/0/0",
    "network": "mainnet"
  }
}
```

**Error Cases**:
- Insufficient entropy: `CRYPTO_001`
- File write permission: `FS_001`
- Invalid parameters: `INPUT_001`

#### `wallet import`
**Purpose**: Import existing wallet from mnemonic or private key
**Usage**: `wallet import [OPTIONS]`

**Options**:
- `--mnemonic <words>`: BIP39 mnemonic phrase
- `--private-key <hex>`: Raw private key (0x prefixed)
- `--save <filename>`: Save encrypted keystore to file
- `--output <json|table>`: Output format (default: table)
- `--network <mainnet|sepolia|...>`: Target network (default: mainnet)

**Validation**:
- Mnemonic must be valid BIP39 phrase
- Private key must be 64 hex characters (with or without 0x prefix)
- Either mnemonic OR private key required, not both

**Output**: Same as `wallet create`

**Error Cases**:
- Invalid mnemonic: `CRYPTO_002`
- Invalid private key format: `CRYPTO_003`
- Conflicting input methods: `INPUT_002`

#### `wallet load`
**Purpose**: Load and decrypt saved wallet
**Usage**: `wallet load <filename> [OPTIONS]`

**Options**:
- `--address-only`: Show only address without decrypting private data
- `--output <json|table>`: Output format (default: table)
- `--derive <index>`: Derive specific address index

**Authentication**: Password prompt for decryption

**Output**:
```json
{
  "success": true,
  "data": {
    "alias": "my-wallet",
    "address": "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99",
    "created_at": "2025-01-21T10:30:00Z",
    "network": "mainnet",
    "mnemonic": "abandon abandon...",  // Only if not --address-only
    "private_key": "0x..."             // Only if not --address-only
  }
}
```

**Error Cases**:
- File not found: `FS_002`
- Invalid keystore format: `CRYPTO_004`
- Wrong password: `AUTH_001`
- Corrupted file: `CRYPTO_005`

#### `wallet list`
**Purpose**: List all locally stored wallet files
**Usage**: `wallet list [OPTIONS]`

**Options**:
- `--output <json|table>`: Output format (default: table)
- `--path <directory>`: Custom wallet directory

**Output**:
```json
{
  "success": true,
  "data": [
    {
      "filename": "my-wallet.json",
      "alias": "my-wallet",
      "address": "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99",
      "created_at": "2025-01-21T10:30:00Z",
      "network": "mainnet",
      "file_size": 1024
    }
  ]
}
```

**Error Cases**:
- Directory not accessible: `FS_003`
- No wallet files found: `WALLET_001`

#### `wallet derive`
**Purpose**: Derive additional addresses from HD wallet
**Usage**: `wallet derive <path> [OPTIONS]`

**Options**:
- `--from-file <filename>`: Source wallet file
- `--count <number>`: Number of addresses to derive (default: 1)
- `--start-index <number>`: Starting derivation index (default: 0)
- `--output <json|table>`: Output format (default: table)

**Path Format**: `m/44'/60'/0'/0/x` or `x` (shorthand for index)

**Authentication**: Password prompt for wallet decryption

**Output**:
```json
{
  "success": true,
  "data": [
    {
      "index": 5,
      "derivation_path": "m/44'/60'/0'/0/5",
      "address": "0x...",
      "private_key": "0x..."
    }
  ]
}
```

**Error Cases**:
- Invalid derivation path: `CRYPTO_006`
- Index out of range: `CRYPTO_007`
- Wallet decryption failed: `AUTH_001`

## Global Options

**Available for all commands**:
- `--help, -h`: Show command help
- `--version, -V`: Show version information
- `--verbose, -v`: Enable verbose logging
- `--quiet, -q`: Suppress non-essential output
- `--config <file>`: Use custom configuration file

## Output Formats

### Table Format (Human-readable)
```
┌─────────────────────────────────────────────┬──────────────┬─────────────┐
│ Address                                     │ Alias        │ Network     │
├─────────────────────────────────────────────┼──────────────┼─────────────┤
│ 0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99 │ my-wallet    │ mainnet     │
└─────────────────────────────────────────────┴──────────────┴─────────────┘
```

### JSON Format (Machine-readable)
All responses follow consistent structure:
```json
{
  "success": boolean,
  "data": object,
  "error": {
    "code": "string",
    "message": "string",
    "details": object
  }
}
```

## Error Handling Contract

### Error Code Categories
- `CRYPTO_xxx`: Cryptographic operation failures
- `FS_xxx`: File system operation failures
- `INPUT_xxx`: User input validation failures
- `AUTH_xxx`: Authentication and authorization failures
- `NETWORK_xxx`: Network connectivity failures
- `WALLET_xxx`: Wallet-specific logic failures

### Error Response Format
```json
{
  "success": false,
  "error": {
    "code": "CRYPTO_001",
    "message": "Insufficient entropy for secure key generation",
    "details": {
      "entropy_bits": 128,
      "required_bits": 256,
      "suggestion": "Ensure system has adequate entropy sources"
    }
  }
}
```

## Security Requirements

### Password Input
- Always use hidden input for password prompts
- Implement timeout for password entry (30 seconds)
- Clear password from memory immediately after use
- Support password confirmation for destructive operations

### Sensitive Data Display
- Private keys and mnemonics only shown when explicitly requested
- Implement confirmation prompts for sensitive operations
- Support --address-only mode for address verification without decryption

### File Operations
- Use secure file permissions (600) for keystore files
- Implement atomic file writes to prevent corruption
- Validate file integrity before processing
- Support custom directory locations for wallet storage

This contract ensures consistent, secure, and user-friendly CLI interactions while maintaining compatibility with security requirements and performance targets.