use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;

    let mut block = Block::new(0, now() , vec![0;32],  vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 50
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 7
                }
            ],
        },
    ], 0, difficulty); 

    block.mine();

    println!("Mined {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.updata_with_block(block).expect("faild to add block");

    let mut second_block = Block::new(1, now() , last_hash,  vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "chris".to_owned(),
                    value: 500,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 150,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 12,
                },
            ],
        },
    ], 0, difficulty); 

    second_block.mine();

    println!("Mined {:?}", &second_block);

    last_hash = second_block.hash.clone();

    blockchain.updata_with_block(second_block).expect("faild to add second_block");
}
