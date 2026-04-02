pub enum CdcOp {
    Insert(String),
    Delete(i64),
}

pub fn connect_to_s3() -> Result<(), String> {
    return Ok(());
}
