//! # Web3 Wallet CLI Application
//!
//! Main entry point for the Web3 wallet CLI tool.
//! Provides secure Ethereum wallet management with BIP39/BIP44 compliance.

use clap::{Args, Parser, Subcommand};
use rpassword::prompt_password;
use std::path::PathBuf;
use tracing::{error, info};
use web3wallet_cli::{WalletConfig, WalletError, WalletManager, WalletResult};
use web3wallet_cli::errors::{UserInputError, FileSystemError};

/// Web3 Wallet CLI - Secure Ethereum wallet management
#[derive(Parser)]
#[command(
    name = "wallet",
    version = env!("CARGO_PKG_VERSION"),
    about = "A secure, professional-grade Web3 wallet CLI tool",
    long_about = "Generate, import, and manage Ethereum wallets with BIP39/BIP44 compliance and MetaMask compatibility"
)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "table", global = true)]
    output: OutputFormat,

    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

/// Output format options
#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// Human-readable table format
    Table,
    /// Machine-readable JSON format
    Json,
}

/// Available wallet commands
#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet
    Create(CreateArgs),
    /// Import an existing wallet
    Import(ImportArgs),
    /// Load and display wallet information
    Load(LoadArgs),
    /// List all stored wallets
    List(ListArgs),
    /// Derive addresses from HD wallet
    Derive(DeriveArgs),
}

/// Arguments for wallet creation
#[derive(Args)]
struct CreateArgs {
    /// Number of mnemonic words (12 or 24)
    #[arg(short, long, value_parser = validate_word_count, default_value = "12")]
    words: u8,

    /// Save wallet to file
    #[arg(short, long)]
    save: Option<String>,

    /// Target network
    #[arg(short, long, default_value = "mainnet")]
    network: String,
}

/// Arguments for wallet import
#[derive(Args)]
struct ImportArgs {
    /// BIP39 mnemonic phrase
    #[arg(short, long, conflicts_with = "private_key")]
    mnemonic: Option<String>,

    /// Private key (hex format)
    #[arg(short, long, conflicts_with = "mnemonic")]
    private_key: Option<String>,

    /// Save wallet to file
    #[arg(short, long)]
    save: Option<String>,

    /// Target network
    #[arg(short, long, default_value = "mainnet")]
    network: String,
}

/// Arguments for wallet loading
#[derive(Args)]
struct LoadArgs {
    /// Wallet file path
    filename: String,

    /// Show only address without decrypting private data
    #[arg(short, long)]
    address_only: bool,

    /// Derive specific address index
    #[arg(short, long)]
    derive: Option<u32>,
}

/// Arguments for wallet listing
#[derive(Args)]
struct ListArgs {
    /// Custom wallet directory
    #[arg(short, long)]
    path: Option<std::path::PathBuf>,
}

/// Arguments for address derivation
#[derive(Args)]
struct DeriveArgs {
    /// HD derivation path or index
    path: String,

    /// Source wallet file
    #[arg(short, long)]
    from_file: Option<String>,

    /// Number of addresses to derive
    #[arg(short, long, default_value = "1")]
    count: u32,

    /// Starting index for derivation
    #[arg(short, long, default_value = "0")]
    start_index: u32,
}

/// Validate mnemonic word count
fn validate_word_count(s: &str) -> Result<u8, String> {
    match s.parse::<u8>() {
        Ok(12) | Ok(24) => Ok(s.parse().unwrap()),
        Ok(n) => Err(format!("Word count must be 12 or 24, got {}", n)),
        Err(_) => Err(format!("Invalid number: {}", s)),
    }
}

/// Initialize logging based on verbosity level
fn init_logging(verbose: bool) {
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .without_time()
        .init();
}

#[tokio::main]
async fn main() -> WalletResult<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.verbose);

    // Load configuration
    let config = load_config(cli.config).await?;

    if cli.verbose {
        info!("Starting Web3 Wallet CLI v{}", env!("CARGO_PKG_VERSION"));
    }

    // Execute command
    let result = match cli.command {
        Commands::Create(args) => {
            info!("Creating new wallet...");
            execute_create(args, &config, cli.output).await
        }
        Commands::Import(args) => {
            info!("Importing wallet...");
            execute_import(args, &config, cli.output).await
        }
        Commands::Load(args) => {
            info!("Loading wallet...");
            execute_load(args, &config, cli.output).await
        }
        Commands::List(args) => {
            info!("Listing wallets...");
            execute_list(args, &config, cli.output).await
        }
        Commands::Derive(args) => {
            info!("Deriving addresses...");
            execute_derive(args, &config, cli.output).await
        }
    };

    if let Err(ref err) = result {
        error!("Command failed: {}", err);
        std::process::exit(1);
    }

    result
}

/// Load configuration from file or use defaults
async fn load_config(config_path: Option<std::path::PathBuf>) -> WalletResult<WalletConfig> {
    match config_path {
        Some(path) => {
            // TODO: Implement config file loading
            info!("Loading config from: {}", path.display());
            Ok(WalletConfig::default())
        }
        None => Ok(WalletConfig::default()),
    }
}

/// Execute wallet creation command
async fn execute_create(
    args: CreateArgs,
    config: &WalletConfig,
    output: OutputFormat,
) -> WalletResult<()> {
    let manager = WalletManager::new(config.clone());

    info!("Generating new {}-word mnemonic wallet...", args.words);
    let wallet = manager.create_wallet(args.words).await?;

    // Display wallet information
    match output {
        OutputFormat::Table => {
            println!("\n🎉 Wallet created successfully!");
            println!("Address:  {}", wallet.address());
            println!("Network:  {}", wallet.network());
            println!("Mnemonic: {}", wallet.mnemonic());
            println!("\n⚠️  IMPORTANT: Store your mnemonic phrase safely!");
            println!("   Anyone with access to this phrase can access your wallet.");
        }
        OutputFormat::Json => {
            let output = serde_json::json!({
                "success": true,
                "address": wallet.address(),
                "network": wallet.network(),
                "mnemonic": wallet.mnemonic(),
                "derivation_path": wallet.derivation_path(),
                "created_at": wallet.created_at()
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    // Save wallet if requested
    if let Some(filename) = args.save {
        let password = prompt_password("Enter password to encrypt wallet: ")?;
        let confirm = prompt_password("Confirm password: ")?;

        if password != confirm {
            return Err(WalletError::UserInput(
                UserInputError::PasswordMismatch
            ));
        }

        let wallet_dir = &config.wallet_dir;
        tokio::fs::create_dir_all(wallet_dir).await.map_err(|e| {
            WalletError::FileSystem(FileSystemError::DirectoryNotAccessible {
                path: wallet_dir.display().to_string(),
                details: e.to_string(),
            })
        })?;

        let file_path = wallet_dir.join(format!("{}.json", filename));
        manager.save_wallet(&wallet, &file_path, &password).await?;

        println!("\n💾 Wallet saved to: {}", file_path.display());
    }

    Ok(())
}

/// Execute wallet import command
async fn execute_import(
    args: ImportArgs,
    config: &WalletConfig,
    output: OutputFormat,
) -> WalletResult<()> {
    let manager = WalletManager::new(config.clone());

    let wallet = if let Some(mnemonic) = args.mnemonic {
        info!("Importing wallet from mnemonic...");
        manager.import_from_mnemonic(&mnemonic).await?
    } else if let Some(private_key) = args.private_key {
        info!("Importing wallet from private key...");
        manager.import_from_private_key(&private_key).await?
    } else {
        // Prompt for mnemonic if no input provided
        let mnemonic = prompt_password("Enter mnemonic phrase: ")?;
        manager.import_from_mnemonic(&mnemonic).await?
    };

    // Display wallet information
    match output {
        OutputFormat::Table => {
            println!("\n✅ Wallet imported successfully!");
            println!("Address:  {}", wallet.address());
            println!("Network:  {}", wallet.network());
            if wallet.has_mnemonic() {
                println!("Type:     HD Wallet (BIP44)");
            } else {
                println!("Type:     Private Key Only");
            }
        }
        OutputFormat::Json => {
            let output = serde_json::json!({
                "success": true,
                "address": wallet.address(),
                "network": wallet.network(),
                "has_mnemonic": wallet.has_mnemonic(),
                "derivation_path": wallet.derivation_path(),
                "created_at": wallet.created_at()
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    // Save wallet if requested
    if let Some(filename) = args.save {
        let password = prompt_password("Enter password to encrypt wallet: ")?;
        let confirm = prompt_password("Confirm password: ")?;

        if password != confirm {
            return Err(WalletError::UserInput(
                UserInputError::PasswordMismatch
            ));
        }

        let wallet_dir = &config.wallet_dir;
        tokio::fs::create_dir_all(wallet_dir).await.map_err(|e| {
            WalletError::FileSystem(FileSystemError::DirectoryNotAccessible {
                path: wallet_dir.display().to_string(),
                details: e.to_string(),
            })
        })?;

        let file_path = wallet_dir.join(format!("{}.json", filename));
        manager.save_wallet(&wallet, &file_path, &password).await?;

        println!("\n💾 Wallet saved to: {}", file_path.display());
    }

    Ok(())
}

/// Execute wallet load command
async fn execute_load(
    args: LoadArgs,
    config: &WalletConfig,
    output: OutputFormat,
) -> WalletResult<()> {
    let manager = WalletManager::new(config.clone());

    // Construct file path
    let file_path = if args.filename.contains('/') || args.filename.contains('\\') {
        PathBuf::from(&args.filename)
    } else {
        config.wallet_dir.join(&args.filename)
    };

    info!("Loading wallet from: {}", file_path.display());

    let wallet = if args.address_only {
        // Load keystore without decryption for address only
        let keystore = web3wallet_cli::services::CryptoService::load_keystore(&file_path).await?;

        match output {
            OutputFormat::Table => {
                println!("\n📁 Wallet file: {}", file_path.display());
                println!("Address:  {}", keystore.metadata.address);
                println!("Network:  {}", keystore.metadata.network);
                println!("Created:  {}", keystore.metadata.created_at);
                if let Some(alias) = &keystore.metadata.alias {
                    println!("Alias:    {}", alias);
                }
            }
            OutputFormat::Json => {
                let output = serde_json::json!({
                    "file": file_path.display().to_string(),
                    "address": keystore.metadata.address,
                    "network": keystore.metadata.network,
                    "created_at": keystore.metadata.created_at,
                    "alias": keystore.metadata.alias
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
        return Ok(());
    } else {
        // Load and decrypt wallet
        let password = prompt_password("Enter wallet password: ")?;
        manager.load_wallet(&file_path, &password).await?
    };

    // Display wallet information
    match output {
        OutputFormat::Table => {
            println!("\n🔓 Wallet loaded successfully!");
            println!("Address:  {}", wallet.address());
            println!("Network:  {}", wallet.network());
            if wallet.has_mnemonic() {
                println!("Type:     HD Wallet (BIP44)");
            } else {
                println!("Type:     Private Key Only");
            }
            if let Some(alias) = wallet.alias() {
                println!("Alias:    {}", alias);
            }
            println!("Created:  {}", wallet.created_at().format("%Y-%m-%d %H:%M:%S UTC"));
        }
        OutputFormat::Json => {
            let output = serde_json::json!({
                "success": true,
                "address": wallet.address(),
                "network": wallet.network(),
                "has_mnemonic": wallet.has_mnemonic(),
                "derivation_path": wallet.derivation_path(),
                "alias": wallet.alias(),
                "created_at": wallet.created_at()
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    // Derive specific address if requested
    if let Some(index) = args.derive {
        if !wallet.has_mnemonic() {
            return Err(WalletError::UserInput(
                UserInputError::InvalidParameters {
                    parameter: "derive".to_string(),
                    value: index.to_string(),
                    expected: "HD wallet with mnemonic".to_string(),
                }
            ));
        }

        let derived = wallet.derive_address(index)?;

        match output {
            OutputFormat::Table => {
                println!("\n🔗 Derived address [{}]:", index);
                println!("Address:  {}", derived.address());
                println!("Path:     {}", derived.derivation_path());
            }
            OutputFormat::Json => {
                let output = serde_json::json!({
                    "derived": {
                        "index": index,
                        "address": derived.address(),
                        "derivation_path": derived.derivation_path()
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
    }

    Ok(())
}

/// Execute wallet list command
async fn execute_list(
    args: ListArgs,
    config: &WalletConfig,
    output: OutputFormat,
) -> WalletResult<()> {
    let wallet_dir = args.path.unwrap_or_else(|| config.wallet_dir.clone());

    info!("Scanning wallet directory: {}", wallet_dir.display());

    // Create directory if it doesn't exist
    if !wallet_dir.exists() {
        tokio::fs::create_dir_all(&wallet_dir).await.map_err(|e| {
            WalletError::FileSystem(FileSystemError::DirectoryNotAccessible {
                path: wallet_dir.display().to_string(),
                details: e.to_string(),
            })
        })?;

        match output {
            OutputFormat::Table => {
                println!("\n📂 Wallet directory: {}", wallet_dir.display());
                println!("No wallets found. Directory created.");
            }
            OutputFormat::Json => {
                let output = serde_json::json!({
                    "directory": wallet_dir.display().to_string(),
                    "wallets": []
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
        return Ok(());
    }

    // Read directory and find wallet files
    let mut entries = tokio::fs::read_dir(&wallet_dir).await.map_err(|e| {
        WalletError::FileSystem(FileSystemError::DirectoryNotAccessible {
            path: wallet_dir.display().to_string(),
            details: e.to_string(),
        })
    })?;

    let mut wallets = Vec::new();

    while let Some(entry) = entries.next_entry().await.map_err(|e| {
        WalletError::FileSystem(FileSystemError::DirectoryNotAccessible {
            path: wallet_dir.display().to_string(),
            details: e.to_string(),
        })
    })? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            // Try to load keystore metadata
            match web3wallet_cli::services::CryptoService::load_keystore(&path).await {
                Ok(keystore) => {
                    wallets.push((path.clone(), keystore));
                }
                Err(_) => {
                    // Skip invalid files
                    continue;
                }
            }
        }
    }

    // Display results
    match output {
        OutputFormat::Table => {
            println!("\n📂 Wallet directory: {}", wallet_dir.display());
            println!("Found {} wallet(s):\n", wallets.len());

            if wallets.is_empty() {
                println!("No wallets found.");
            } else {
                println!("{:<20} {:<44} {:<12} {:<20}",
                    "FILENAME", "ADDRESS", "NETWORK", "CREATED");
                println!("{}", "─".repeat(100));

                for (path, keystore) in wallets {
                    let filename = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    let short_addr = if keystore.metadata.address.len() >= 42 {
                        format!("{}...{}",
                            &keystore.metadata.address[..6],
                            &keystore.metadata.address[38..])
                    } else {
                        keystore.metadata.address.clone()
                    };

                    println!("{:<20} {:<44} {:<12} {:<20}",
                        filename,
                        short_addr,
                        keystore.metadata.network,
                        keystore.metadata.created_at[..19].replace('T', " ")
                    );
                }
            }
        }
        OutputFormat::Json => {
            let wallet_list: Vec<_> = wallets.into_iter().map(|(path, keystore)| {
                serde_json::json!({
                    "filename": path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown"),
                    "path": path.display().to_string(),
                    "address": keystore.metadata.address,
                    "network": keystore.metadata.network,
                    "created_at": keystore.metadata.created_at,
                    "alias": keystore.metadata.alias
                })
            }).collect();

            let output = serde_json::json!({
                "directory": wallet_dir.display().to_string(),
                "count": wallet_list.len(),
                "wallets": wallet_list
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

/// Execute address derivation command
async fn execute_derive(
    args: DeriveArgs,
    config: &WalletConfig,
    output: OutputFormat,
) -> WalletResult<()> {
    let manager = WalletManager::new(config.clone());

    // Load wallet if file is specified
    let wallet = if let Some(filename) = args.from_file {
        let file_path = if filename.contains('/') || filename.contains('\\') {
            PathBuf::from(&filename)
        } else {
            config.wallet_dir.join(&filename)
        };

        let password = prompt_password("Enter wallet password: ")?;
        manager.load_wallet(&file_path, &password).await?
    } else {
        // Prompt for mnemonic
        let mnemonic = prompt_password("Enter mnemonic phrase: ")?;
        manager.import_from_mnemonic(&mnemonic).await?
    };

    if !wallet.has_mnemonic() {
        return Err(WalletError::UserInput(
            UserInputError::InvalidParameters {
                parameter: "wallet".to_string(),
                value: "private key only".to_string(),
                expected: "HD wallet with mnemonic".to_string(),
            }
        ));
    }

    // Parse derivation path or index
    let start_index = if args.path.parse::<u32>().is_ok() {
        // Path is a simple index
        args.path.parse::<u32>().unwrap()
    } else {
        // TODO: Parse full derivation path - for now just use start_index
        args.start_index
    };

    let mut derived_addresses = Vec::new();

    // Derive addresses
    for i in 0..args.count {
        let index = start_index + i;
        let derived = wallet.derive_address(index)?;
        derived_addresses.push((index, derived));
    }

    // Display results
    match output {
        OutputFormat::Table => {
            println!("\n🔗 Derived addresses from HD wallet:");
            println!("Base address: {}", wallet.address());
            println!("Base path:    {}\n", wallet.derivation_path());

            println!("{:<6} {:<44} {:<30}",
                "INDEX", "ADDRESS", "DERIVATION PATH");
            println!("{}", "─".repeat(85));

            for (index, derived) in derived_addresses {
                println!("{:<6} {:<44} {:<30}",
                    index,
                    derived.address(),
                    derived.derivation_path()
                );
            }
        }
        OutputFormat::Json => {
            let addresses: Vec<_> = derived_addresses.into_iter().map(|(index, derived)| {
                serde_json::json!({
                    "index": index,
                    "address": derived.address(),
                    "derivation_path": derived.derivation_path()
                })
            }).collect();

            let output = serde_json::json!({
                "base_address": wallet.address(),
                "base_path": wallet.derivation_path(),
                "count": args.count,
                "start_index": start_index,
                "addresses": addresses
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}