use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

#[wasm_bindgen]
pub fn greedy_snake_move(snake: &[i32], food: &[i32]) -> i32 {
    let head_x = snake[0];
    let head_y = snake[1];
    let food_x = food[0];
    let food_y = food[1];

    console::log_1(&JsValue::from_str(format!("Snake: {:?}", snake).as_str()));
    console::log_1(&JsValue::from_str(format!("Food: {:?}", food).as_str()));
    console::log_1(&JsValue::from_str(format!("{} {}", head_x < food_x, head_y < food_y).as_str()));
    
    match (head_x < food_x, head_y < food_y) {
        (true, true) => {
            if head_x != food_x && is_valid(snake, head_x + 1, head_y) {
                3
            } else if head_y != food_y && is_valid(snake, head_x, head_y + 1) {
                0
            } else {
                default_move(snake, food)
            }
        }
        (false, true) => {
            if head_x != food_x && is_valid(snake, head_x - 1, head_y) {
                1
            } else if head_y != food_y && is_valid(snake, head_x, head_y + 1) {
                0
            } else {
                default_move(snake, food)
            }
        }
        (true, false) => {
            if head_x != food_x && is_valid(snake, head_x + 1, head_y) {
                3
            } else if head_y != food_y && is_valid(snake, head_x, head_y - 1) {
                2
            } else {
                default_move(snake, food)
            }
        }
        (false, false) => {
            if head_x != food_x && is_valid(snake, head_x - 1, head_y) {
                1
            } else if head_y != food_y && is_valid(snake, head_x, head_y - 1) {
                2
            } else {
                default_move(snake, food)
            }
        }
    }
}

// 检查给定坐标是否合法
// 1. 不在蛇的身体上
// 2. 不在边界上
fn is_valid(snake: &[i32], x: i32, y: i32) -> bool {
    // 检查是否超出边界 (0-7)
    if x < 1 || x > 8 || y < 1 || y > 8 {
        return false;
    }
    
    // 检查是否是蛇的身体部分（从第二节开始检查）
    for i in (2..snake.len()-2).step_by(2) {
        if snake[i] == x && snake[i+1] == y {
            return false;
        }
    }
    
    true
}

fn default_move(snake: &[i32], food: &[i32]) -> i32 {
    let head_x = snake[0];
    let head_y = snake[1];
    
    // 尝试四个方向（上、左、下、右）
    let directions = [
        (0, (head_x, head_y + 1)), // 上
        (1, (head_x - 1, head_y)), // 左
        (2, (head_x, head_y - 1)), // 下
        (3, (head_x + 1, head_y)), // 右
    ];
    
    // 按顺序检查每个方向是否合法
    for &(dir, (x, y)) in &directions {
        if is_valid(snake, x, y) {
            return dir;
        }
    }
    
    // 如果所有方向都不合法，默认向上（虽然这种情况不应该发生）
    0
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test_is_valid() {
        let snake = vec![4,4,4,5,4,6,4,7];
        let food = vec![1, 1];
        assert_eq!(greedy_snake_move(&snake, &food), 1);

        let snake = vec![3, 4, 4, 4, 4, 5, 4, 6];
        assert_eq!(greedy_snake_move(&snake, &food), 1);
    }
}