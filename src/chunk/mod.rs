//! Handles raw data from the database

use super::hash::Hash;

/// A chunk of raw data from the database
pub struct Chunk {
    hash: Hash,
    data: Vec<u8>,
}

impl Chunk {
    pub fn from_bytes<'a>(bytes: &'a [u8]) -> Self {
        unimplemented!()
    }
}