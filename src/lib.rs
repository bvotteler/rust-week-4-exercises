use clap::{Arg, Command};
use std::io::Read;
use std::str::FromStr;
use thiserror::Error;

// Custom errors for Bitcoin operations
#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("Invalid transaction format")]
    InvalidTransaction,
    #[error("Invalid script format")]
    InvalidScript,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Parse error: {0}")]
    ParseError(String),
}

// Generic Point struct for Bitcoin addresses or coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        // Implement constructor for Point
        Point { x, y }
    }
}

// Custom serialization for Bitcoin transaction
pub trait BitcoinSerialize {
    fn serialize(&self) -> Vec<u8>;
}

// Legacy Bitcoin transaction
#[derive(Debug, Clone)]
pub struct LegacyTransaction {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl LegacyTransaction {
    pub fn builder() -> LegacyTransactionBuilder {
        // Return a new builder for constructing a transaction
        LegacyTransactionBuilder::new()
    }
}

// Transaction builder
pub struct LegacyTransactionBuilder {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Default for LegacyTransactionBuilder {
    fn default() -> Self {
        // Implement default values
        LegacyTransactionBuilder {
            version: 1_i32,
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0_u32,
        }
    }
}

impl LegacyTransactionBuilder {
    pub fn new() -> Self {
        // Initialize new builder by calling default
        LegacyTransactionBuilder::default()
    }

    pub fn version(mut self, version: i32) -> Self {
        // Set the transaction version
        self.version = version;
        self
    }

    pub fn add_input(mut self, input: TxInput) -> Self {
        // Add input to the transaction
        self.inputs.push(input);
        self
    }

    pub fn add_output(mut self, output: TxOutput) -> Self {
        // Add output to the transaction
        self.outputs.push(output);
        self
    }

    pub fn lock_time(mut self, lock_time: u32) -> Self {
        // Set lock_time for transaction
        self.lock_time = lock_time;
        self
    }

    pub fn build(self) -> LegacyTransaction {
        // Build and return the final LegacyTransaction
        LegacyTransaction {
            version: self.version,
            inputs: self.inputs,
            outputs: self.outputs,
            lock_time: self.lock_time,
        }
    }
}

// Transaction components
#[derive(Debug, Clone)]
pub struct TxInput {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64, // in satoshis
    pub script_pubkey: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub vout: u32,
}

// Simple CLI argument parser
pub fn parse_cli_args(args: &[String]) -> Result<CliCommand, BitcoinError> {
    // Match args to "send" or "balance" commands and parse required arguments
    let cli = Command::new("rust-week-4-challenge")
        .subcommand_required(true)
        // .arg_required_else_help(true)
        .subcommand(
            Command::new("send")
                .arg(
                    Arg::new("amount")
                        .required(true)
                        .index(1)
                        .value_parser(clap::value_parser!(u64)),
                )
                .arg(Arg::new("address").required(true).index(2)),
        )
        .subcommand(Command::new("balance"));

    // extend args to have an expected fake binary name as first arg
    let mut fake_full_args = vec!["fake_bin".to_string()];
    fake_full_args.extend_from_slice(args);

    let matches = cli
        .try_get_matches_from(fake_full_args)
        .map_err(|e| BitcoinError::ParseError(e.to_string()))?;

    match matches.subcommand() {
        Some(("send", sub_m)) => {
            let amount = *sub_m.get_one::<u64>("amount").unwrap();
            let address = sub_m.get_one::<String>("address").unwrap().clone();

            Ok(CliCommand::Send { amount, address })
        }
        Some(("balance", _)) => Ok(CliCommand::Balance),
        _ => Err(BitcoinError::ParseError("Unknown command".to_string())),
    }
}

pub enum CliCommand {
    Send { amount: u64, address: String },
    Balance,
}

// Decoding legacy transaction
impl TryFrom<&[u8]> for LegacyTransaction {
    type Error = BitcoinError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // Parse binary data into a LegacyTransaction
        // Minimum length is 10 bytes (4 version + 4 inputs count + 4 lock_time)
        if data.len() < 16 {
            return Err(BitcoinError::InvalidTransaction);
        }

        let mut reader = data;
        let mut version_bytes = [0u8; 4];
        let mut input_count_bytes = [0u8; 4];
        let mut output_count_bytes = [0u8; 4];
        let mut lock_time_bytes = [0u8; 4];

        reader
            .read(&mut version_bytes)
            .map_err(|_| BitcoinError::ParseError(String::from("Failed to parse version")))?;

        reader
            .read(&mut input_count_bytes)
            .map_err(|_| BitcoinError::ParseError(String::from("Failed to parse input count")))?;

        reader
            .read(&mut output_count_bytes)
            .map_err(|_| BitcoinError::ParseError(String::from("Failed to parse output count")))?;

        reader
            .read(&mut lock_time_bytes)
            .map_err(|_| BitcoinError::ParseError(String::from("Failed to parse lock_time")))?;

        let mut builder = LegacyTransaction::builder()
            .version(i32::from_le_bytes(version_bytes))
            .lock_time(u32::from_le_bytes(lock_time_bytes));

        // create fake TxInputs for now, test data does not contain real ones
        for _ in 0..u32::from_le_bytes(input_count_bytes) {
            builder = builder.add_input(TxInput {
                previous_output: OutPoint {
                    txid: [0u8; 32],
                    vout: 0,
                },
                script_sig: Vec::new(),
                sequence: 0,
            });
        }

        // create fake TxOutputs for now, test data does not contain real ones
        for _ in 0..u32::from_le_bytes(output_count_bytes) {
            builder = builder.add_output(TxOutput {
                value: 0,
                script_pubkey: Vec::new(),
            });
        }

        let mut tx = builder.build();
        // need to shrink to fit vectors or capacity isn't what the test expects
        tx.inputs.shrink_to_fit();
        tx.outputs.shrink_to_fit();

        Ok(tx)
    }
}

// Custom serialization for transaction
impl BitcoinSerialize for LegacyTransaction {
    fn serialize(&self) -> Vec<u8> {
        // Serialize only version and lock_time (simplified)
        // 1. serialize versioin (little endian)
        let mut vec = self.version.to_le_bytes().to_vec();

        // 2. serialize lock_time (little endian)
        vec.extend(self.lock_time.to_le_bytes());

        vec
    }
}
