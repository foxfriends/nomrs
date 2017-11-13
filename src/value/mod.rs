//! Generic representation of a value in the database

mod conversion;
use std::collections::{HashMap, HashSet};
use hash::{Hash, EMPTY_HASH};
use chunk::Chunk;

pub enum Type {
    Boolean,
    Number,
    String,
    Blob,
    Set(Box<Type>),
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Union(HashSet<Type>),
    Ref(Box<Type>),
    Struct(StructType),
    Optional(Box<Type>),
    Type,
}

// TODO: this representation of value is probably wrong, and all of this will just be raw bytes
//       with conversions defined instead
// pub enum Value {
//     Boolean(bool),
//     Number(Vec<u8>),
//     String(String),
//     Blob(Vec<u8>),
//     Set(HashSet<Value>),
//     List(Vec<Value>),
//     Map(HashMap<Value, Value>),
//     Union(Box<Value>),
//     Ref(Ref),
//     Struct(Struct),
//     Optional(Option<Box<Value>>),
//     Type(Type),
// }

pub struct StructType(HashMap<String, Type>);

pub struct Struct(HashMap<String, Chunk>);

#[derive(Debug)]
pub struct Value(pub(crate) Chunk);
impl Value {
    pub fn raw(&self) -> &Vec<u8> {
        self.0.data()
    }
    pub fn into_raw(self) -> Vec<u8> {
        self.0.into_data()
    }
}

#[derive(Debug)]
pub struct Commit {
    meta: Chunk,
    parents: Chunk,
    value: Chunk,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ref {
    hash: Hash,
    // value: Box<Value>,
}
impl Ref {
    pub fn new(hash: Hash) -> Self {
        Self{ hash }
    }
    pub fn is_empty(&self) -> bool {
        self.hash == EMPTY_HASH
    }
    pub fn hash(&self) -> Hash {
        self.hash
    }
}

pub trait IntoNoms {
    fn into_noms(&self) -> Value;
}
pub trait FromNoms {
    fn from_noms(&Value) -> Self;
}
