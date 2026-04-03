mod day2;
use std::vec;

use day2::CdcOp;

fn parse_input(bytes: Vec<u8>) {
    // take the first byte from bytes
    let first_byte: u8 = bytes[0];
    let payload: &[u8] = &bytes[1..];
    // convert payload to string
    let payload: std::borrow::Cow<'_, str> = String::from_utf8_lossy(payload);
    let cdcop = match first_byte {
        1 => CdcOp::Insert(payload.to_string()),
        2 => CdcOp::Update(payload.to_string()),
        3 => CdcOp::Delete(payload.to_string().parse::<i64>().unwrap_or(0)),
        _ => {
            println!("First byte is something else");
            // Return a default value to satisfy the return type
            CdcOp::Insert("default".to_string())
        }
    };
    batch_write_to_s3(vec![cdcop]);
}

fn batch_write_to_s3(cdcops: Vec<CdcOp>) {
    println!("Batch writing to S3...");
    for cdc_op in cdcops {
        println!("Executing: {:?}", cdc_op);
    }
}

fn main() {
    let input: Vec<u8> = vec![1, 2, 3, 4, 5];
    parse_input(input);
}
