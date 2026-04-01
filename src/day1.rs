pub fn the_move(data: String) {
    println!("Data sent to writer: {}", data);
}

pub fn the_borrow(data: &String) {
    println!("Data sent to writer: {}", data);
}

pub fn mutable_mutation(properties: &mut std::collections::HashMap<String, String>) {
    properties.insert("olake_version".to_string(), "0.6.2".to_string());
}
