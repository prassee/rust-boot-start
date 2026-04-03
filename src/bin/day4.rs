mod day2;
use std::vec;

use day2::CdcOp;

fn parse_input(bytes: &Vec<u8>) -> CdcOp {
    // take the first byte from bytes
    let first_byte: u8 = bytes[0];
    let payload: &[u8] = &bytes[1..];
    // convert payload to string
    let payload: std::borrow::Cow<'_, str> = String::from_utf8_lossy(payload);
    match first_byte {
        1 => CdcOp::Insert(payload.to_string()),
        2 => CdcOp::Update(payload.to_string()),
        3 => CdcOp::Delete(payload.to_string().parse::<i64>().unwrap_or(0)),
        _ => {
            println!("First byte is something else");
            // Return a default value to satisfy the return type
            CdcOp::Insert("default".to_string())
        }
    }
}

fn batch_write_to_s3(cdcops: Vec<CdcOp>) {
    println!("Batch writing to S3...");
    for cdc_op in cdcops {
        println!("Executing: {:?}", cdc_op);
    }
}

fn main() {
    // update inputs  with 30 more entries
    let inputs = vec![
        vec![1, 72, 101, 108, 108, 111],                   // "Hello"
        vec![1, 87, 111, 114, 108, 100],                   // "World"
        vec![3, 0, 0, 0, 42],                              // Delete ID 42
        vec![2, 85, 112, 100, 97, 116, 101],               // "Update"
        vec![1, 82, 117, 115, 116],                        // "Rust"
        vec![1, 68, 97, 116, 97],                          // "Data"
        vec![2, 78, 101, 119, 32, 69, 110, 116, 114, 121], // "New Entry"
        vec![3, 0, 0, 0, 99],                              // Delete ID 99
        vec![1, 84, 101, 115, 116],                        // "Test"
        vec![2, 85, 112, 100, 97, 116, 101, 32, 50],       // "Update 2"
        vec![
            1, 82, 117, 115, 116, 32, 76, 97, 110, 103, 117, 97, 103, 101,
        ], // "Rust Language"
        vec![3, 0, 0, 0, 123],                             // Delete ID 123
        vec![1, 68, 97, 116, 97, 32, 83, 99, 105, 101, 110, 99, 101], // "Data Science"
        vec![2, 77, 111, 114, 101, 32, 85, 112, 100, 97, 116, 101], // "More Update"
        vec![
            1, 82, 117, 115, 116, 32, 80, 114, 111, 103, 114, 97, 109, 109, 105, 110, 103,
        ], // "Rust Programming"
    ];
    let mut all_cdc_ops: Vec<CdcOp> = Vec::new();
    let batch_size = 10;
    for input in inputs {
        let cdc_op = parse_input(&input);
        all_cdc_ops.push(cdc_op);
        if all_cdc_ops.len() >= batch_size {
            batch_write_to_s3(all_cdc_ops.clone());
            all_cdc_ops.clear();
        }
    }
    // Write any remaining operations to S3
    if !all_cdc_ops.is_empty() {
        batch_write_to_s3(all_cdc_ops);
    }
}
