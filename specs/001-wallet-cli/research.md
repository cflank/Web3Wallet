# Research: Web3 Wallet CLI Development

## Executive Summary
Comprehensive research analysis for building a production-ready Web3 wallet CLI tool in Rust, focusing on security-first principles, performance optimization, and ecosystem compatibility.

## 1. Cryptographic Library Selection

### Decision: Alloy-rs (with fallback considerations for ethers-rs)
**Rationale**:
- Alloy v1.0 (2024) is the official successor to ethers-rs, offering 60% faster U256 operations and 10x faster ABI encoding
- Active development with major ecosystem adoption (Reth, Foundry, Revm, SP1 zkVM)
- Built-in cryptographic utilities for secp256k1 operations with high-performance guarantees
- Security-audited codebase with comprehensive documentation

**Alternatives Considered**:
- ethers-rs: Deprecated and archived (October 2024), superseded by Alloy
- web3-rust: Minimal maintenance, maintainer recommends Alloy
- rust-secp256k1: Specialized for secp256k1 operations, can complement Alloy
- k256: Pure Rust implementation for no-C-dependency environments

**Implementation Strategy**: Use Alloy as primary framework with rust-secp256k1 for specialized cryptographic operations requiring maximum security isolation.

## 2. HD Wallet Implementation Standards

### Decision: BIP44 derivation with m/44'/60'/0'/0/x path using Alloy HD utilities
**Rationale**:
- Industry standard ensuring MetaMask and ecosystem compatibility
- Proven security model with widespread adoption
- Rust ownership system provides inherent memory safety for private key handling
- Comprehensive error handling through Result types

**Alternatives Considered**:
- Custom derivation paths: Poor ecosystem compatibility
- BIP32 only: Less standardized for multi-coin scenarios
- Hardware wallet exclusive: Limited accessibility

**Security Considerations**:
- Implement zeroize for secure memory cleanup
- Derive keys on-demand to minimize memory exposure
- Store only master seed, not derived private keys
- Comprehensive derivation path validation

## 3. Keystore Format Compatibility

### Decision: Ethereum UTC/JSON Keystore with PBKDF2-HMAC-SHA256 and AES-128-CTR
**Rationale**:
- Universal compatibility with MetaMask, MEW, Geth, Parity
- Current MetaMask uses 900,000 PBKDF2 iterations (exceeds OWASP 2024 recommendations)
- Proven format with extensive ecosystem testing
- Well-documented standard with comprehensive tooling support

**Alternatives Considered**:
- Custom encryption: Breaks ecosystem compatibility
- Hardware wallet only: Reduces accessibility
- Higher iteration counts: Potential performance impact

**Compatibility Matrix**:
- **Export**: Always use 900k iterations (current MetaMask standard)
- **Import**: Support legacy formats (lower iterations) for backward compatibility
- **Encryption**: AES-128-CTR with 32-byte salt, 16-byte IV
- **Validation**: Comprehensive JSON schema validation for all imports

## 4. Secure Password and Memory Management

### Decision: Argon2id for KDF, rpassword for input, zeroize for memory protection
**Rationale**:
- Argon2id is OWASP 2024 recommended algorithm, resistant to GPU/ASIC attacks
- Meets <1s performance requirement with proper parameter tuning
- Cross-platform compatibility (Linux, Windows, macOS, WASM)
- Guaranteed memory clearing with zeroize integration

**Alternatives Considered**:
- PBKDF2: FIPS compliant but requires 600k+ iterations, less resistant to modern attacks
- bcrypt: Good security but less configurable
- scrypt: Good memory hardness but Argon2 more standardized

**Parameter Optimization** (OWASP 2024 compliant):
- Memory: 47,104 KB (46 MiB) with t=1, p=1 for maximum security
- Alternative: 19,456 KB (19 MiB) with t=2, p=1 for performance balance
- Salt: 32 bytes from cryptographically secure RNG (getrandom crate)

## 5. Performance and Security Integration

### Entropy Sources
**Decision**: getrandom crate with OS-specific fallbacks
- Cryptographically secure randomness across platforms
- Entropy quality validation before key generation
- Fallback mechanisms for constrained environments

### Memory Protection Strategy
**Implementation**:
```rust
#[derive(Zeroize, ZeroizeOnDrop)]
struct SecureData {
    sensitive: Vec<u8>,
}
```

### Async Operations Architecture
- Key derivation operations use tokio for UI responsiveness
- File I/O operations asynchronous to prevent blocking
- Timeout handling for all blockchain operations (30s max per constitution)

## 6. Testing and Validation Strategy

### Cryptographic Testing
- Property-based tests for all cryptographic operations
- Cross-validation with known test vectors (BIP39, BIP44)
- Compatibility testing with MetaMask-generated keystores
- Security regression testing for memory leaks

### Performance Benchmarks
- Command response time validation (<1s requirement)
- Memory usage profiling (<50MB baseline per constitution)
- Stress testing with large keystore collections
- Cross-platform performance validation

## 7. Dependency Security Assessment

### Core Dependency Audit Status
- **Alloy**: Active development, regular security audits
- **bip39**: Mature, well-audited, minimal dependency tree
- **aes-gcm**: CAESAR competition winner, extensive analysis
- **argon2**: Password Hashing Competition winner, OWASP recommended
- **zeroize**: Security-focused memory clearing, minimal attack surface

### Update Strategy
- Semantic versioning for all security-critical dependencies
- Regular dependency audits using cargo-audit
- Security impact assessment for all updates
- Pinned versions for reproducible builds

## Implementation Roadmap

### Phase 1: Core Security Infrastructure
1. Establish secure memory management patterns
2. Implement Argon2id key derivation
3. Create UTC/JSON keystore format handlers
4. Establish comprehensive error handling framework

### Phase 2: Cryptographic Operations
1. Integrate Alloy for HD wallet functionality
2. Implement BIP39 mnemonic generation and validation
3. Create address derivation with BIP44 compliance
4. Establish secure key import/export workflows

### Phase 3: CLI Interface and Integration
1. Build command structure with clap framework
2. Implement secure password input workflows
3. Create output formatting (JSON/human-readable)
4. Establish comprehensive testing suite

This research foundation ensures the Web3 wallet CLI tool meets constitutional requirements for security-first development, error handling excellence, and performance standards while maintaining ecosystem compatibility.