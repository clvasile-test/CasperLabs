use crate::bytesrepr::{Error, FromBytes, ToBytes};
use crate::key::Key;
use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Account {
    public_key: [u8; 32],
    nonce: u64,
    known_urefs: BTreeMap<String, Key>,
}

impl Account {
    pub fn new(public_key: [u8; 32], nonce: u64, known_urefs: BTreeMap<String, Key>) -> Self {
        Account {
            public_key,
            nonce,
            known_urefs,
        }
    }

    pub fn insert_urefs(&mut self, keys: &mut BTreeMap<String, Key>) {
        self.known_urefs.append(keys);
    }

    pub fn urefs_lookup(&self) -> &BTreeMap<String, Key> {
        &self.known_urefs
    }

    pub fn get_urefs_lookup(self) -> BTreeMap<String, Key> {
        self.known_urefs
    }

    pub fn pub_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }
}

impl ToBytes for Account {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(&self.public_key.to_bytes());
        result.append(&mut self.nonce.to_bytes());
        result.append(&mut self.known_urefs.to_bytes());
        result
    }
}

impl FromBytes for Account {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (public_key, rem1): ([u8; 32], &[u8]) = FromBytes::from_bytes(bytes)?;
        let (nonce, rem2): (u64, &[u8]) = FromBytes::from_bytes(rem1)?;
        let (known_urefs, rem3): (BTreeMap<String, Key>, &[u8]) = FromBytes::from_bytes(rem2)?;
        Ok((
            Account {
                public_key,
                nonce,
                known_urefs,
            },
            rem3,
        ))
    }
}
