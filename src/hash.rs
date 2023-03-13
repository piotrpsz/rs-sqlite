#![allow(dead_code)]
#![allow(unused_imports)]
/*
 * Copyright (C) 2023 Piotr Pszczółkowski
 * Licence: GNU v2
 *
 * E-mail: piotr@beesoft.pl
 *
 * Project: rs-sqlite
 * File: hash.rs
 */
// use argon2::{self, Config};
use fxhash::hash32;

pub fn hash(text: &str) -> u32 {
    hash32(text)
    // let cfg = argon2::Config{
    //     variant: argon2::Variant::Argon2d,
    //     hash_length: std::mem::size_of::<u32>() as u32,
    //     ..Default::default()};
    // let salt = b"1234567890123456";
    // let data = argon2::hash_raw(text.as_bytes(), salt, &cfg).unwrap();
    // u32::from_be_bytes(data[0..4].try_into().unwrap())
}
