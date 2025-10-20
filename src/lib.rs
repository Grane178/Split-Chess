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

    pub fn make_move(&mut self, user_x: usize, user_y: usize) -> Result<GameState, String> {
        // 验证输入范围
        if !(1..=4).contains(&user_x) || !(1..=4).contains(&user_y) {
            return Err("坐标必须在1-4范围内！".to_string());
        }

        // 转换坐标
        let board_x = 4 - user_y;
        let board_y = user_x - 1;

        // 执行移动
        if !self.place_chess(board_x, board_y) {
            return Err("无效的移动！".to_string());
        }

        // 检查游戏状态
        let state = self.get_game_state();
        if state == GameState::Playing {
            self.change_player();
        }

        Ok(state)
    }

    pub fn get_game_state(&self) -> GameState {
        if self.rounds > 2 {
            if self.blackcount == 0 {
                return GameState::RedWins;
            }
            if self.redcount == 0 {
                return GameState::BlackWins;
            }
            if self.rounds > 200 {
                // 防止无限游戏
                return GameState::Draw;
            }
        }
        GameState::Playing
    }

    fn change_player(&mut self) {
        self.currentplayer = match self.currentplayer {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
            _ => Color::Red,
        };
    }

    fn place_chess(&mut self, x: usize, y: usize) -> bool {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return false;
        }

        // 只能在自己的格子或空位上放置
        if self.camp[x][y] != Color::Empty && self.camp[x][y] != self.currentplayer {
            return false;
        }

        self.num[x][y] += 1;
        self.camp[x][y] = self.currentplayer;

        match self.currentplayer {
            Color::Red => self.redcount += 1,
            Color::Black => self.blackcount += 1,
            _ => {}
        }

        self.rounds += 1;

        // 简单的分裂逻辑（后续会完善）
        if self.num[x][y] >= self.maxnum[x][y] {
            // 分裂逻辑待实现
            println!("触发分裂！位置: ({}, {})", x, y);
        }

        true
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
