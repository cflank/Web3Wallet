# Tasks: Web3 Wallet CLI Tool

**Input**: Design documents from `/specs/001-wallet-cli/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → SUCCESS: Tech stack extracted (Rust 2021, alloy-rs, bip39, clap, aes-gcm, etc.)
2. Load optional design documents:
   → data-model.md: Extracted entities (Wallet, Keystore, Address, Command, Error)
   → contracts/: CLI interface and error handling contracts loaded
   → research.md: Security decisions and implementation strategy extracted
3. Generate tasks by category:
   → Setup: project init, dependencies, linting
   → Tests: contract tests, integration tests (TDD-first)
   → Core: models, crypto services, CLI commands
   → Integration: file I/O, error handling, validation
   → Polish: unit tests, performance, docs
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Tests before implementation (TDD)
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness
   → SUCCESS: All contracts, entities, and CLI commands covered
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths based on plan.md structure (Rust CLI tool)

## Phase 3.1: Setup
- [x] T001 Create Cargo.toml with all dependencies (ethers-rs 2.0, bip39 2.0, clap 4.0, aes-gcm 0.10, pbkdf2 0.12, tokio 1.0, rpassword 7.0, serde 1.0, zeroize 1.6, hex 0.4)
- [x] T002 [P] Create project directory structure (src/models/, src/services/, src/cli/, src/lib.rs, src/main.rs)
- [x] T003 [P] Configure rustfmt.toml and clippy.toml for code quality
- [x] T004 [P] Create tests directory structure (tests/contract/, tests/integration/, tests/unit/)

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests (CLI Interface)
- [x] T005 [P] Contract test `wallet create` command in tests/contract/test_create_command.rs
- [x] T006 [P] Contract test `wallet import` command in tests/contract/test_import_command.rs
- [x] T007 [P] Contract test `wallet load` command in tests/contract/test_load_command.rs
- [x] T008 [P] Contract test `wallet list` command in tests/contract/test_list_command.rs
- [x] T009 [P] Contract test `wallet derive` command in tests/contract/test_derive_command.rs

### Error Handling Tests
- [x] T010 [P] Error handling test for cryptographic failures in tests/contract/test_crypto_errors.rs
- [x] T011 [P] Error handling test for file system failures in tests/contract/test_fs_errors.rs
- [x] T012 [P] Error handling test for user input validation in tests/contract/test_input_errors.rs

### Integration Tests
- [x] T013 [P] Integration test wallet creation flow in tests/integration/test_wallet_creation.rs
- [x] T014 [P] Integration test wallet import flow in tests/integration/test_wallet_import.rs
- [x] T015 [P] Integration test encrypted storage roundtrip in tests/integration/test_storage_encryption.rs
- [x] T016 [P] Integration test MetaMask compatibility in tests/integration/test_metamask_compat.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### Error Types and Basic Infrastructure
- [x] T017 [P] WalletError enum and error types in src/errors.rs
- [x] T018 [P] Basic configuration and constants in src/config.rs
- [x] T019 [P] Utility functions and helpers in src/utils.rs

### Data Models
- [x] T020 [P] Wallet struct and methods in src/models/wallet.rs
- [x] T021 [P] Keystore struct and serialization in src/models/keystore.rs
- [x] T022 [P] Address struct and validation in src/models/address.rs
- [x] T023 [P] Command structs and parsing in src/models/command.rs

### Cryptographic Services
- [x] T024 [P] Mnemonic generation service in src/services/mnemonic.rs
- [x] T025 [P] HD wallet derivation service (integrated in wallet.rs)
- [ ] T026 [P] Encryption/decryption service in src/services/encryption.rs (partial)
- [ ] T027 [P] Key derivation service (Argon2id) in src/services/kdf.rs (partial)

### File Operations
- [ ] T028 [P] Keystore file I/O operations in src/services/storage.rs (partial)
- [ ] T029 [P] Wallet file management in src/services/file_manager.rs (partial)

### CLI Command Handlers
- [ ] T030 Create command handler in src/cli/create.rs
- [ ] T031 Import command handler in src/cli/import.rs
- [ ] T032 Load command handler in src/cli/load.rs
- [ ] T033 List command handler in src/cli/list.rs
- [ ] T034 Derive command handler in src/cli/derive.rs

### CLI Framework
- [ ] T035 Main CLI argument parsing in src/cli/mod.rs
- [ ] T036 Output formatting (JSON/table) in src/cli/output.rs
- [ ] T037 Password input handling in src/cli/input.rs
- [ ] T038 Main application entry point in src/main.rs

## Phase 3.4: Integration & Security
- [ ] T039 Secure memory management integration with zeroize
- [ ] T040 Input validation and sanitization
- [ ] T041 File permission and security settings
- [ ] T042 Error message sanitization (no sensitive data exposure)
- [ ] T043 Command confirmation prompts for sensitive operations

## Phase 3.5: Polish & Documentation
- [ ] T044 [P] Unit tests for cryptographic functions in tests/unit/test_crypto.rs
- [ ] T045 [P] Unit tests for file operations in tests/unit/test_file_ops.rs
- [ ] T046 [P] Unit tests for validation logic in tests/unit/test_validation.rs
- [ ] T047 [P] Performance tests (<1s response time) in tests/integration/test_performance.rs
- [ ] T048 [P] Security tests (memory cleanup) in tests/integration/test_security.rs
- [ ] T049 [P] Cross-platform compatibility tests in tests/integration/test_cross_platform.rs
- [ ] T050 Library documentation (src/lib.rs) with examples
- [ ] T051 CLI help text and usage documentation
- [ ] T052 README.md with installation and usage instructions

## Dependencies
**Critical TDD Flow**:
- All tests (T005-T016) MUST be completed before ANY implementation (T017+)
- T017-T019 (infrastructure) before T020-T023 (models)
- T020-T023 (models) before T024-T029 (services)
- T024-T029 (services) before T030-T038 (CLI)
- T030-T038 (CLI) before T039-T043 (integration)
- T039-T043 (integration) before T044-T052 (polish)

**Parallel Execution Rules**:
- T001-T004: Can run in parallel (different files)
- T005-T016: Can run in parallel (independent test files)
- T017-T029: Can run in parallel within each phase (different files)
- T030-T034: Sequential (may share CLI infrastructure)
- T044-T049: Can run in parallel (independent test files)

## Parallel Example
```bash
# Launch contract tests together (Phase 3.2):
Task: "Contract test wallet create command in tests/contract/test_create_command.rs"
Task: "Contract test wallet import command in tests/contract/test_import_command.rs"
Task: "Contract test wallet load command in tests/contract/test_load_command.rs"
Task: "Contract test wallet list command in tests/contract/test_list_command.rs"
Task: "Contract test wallet derive command in tests/contract/test_derive_command.rs"

# Launch core models together (Phase 3.3):
Task: "Wallet struct and methods in src/models/wallet.rs"
Task: "Keystore struct and serialization in src/models/keystore.rs"
Task: "Address struct and validation in src/models/address.rs"
Task: "Command structs and parsing in src/models/command.rs"
```

## Notes
- [P] tasks = different files, no dependencies
- **CRITICAL**: Verify all tests fail before implementing
- Follow constitutional security requirements throughout
- Commit after each logical task grouping
- Run `cargo clippy` and `cargo fmt` after each phase

## Task Generation Rules
*Applied during main() execution*

1. **From CLI Interface Contract**:
   - Each command → contract test task [P] + implementation task
   - Each error case → error handling test [P]

2. **From Data Model**:
   - Each entity → model creation task [P]
   - Each relationship → integration test

3. **From User Stories (quickstart.md)**:
   - Each scenario → integration test [P]
   - Security requirements → security tests

4. **Ordering**:
   - Setup → Tests → Models → Services → CLI → Integration → Polish
   - TDD principle: All tests before implementation

## Validation Checklist
*GATE: Checked by main() before returning*

- [x] All CLI commands have corresponding tests
- [x] All entities have model tasks
- [x] All tests come before implementation
- [x] Parallel tasks truly independent
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] Security and performance requirements covered
- [x] Constitutional compliance maintained throughout
- [x] MetaMask compatibility testing included