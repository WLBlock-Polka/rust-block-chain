use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTranscation,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspend_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            unspend_outputs: HashSet::new()
        }
    }
    pub fn updata_with_block (&mut self, block:Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();
            // 判断区块的index是否正确
        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        } else if !block::check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationErr::InvalidHash);
        } else if i != 0 {
            //  Not block
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            } else if block.prev_block_hash != prev_block.hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return  Err(BlockValidationErr::InvalidCoinbaseTranscation);
            }
            let mut block_spent : HashSet<Hash> = HashSet::new();
            let mut block_created : HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;
            for transaction in transactions {
                let input_hashes = transaction.input_hash();
                if !(&input_hashes - &self.unspend_outputs).is_empty()
                    || (&input_hashes).is_empty() {
                    return Err(BlockValidationErr::InvalidInput);
                }
                let input_value = transaction.input_value();
                let output_value = transaction.output_value();
                if input_value > output_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = output_value - input_value;
                total_fee += fee;
                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hash());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTranscation);
            } else {
                block_created.extend(coinbase.output_hash());
            }
            self.unspend_outputs.retain(|output| !block_spent.contains(output));
            self.unspend_outputs.extend(block_created);

        }

        self.blocks.push(block);
        Ok(())
    }
}