use wasm_bindgen::prelude::*;
use crate::{Board, GameState, Color, BOARD_SIZE};

// 导入 JS 函数
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 用于调试的宏
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Web 游戏包装器
#[wasm_bindgen]
pub struct WebGame {
    board: Board,
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGame {
        unsafe{
            console_log!("创建新游戏");
        }
        WebGame {
            board: Board::new(),
        }
    }

    #[wasm_bindgen]
    pub fn make_move(&mut self, x: usize, y: usize) -> String {
        unsafe {
            console_log!("尝试移动: ({}, {})", x, y);
        }
        
        match self.board.make_move(x, y) {
            Ok(GameState::Playing) => {
                unsafe {
                    console_log!("移动成功，游戏继续");
                }
                "playing".to_string()
            }
            Ok(GameState::RedWins) => {
                unsafe {
                    console_log!("红方获胜！");
                }
                "red_wins".to_string()
            }
            Ok(GameState::BlackWins) => {
                unsafe {
                    console_log!("黑方获胜！");
                }
                "black_wins".to_string()
            }
            Ok(GameState::Draw) => {
                unsafe {
                    console_log!("游戏平局！");
                }
                "draw".to_string()
            }
            Err(e) => {
                unsafe {
                    console_log!("移动失败: {}", e);
                }
                format!("error:{}", e)
            }
        }
    }

    #[wasm_bindgen]
    pub fn get_board_state(&self) -> String {
        let mut result = String::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let color = match self.board.camp[row][col] {
                    Color::Red => "R",
                    Color::Black => "B",
                    Color::Empty => ".",
                };
                result.push_str(&format!("{}{}", color, self.board.num[row][col]));
                if col < BOARD_SIZE - 1 {
                    result.push('|');
                }
            }
            if row < BOARD_SIZE - 1 {
                result.push('\n');
            }
        }
        result
    }

    #[wasm_bindgen]
    pub fn get_game_info(&self) -> String {
        format!("{}|{}|{}|{}", 
                self.board.rounds,
                self.board.redcount,
                self.board.blackcount,
                match self.board.currentplayer {
                    Color::Red => "red",
                    Color::Black => "black",
                    _ => "none"
                }
        )
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.board = Board::new();
        unsafe {console_log!("游戏重置");}
    }
}

impl Default for WebGame {
    fn default() -> Self {
        WebGame::new()
    }
}
