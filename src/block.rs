use super::*;

use std::fmt::{ self, Debug, Formatter };

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub prev_block_hash: Hash,
    pub hash: Hash,
    pub transactions: Vec<Transaction>,
    nonce: u64,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {:?} at {} with: {}, nonce: {}", 
            &self.index, 
            &hex::encode(&self.hash), 
            &self.timestamp, 
            &self.transactions.len(),
            &self.nonce
        )
    }
}

impl Block {
    pub  fn new(index: u32, timestamp: u128, prev_block_hash: Hash, transactions: Vec<Transaction>, nonce: u64, difficulty: u128) -> Self {
        Block {
            index,
            timestamp,
            prev_block_hash,
            transactions,
            hash: vec![0; 32],
            nonce,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.transactions.iter().flat_map(|transaction| transaction.bytes()).collect::<Vec<u8>>());

        bytes
    }
}

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}