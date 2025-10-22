use std::io::{self, Write};

fn main() {
    println!("棋盘游戏 - 终端版本");
    println!("==================");
    println!("游戏规则:");
    println!("- 红方先行，双方轮流放置棋子");
    println!("- 当格子棋子达到容量上限时会分裂");
    println!("- 消灭对方所有棋子获胜");
    println!();
    
    let mut rounds = 0;
    let mut current_player = "红方";
    
    println!("游戏开发中...");
    println!("当前回合: {}", rounds);
    println!("当前玩家: {}", current_player);
    println!();
    print!("请输入坐标 (格式: x y): ");
    
    io::stdout().flush().unwrap();
    
    // 简单的输入测试
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    println!("你输入了: {}", input.trim());
}
