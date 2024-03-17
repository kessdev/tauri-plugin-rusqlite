use md5;

pub fn calculate_hash(text: &String) -> String {
    let digest = md5::compute(text.as_bytes());
    format!("{:x}", digest)
}