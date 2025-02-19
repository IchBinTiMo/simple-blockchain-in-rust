use std::collections::HashMap;
use std::io::{Error, ErrorKind};


use crate::block::Block;

pub struct Blockchain {
    blocks_map: HashMap<u32, Block>,
    blocks_vec: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blocks_map: HashMap<u32, Block> = HashMap::new();
        let mut blocks_vec: Vec<Block> = Vec::new();

        let genesis: Block = Block::default();

        blocks_map.insert(genesis.get_index(), genesis.clone());
        blocks_vec.push(genesis);

        Blockchain {
            blocks_map,
            blocks_vec
        }
    }

    fn is_block_valid(&self, current: &Block, prev: &Block) -> bool {
        current.get_index() == prev.get_index() + 1 && 
        current.get_previous_hash() == prev.get_hash() && 
        current.get_hash() == current.calculate_hash()
    }

    pub fn add_block(&mut self, data: i32) -> Result<&str, Error> {
        let prev: &Block = self.blocks_map.get(&(self.len() as u32 - 1)).unwrap();

        let new_block: Block = Block::new(data, prev);

        if !self.is_block_valid(&new_block, prev) {
            println!("Block not valid");
            Err(Error::new(ErrorKind::Other, "Block not valid"))
        } else {
            self.blocks_map.insert(new_block.get_index(), new_block.clone());
            self.blocks_vec.push(new_block);
            println!("Block added");
            Ok("Block added")
        }
    }

    fn len(&self) -> usize {
        self.blocks_map.len()
    }

    pub fn get_chain(&self) -> Vec<Block> {
        self.blocks_vec.clone()
    }
}