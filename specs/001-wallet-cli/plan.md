# Implementation Plan: Web3 Wallet CLI Tool

**Branch**: `001-wallet-cli` | **Date**: 2025-01-21 | **Spec**: [link](./spec.md)
**Input**: Feature specification from `/specs/001-wallet-cli/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → SUCCESS: Feature spec loaded successfully
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from context (single=CLI tool)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Professional-grade Web3 wallet CLI tool for secure generation, import, storage, and management of Ethereum addresses using BIP39/BIP44 standards with AES-256-GCM encryption and HD wallet support compatible with MetaMask.

## Technical Context
**Language/Version**: Rust 2021 Edition
**Primary Dependencies**: ethers-rs 2.0, bip39 2.0, clap 4.0, aes-gcm 0.10, pbkdf2 0.12, tokio 1.0
**Storage**: Encrypted JSON keystore files (local filesystem)
**Testing**: cargo test with custom security test suite
**Target Platform**: Cross-platform CLI (Windows, macOS, Linux)
**Project Type**: single
**Performance Goals**: <1s command response time, <50MB memory usage
**Constraints**: Cryptographic security compliance, MetaMask compatibility
**Scale/Scope**: Individual developer tool, handles multiple wallet files

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Security-First Development
✅ **PASS**: Uses industry-standard libraries (ethers-rs, bip39, aes-gcm)
✅ **PASS**: AES-256-GCM encryption for private keys and mnemonics
✅ **PASS**: BIP39 standard with cryptographically secure RNG
✅ **PASS**: zeroize library for memory cleanup after sensitive operations

### II. Error Handling Excellence
✅ **PASS**: All functions return Result<T, WalletError> types
✅ **PASS**: Comprehensive error types for each failure mode
✅ **PASS**: User-friendly error messages without internal details
✅ **PASS**: No panic conditions in production code paths

### III. Performance & Memory Safety
✅ **PASS**: Rust ownership system prevents memory corruption
✅ **PASS**: Tokio async runtime for I/O operations
✅ **PASS**: 1-second response time requirement
✅ **PASS**: Memory usage tracking and optimization

### IV. User Experience & CLI Design
✅ **PASS**: Clap framework provides intuitive CLI interface
✅ **PASS**: Confirmation prompts for sensitive operations
✅ **PASS**: Hidden password input using rpassword
✅ **PASS**: Input validation before processing

### V. Dependency Management
✅ **PASS**: All dependencies are well-maintained with security audit history
✅ **PASS**: No custom cryptographic implementations
✅ **PASS**: Version pinning in Cargo.toml
✅ **PASS**: Regular security dependency updates planned

## Project Structure

### Documentation (this feature)
```
specs/001-wallet-cli/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 1: Single project (DEFAULT)
src/
├── models/              # Wallet, Keystore, Address structs
├── services/            # Crypto, Storage, Validation services
├── cli/                 # Command handlers and output formatting
└── lib/                 # Public API and utilities

tests/
├── contract/            # Command interface tests
├── integration/         # End-to-end CLI tests
└── unit/                # Individual module tests

Cargo.toml               # Dependencies and project metadata
```

**Structure Decision**: Option 1 (Single project) - CLI tool with library structure

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - Research best practices for secure key derivation in Rust
   - Investigate MetaMask keystore format compatibility
   - Study BIP44 derivation path implementations
   - Evaluate entropy sources for secure random generation

2. **Generate and dispatch research agents**:
   ```
   Task: "Research Rust cryptographic libraries for Web3 wallet security"
   Task: "Find best practices for HD wallet implementation in ethers-rs"
   Task: "Study MetaMask keystore format for compatibility requirements"
   Task: "Research secure password handling in CLI applications"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all technical decisions documented

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Wallet: mnemonic, master_key, derivation_path, metadata
   - Keystore: encrypted_data, salt, nonce, kdf_params
   - Address: ethereum_address, private_key, derivation_index
   - Command: subcommand, parameters, output_format

2. **Generate API contracts** from functional requirements:
   - CLI command interface specifications
   - Error type definitions and handling
   - Output format schemas (JSON/human-readable)
   - File format specifications for keystores

3. **Generate contract tests** from contracts:
   - Command parsing and validation tests
   - Error handling and edge case tests
   - Output format verification tests
   - Cross-platform compatibility tests

4. **Extract test scenarios** from user stories:
   - Each acceptance scenario → integration test
   - Quickstart test validates primary user flow

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/powershell/update-agent-context.ps1 -AgentType claude`
   - Add Rust Web3 development context
   - Include cryptographic security considerations
   - Maintain under 150 lines for efficiency

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs
- Each contract → contract test task [P]
- Each entity → model creation task [P]
- Each CLI command → implementation task
- Security tests for encryption/decryption flows

**Ordering Strategy**:
- TDD order: Tests before implementation
- Dependency order: Models → Services → CLI → Integration
- Mark [P] for parallel execution (independent modules)

**Estimated Output**: 20-25 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)
**Phase 4**: Implementation (execute tasks.md following constitutional principles)
**Phase 5**: Validation (run tests, execute quickstart.md, security validation)

## Complexity Tracking
*No constitutional violations identified*

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none identified)

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*