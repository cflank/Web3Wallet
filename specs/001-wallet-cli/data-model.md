# Data Model: Web3 Wallet CLI

## Core Entities

### Wallet
The primary entity representing a hierarchical deterministic wallet.

**Fields**:
- `mnemonic`: BIP39 mnemonic phrase (12 or 24 words)
- `master_private_key`: Master private key derived from mnemonic
- `master_public_key`: Master public key for address derivation
- `derivation_path`: Base HD path (default: m/44'/60'/0'/0)
- `created_at`: Timestamp of wallet creation
- `alias`: User-defined name for the wallet
- `network`: Target network (mainnet, sepolia, etc.)

**Relationships**:
- One-to-many with Address entities
- One-to-one with Keystore for persistence

**Validation Rules**:
- Mnemonic must be valid BIP39 phrase
- Derivation path must follow BIP44 standard
- Alias must be unique within local storage
- Network must be supported Ethereum-compatible chain

**State Transitions**:
- Created → Unlocked (via password)
- Unlocked → Locked (after timeout or explicit command)
- Created → Imported (from existing mnemonic/private key)

### Keystore
Encrypted storage format for wallet persistence.

**Fields**:
- `encrypted_data`: AES-256-GCM encrypted wallet data
- `salt`: Cryptographic salt for key derivation (32 bytes)
- `nonce`: AES-GCM nonce (12 bytes)
- `kdf_params`: Key derivation function parameters
  - `algorithm`: "argon2id" or "pbkdf2"
  - `iterations`: Number of iterations
  - `memory`: Memory usage (for Argon2)
  - `parallelism`: Parallel threads (for Argon2)
- `cipher_params`: Encryption parameters
  - `algorithm`: "aes-256-gcm"
  - `iv`: Initialization vector
- `mac`: Message authentication code
- `version`: Keystore format version
- `metadata`: Non-sensitive wallet information
  - `alias`: Wallet alias
  - `address`: Primary Ethereum address
  - `created_at`: Creation timestamp
  - `network`: Target network

**Relationships**:
- One-to-one with Wallet entity
- Stored as JSON file in local filesystem

**Validation Rules**:
- Encrypted data must be valid AES-256-GCM ciphertext
- Salt must be cryptographically random
- KDF parameters must meet security minimums
- MAC must validate against encrypted data
- Version must be supported format

### Address
Individual Ethereum address derived from HD wallet.

**Fields**:
- `ethereum_address`: 42-character hex address with 0x prefix
- `private_key`: 32-byte private key for this address
- `public_key`: 33-byte compressed public key
- `derivation_index`: Index in HD path (m/44'/60'/0'/0/index)
- `derivation_path`: Full HD path for this address
- `balance`: Last known ETH balance (optional, cached)
- `nonce`: Last known transaction nonce (optional, cached)
- `created_at`: Timestamp of address derivation

**Relationships**:
- Many-to-one with Wallet entity
- One-to-many with Transaction entities

**Validation Rules**:
- Address must be valid Ethereum address format
- Private key must correspond to the address
- Derivation index must be non-negative integer
- Derivation path must be valid BIP44 format

### Command
CLI operation with parameters and execution context.

**Fields**:
- `subcommand`: Primary command (create, import, load, list, derive)
- `parameters`: Command-specific arguments and flags
- `output_format`: "json" or "table"
- `input_source`: Source of sensitive data (stdin, file, prompt)
- `validation_rules`: Input validation requirements
- `security_level`: Required security checks (password, confirmation)

**Relationships**:
- Commands operate on Wallet and Address entities
- Commands produce CommandResult entities

**Validation Rules**:
- Subcommand must be in allowed list
- Parameters must match subcommand requirements
- Output format must be supported
- Security level appropriate for operation sensitivity

### Error Types
Comprehensive error handling for all failure modes.

**Error Categories**:
- `CryptographicError`: Encryption, decryption, key derivation failures
- `ValidationError`: Input validation and format checking failures
- `FileSystemError`: File I/O and permission errors
- `NetworkError`: Blockchain connectivity and timeout errors
- `UserInputError`: Invalid user-provided data
- `SecurityError`: Authentication and authorization failures

**Fields**:
- `error_type`: Specific error category
- `message`: User-friendly error description
- `code`: Unique error identifier for programmatic handling
- `context`: Additional error context for debugging
- `severity`: Error severity level (warning, error, critical)

## Data Flow Architecture

### Wallet Creation Flow
```
User Input → Entropy Generation → Mnemonic → Master Key → Addresses → Keystore → File Storage
```

### Wallet Import Flow
```
User Input → Validation → Key Derivation → Address Generation → Keystore → File Storage
```

### Address Derivation Flow
```
Keystore → Decryption → Master Key → HD Derivation → Address → Validation
```

### Security Context
All sensitive data (private keys, mnemonics) must:
- Use `zeroize` for secure memory cleanup
- Be encrypted before any file I/O operations
- Never appear in logs or error messages
- Have minimal lifetime in memory
- Be protected by OS-level security features

## Storage Schema

### Keystore JSON Format
```json
{
  "version": "1.0.0",
  "metadata": {
    "alias": "my-wallet",
    "address": "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99",
    "created_at": "2025-01-21T10:30:00Z",
    "network": "mainnet"
  },
  "crypto": {
    "cipher": "aes-256-gcm",
    "ciphertext": "...",
    "cipherparams": {
      "iv": "..."
    },
    "kdf": "argon2id",
    "kdfparams": {
      "dklen": 32,
      "memory": 47104,
      "time": 1,
      "parallelism": 1,
      "salt": "..."
    },
    "mac": "..."
  }
}
```

This data model ensures type safety, security best practices, and compatibility with existing Ethereum ecosystem standards while maintaining the performance and usability requirements defined in the project constitution.