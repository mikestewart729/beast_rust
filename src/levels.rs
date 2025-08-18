use std::fmt;

pub struct LevelConfig {
    pub block_count: usize,
    pub static_block_count: usize,
}

#[derive(Debug)]
pub enum Level {
    One,
    Two, 
    Three,
}

// Handle this more gracefully in case we decide to add more levels
impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::One => write!(f, "1"),
            Level::Two => write!(f, "2"), 
            Level::Three => write!(f, "3"),
        }
    }
}

impl Level {
    pub fn get_level_config(&self) -> LevelConfig {
        match self {
            Level::One => LevelConfig {
                block_count: 30,
                static_block_count: 5,
            },
            Level::Two => LevelConfig {
                block_count: 20,
                static_block_count: 10,
            },
            Level::Three => LevelConfig { 
                block_count: 12, 
                static_block_count: 20, 
            },
        }
    }
}