pub mod operation;
pub mod rule;
pub mod status;

use crate::block::Block;
use crate::board::operation::Operation;

use rand::Rng;

pub struct Board {
    pub blocks: Vec<Block>,
    pub width: i32,
    pub height: i32,
    pub num_mines: i32,
    pub num_flags: i32,
    pub num_revealed: i32,
    pub num_safe_blocks: i32,
    pub num_blocks: i32,
    pub game_over: bool,
    pub game_won: bool,
}

impl Board {
    pub fn new(width: i32, height: i32, num_mines: i32) -> Board {
        let mut blocks = Vec::new();
        let num_blocks = width * height;
        let num_safe_blocks = num_blocks - num_mines;
        let mut index = 0;
        for _ in 0..num_blocks {
            blocks.push(Block::new());
            blocks[index as usize].index = index;
            index += 1;
        }
        Board {
            blocks: blocks,
            width: width,
            height: height,
            num_mines: num_mines,
            num_flags: 0,
            num_revealed: 0,
            num_safe_blocks: num_safe_blocks,
            num_blocks: num_blocks,
            game_over: false,
            game_won: false,
        }
    }

    pub fn init(&mut self) {
        // generate mines
        for _ in 0..self.num_mines {
            let mut rng = rand::thread_rng();
            let mut index = rng.gen_range(0..self.num_blocks);
            while self.blocks[index as usize].is_mine {
                index = rng.gen_range(0..self.num_blocks);
            }
            self.blocks[index as usize].is_mine = true;
        }

        // calculate adjacent mines
    }

    pub fn reveal_block(&mut self, index: i32) {
        if self.blocks[index as usize].is_revealed {
            return;
        }
        self.blocks[index as usize].is_revealed = true;
        self.num_revealed += 1;
        if self.blocks[index as usize].is_mine {
            self.game_over = true;
            return;
        }
        if self.blocks[index as usize].adjacent_mines == 0 {
            // self.reveal_adjacent_blocks(index);
        }
        if self.num_revealed == self.num_safe_blocks {
            self.game_won = true;
        }
    }

    pub fn flip_flag(&mut self, index: i32) {
        if self.blocks[index as usize].is_revealed {
            return;
        }
        if self.blocks[index as usize].is_flagged {
            self.blocks[index as usize].is_flagged = false;
            self.num_flags -= 1;
        } else {
            self.blocks[index as usize].is_flagged = true;
            self.num_flags += 1;
        }
    }

    pub fn play(&mut self) {
        while !self.game_over && !self.game_won {
            // get user input
            let mut operate: Operation = Operation::new();

            // match user input
            // reveal block
            // flip flag

            // check game over
            if self.num_revealed == self.num_safe_blocks - self.num_mines {
                self.game_won = true;
            }

            if self.game_over {
                // reveal all blocks
            }
        }
    }
}
