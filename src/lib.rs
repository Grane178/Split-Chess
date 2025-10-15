pub const BOARD_SIZE: usize = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Empty,
    Red,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    Playing,
    RedWins,
    BlackWins,
    Draw,
}

#[derive(Debug)]
pub struct Board {
    pub camp: [[Color; BOARD_SIZE]; BOARD_SIZE],
    pub num: [[i32; BOARD_SIZE]; BOARD_SIZE],
    pub maxnum: [[i32; BOARD_SIZE]; BOARD_SIZE],
    pub redcount: i32,
    pub blackcount: i32,
    pub rounds: i32,
    pub currentplayer: Color,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            camp: [[Color::Empty; BOARD_SIZE]; BOARD_SIZE],
            num: [[0; BOARD_SIZE]; BOARD_SIZE],
            maxnum: [[0; BOARD_SIZE]; BOARD_SIZE],
            redcount: 0,
            blackcount: 0,
            rounds: 0,
            currentplayer: Color::Red,
        };

        // 初始化最大容量
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let mut checker = 0;
                if i > 0 {
                    checker += 1;
                }
                if j > 0 {
                    checker += 1;
                }
                if i < BOARD_SIZE - 1 {
                    checker += 1;
                }
                if j < BOARD_SIZE - 1 {
                    checker += 1;
                }
                board.maxnum[i][j] = checker;
            }
        }

        board
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
