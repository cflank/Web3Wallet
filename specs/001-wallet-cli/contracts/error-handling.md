# Error Handling Contract

## Error Type Hierarchy

### WalletError (Root Error Type)

All operations return `Result<T, WalletError>` where WalletError is an enum containing all possible error conditions.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum WalletError {
    Cryptographic(CryptographicError),
    FileSystem(FileSystemError),
    UserInput(UserInputError),
    Authentication(AuthenticationError),
    Network(NetworkError),
    Validation(ValidationError),
}
```

### Cryptographic Errors (CRYPTO_xxx)

**CryptographicError**:
- `CRYPTO_001`: Insufficient entropy for secure key generation
- `CRYPTO_002`: Invalid BIP39 mnemonic phrase
- `CRYPTO_003`: Invalid private key format
- `CRYPTO_004`: Keystore decryption failed
- `CRYPTO_005`: Data corruption detected during decryption
- `CRYPTO_006`: Invalid HD derivation path
- `CRYPTO_007`: Derivation index out of valid range
- `CRYPTO_008`: Key derivation function failed
- `CRYPTO_009`: Signature generation failed
- `CRYPTO_010`: Address generation failed

**Error Context**:
```rust
#[derive(Debug, Clone)]
pub struct CryptographicError {
    pub code: String,
    pub message: String,
    pub operation: String,      // e.g., "mnemonic_generation", "key_derivation"
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub suggestion: Option<String>,
}
```

### File System Errors (FS_xxx)

**FileSystemError**:
- `FS_001`: Permission denied for file write operation
- `FS_002`: Wallet file not found
- `FS_003`: Directory not accessible
- `FS_004`: Disk space insufficient for operation
- `FS_005`: File already exists (for save operations)
- `FS_006`: Invalid file format or corruption
- `FS_007`: Path traversal security violation
- `FS_008`: File lock acquisition failed

### User Input Errors (INPUT_xxx)

**UserInputError**:
- `INPUT_001`: Invalid command parameters
- `INPUT_002`: Conflicting command options
- `INPUT_003`: Missing required parameter
- `INPUT_004`: Parameter value out of valid range
- `INPUT_005`: Unsupported output format
- `INPUT_006`: Invalid network specification
- `INPUT_007`: Password confirmation mismatch
- `INPUT_008`: Operation timeout (user input)

### Authentication Errors (AUTH_xxx)

**AuthenticationError**:
- `AUTH_001`: Wrong password for wallet decryption
- `AUTH_002`: Password too weak (minimum requirements not met)
- `AUTH_003`: Maximum authentication attempts exceeded
- `AUTH_004`: Session timeout
- `AUTH_005`: User canceled authentication

### Network Errors (NETWORK_xxx)

**NetworkError**:
- `NETWORK_001`: Network connectivity failure
- `NETWORK_002`: Request timeout
- `NETWORK_003`: Invalid network configuration
- `NETWORK_004`: Rate limiting exceeded
- `NETWORK_005`: Unsupported network protocol

### Validation Errors (VALIDATION_xxx)

**ValidationError**:
- `VALIDATION_001`: Address format validation failed
- `VALIDATION_002`: Keystore schema validation failed
- `VALIDATION_003`: Command syntax validation failed
- `VALIDATION_004`: Data integrity check failed
- `VALIDATION_005`: Version compatibility check failed

## Error Display Contract

### User-Facing Error Messages

All errors must provide:
1. **Clear Description**: What went wrong in plain language
2. **Context Information**: Relevant details without exposing sensitive data
3. **Actionable Suggestion**: How the user can resolve the issue
4. **Error Code**: For programmatic handling and support

### Message Format Template
```
Error [CODE]: [DESCRIPTION]

Details:
  [CONTEXT_KEY]: [CONTEXT_VALUE]

Suggestion: [ACTIONABLE_ADVICE]

For technical support, reference error code: [CODE]
```

### Example Error Messages

#### CRYPTO_002: Invalid Mnemonic
```
Error CRYPTO_002: Invalid BIP39 mnemonic phrase

Details:
  Word count: 11 (expected: 12 or 24)
  Invalid words: ["invalidword"]

Suggestion: Verify the mnemonic phrase has the correct number of words (12 or 24) and all words are from the BIP39 wordlist.

For technical support, reference error code: CRYPTO_002
```

#### FS_002: File Not Found
```
Error FS_002: Wallet file not found

Details:
  Requested file: my-wallet.json
  Search directory: /home/user/.wallet

Suggestion: Check that the filename is correct and the wallet file exists in the expected directory. Use 'wallet list' to see available wallets.

For technical support, reference error code: FS_002
```

#### AUTH_001: Wrong Password
```
Error AUTH_001: Incorrect password for wallet decryption

Details:
  Wallet file: my-wallet.json
  Attempts remaining: 2

Suggestion: Verify the password is correct. After 3 failed attempts, the operation will be blocked for security.

For technical support, reference error code: AUTH_001
```

## Error Recovery Strategies

### Automatic Recovery
- **Transient Network Errors**: Implement exponential backoff retry
- **File Locking**: Wait and retry with timeout
- **Memory Errors**: Trigger garbage collection and retry once

### User-Guided Recovery
- **Wrong Password**: Prompt for retry with attempt counter
- **File Conflicts**: Offer overwrite or rename options
- **Missing Dependencies**: Provide installation instructions

### Graceful Degradation
- **Network Unavailable**: Operate in offline mode where possible
- **Insufficient Permissions**: Suggest alternative file locations
- **Outdated Format**: Offer format migration

## Security Considerations

### Information Disclosure Prevention
- Never include private keys, mnemonics, or passwords in error messages
- Sanitize file paths to prevent information leakage
- Use generic messages for authentication failures to prevent user enumeration

### Error Logging
- Log errors with sufficient detail for debugging
- Redact sensitive information from logs
- Include request IDs for tracing in production
- Implement log rotation and retention policies

### Rate Limiting
- Implement exponential backoff for repeated authentication failures
- Temporary lockout after excessive failed attempts
- Track and limit error-generating requests

## Testing Contract

### Error Injection Testing
All error paths must be testable through:
- Mock implementations that trigger specific error conditions
- Integration tests with invalid inputs
- Stress tests to trigger resource exhaustion errors
- Network simulation to test timeout and connectivity errors

### Error Message Validation
- Verify error codes are unique and consistent
- Ensure error messages are helpful and actionable
- Test error message formatting across different output modes
- Validate that sensitive information is not exposed

### Recovery Testing
- Test automatic retry mechanisms
- Verify graceful degradation scenarios
- Ensure error state cleanup prevents resource leaks
- Test user-guided recovery workflows

This error handling contract ensures robust, secure, and user-friendly error management throughout the Web3 wallet CLI application while maintaining constitutional requirements for security and reliability.