pub enum CdcOp {
    Insert(String),
    Delete(i64),
}

pub fn connect_to_s3() -> Result<(), String> {
    return Ok(());
}

fn main() {
    let cdc_op = CdcOp::Insert("data".to_string());

    match cdc_op {
        CdcOp::Insert(data) => println!("Insert operation with data: {}", data),
        CdcOp::Delete(id) => println!("Delete operation with id: {}", id),
    }
}
