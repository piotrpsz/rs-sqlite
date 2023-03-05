use argon2::{self, Config};

pub fn hash(text: &str) -> u32 {
    let cfg = argon2::Config{
        variant: argon2::Variant::Argon2d,
        hash_length: std::mem::size_of::<u32>() as u32,
        ..Default::default()};
    let salt = b"1234567890123456";
    let data = argon2::hash_raw(text.as_bytes(), salt, &cfg).unwrap();
    println!("bytes number: {}", data.len());
    u32::from_be_bytes(data[0..4].try_into().unwrap())
}
