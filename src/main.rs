mod day1;
mod day2;

use day1::*;
use day2::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let greet = "Hello".to_string() + " world";

    the_borrow(&greet);
    the_borrow(&greet);

    the_move(greet);
    // send_to_writer(greet);

    let mut iceberg_properties = std::collections::HashMap::new();
    mutable_mutation(&mut iceberg_properties);
    mutable_mutation(&mut iceberg_properties);

    let cdc_op = CdcOp::Insert("data".to_string());

    match cdc_op {
        CdcOp::Insert(data) => println!("Insert operation with data: {}", data),
        CdcOp::Delete(id) => println!("Delete operation with id: {}", id),
    }

    let olake_version = iceberg_properties.get("olake_version");
    // I can't have this value declared in line inise unwrap_or because it needs to be a reference, so I need to declare it outside and then pass a reference to it.
    let fallback_version = "0.4.2".to_string();
    let olake_version = olake_version.unwrap_or(&fallback_version);
    println!("Olake version: {}", olake_version);

    Ok(())
}
