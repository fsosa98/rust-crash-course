use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut new_hash_map: HashMap<String, u32> = HashMap::new();
    new_hash_map.insert(address, amount);
    new_hash_map
}
