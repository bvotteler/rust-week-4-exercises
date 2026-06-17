use rust_week_4_exercises::bitcoin_error::BitcoinError;
use rust_week_4_exercises::*;

#[test]
fn test_point_creation() {
    let point = Point::new(10, 20);
    assert_eq!(point.x, 10);
    assert_eq!(point.y, 20);
}

#[test]
fn test_builder_default() {
    let builder = LegacyTransactionBuilder::default();

    assert_eq!(builder.version, 1);
    assert!(builder.inputs.is_empty());
    assert!(builder.outputs.is_empty());
    assert_eq!(builder.lock_time, 0);
}

#[test]
fn test_builder_new_uses_default() {
    let default_builder = LegacyTransactionBuilder::default();
    let new_builder = LegacyTransactionBuilder::new();

    assert_eq!(default_builder.version, new_builder.version);
    assert_eq!(default_builder.inputs.len(), new_builder.inputs.len());
    assert_eq!(default_builder.outputs.len(), new_builder.outputs.len());
    assert_eq!(default_builder.lock_time, new_builder.lock_time);
}

#[test]
fn test_builder_method_chaining() {
    let builder = LegacyTransactionBuilder::default()
        .version(2)
        .add_input(TxInput {
            previous_output: OutPoint {
                txid: [0; 32],
                vout: 0,
            },
            script_sig: vec![],
            sequence: 0xFFFFFFFF,
        })
        .add_output(TxOutput {
            value: 50_000_000, // 0.5 BTC
            script_pubkey: vec![],
        })
        .lock_time(500_000);

    assert_eq!(builder.version, 2);
    assert_eq!(builder.inputs.len(), 1);
    assert_eq!(builder.outputs.len(), 1);
    assert_eq!(builder.lock_time, 500_000);
}

#[test]
fn test_builder_build() {
    let tx = LegacyTransactionBuilder::default().version(1).build();

    assert_eq!(tx.version, 1);
    assert!(tx.inputs.is_empty());
    assert!(tx.outputs.is_empty());
    assert_eq!(tx.lock_time, 0);
}

#[test]
fn test_transaction_serialization() {
    let tx = LegacyTransaction {
        version: 1,
        inputs: Vec::new(),
        outputs: Vec::new(),
        lock_time: 0,
    };

    let serialized = tx.serialize();
    assert_eq!(serialized.len(), 8); // 4 bytes version + 4 bytes lock_time
}

#[test]
fn test_transaction_decoding() {
    // Version (1) + inputs count (0) + outputs count (0) + lock_time (0)
    let data = [
        1, 0, 0, 0, // version (i32)
        0, 0, 0, 0, // inputs count (u32)
        0, 0, 0, 0, // outputs count (u32)
        0, 0, 0, 0, // lock_time (u32)
    ];
    let tx = LegacyTransaction::try_from(&data[..]).unwrap();
    assert_eq!(tx.version, 1);
    assert_eq!(tx.lock_time, 0);
    assert_eq!(tx.inputs.len(), 0);
    assert_eq!(tx.outputs.len(), 0);
}

#[test]
fn test_transaction_decoding_with_inputs() {
    // Version (1) + inputs count (1) + outputs count (0) + lock_time (0)
    let data = [
        1, 0, 0, 0, // version (i32)
        1, 0, 0, 0, // inputs count (u32)
        0, 0, 0, 0, // outputs count (u32)
        0, 0, 0, 0, // lock_time (u32)
    ];
    let tx = LegacyTransaction::try_from(&data[..]).unwrap();
    assert_eq!(tx.version, 1);
    assert_eq!(tx.inputs.capacity(), 1); // Verify we reserved space
    assert_eq!(tx.lock_time, 0);
}

#[test]
fn test_transaction_decoding_error() {
    let data = [1, 0, 0]; // Too short
    let result = LegacyTransaction::try_from(&data[..]);
    assert!(matches!(result, Err(BitcoinError::InvalidTransaction)));
}

#[test]
fn test_cli_parsing() {
    let args = vec![
        "send".to_string(),
        "1000".to_string(),
        "address".to_string(),
    ];
    let cmd = parse_cli_args(&args).unwrap();

    if let CliCommand::Send { amount, address } = cmd {
        assert_eq!(amount, 1000);
        assert_eq!(address, "address");
    } else {
        panic!("Wrong command variant");
    }
}

#[test]
fn test_cli_parsing_errors() {
    // Test missing args
    let args = vec!["send".to_string()];
    let result = parse_cli_args(&args);
    assert!(matches!(result, Err(BitcoinError::ParseError(_))));

    // Test invalid command
    let args = vec!["invalid".to_string()];
    let result = parse_cli_args(&args);
    assert!(matches!(result, Err(BitcoinError::ParseError(_))));
}

#[test]
fn test_generic_point() {
    let int_point = Point::new(10, 20);
    assert_eq!(int_point.x, 10);
    assert_eq!(int_point.y, 20);

    let float_point = Point::new(10.5, 20.5);
    assert_eq!(float_point.x, 10.5);
    assert_eq!(float_point.y, 20.5);

    let str_point = Point::new("x".to_string(), "y".to_string());
    assert_eq!(str_point.x, "x");
    assert_eq!(str_point.y, "y");
}
