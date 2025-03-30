use board::Board;
use point::{Direction, Point};
use snake::Snake;
use wasm_bindgen::prelude::wasm_bindgen;

use std::collections::{VecDeque, HashSet};

mod board;
mod snake;
mod point;

// Import the `console_log` macro
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(snake_pos: &[i32], foods_pos: &[i32], barriers_pos: &[i32]) -> i32 {
    let snake_pos_str = format!("Snake Pos: {:?}", snake_pos);
    let foods_pos_str = format!("Foods Pos: {:?}", foods_pos);
    let barriers_pos_str = format!("Barriers Pos: {:?}", barriers_pos);

    log("-----------------------------");
    log(&snake_pos_str);
    log(&foods_pos_str);
    log(&barriers_pos_str);

    let board = Board::new(8, foods_pos, barriers_pos);
    let snake = Snake::from(snake_pos);

    // 先判断能不能吃到食物
    if let None = bfs(&board, snake.head()) {
        return -1;
    }

    const ALL_DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Left, Direction::Down, Direction::Right];
    let posible_directions: Vec<Direction> = ALL_DIRECTIONS
        .iter()
        .filter(|&&d| d != Direction::from(
            (snake.head(), snake.body_segment(1).unwrap())
        ))
        .cloned()
        .collect();

    posible_directions
        .iter()
        .map(|&d| (d, calc_distance(&board, &snake, d)))
        .filter(|(_, d)| d.is_some())
        .min_by(|(_, d1), (_, d2)| d1.cmp(&d2))
        .map(|(d, _)| d as i32)
        .unwrap_or(-1)
}

fn calc_distance(board: &Board, snake: &Snake, direction: Direction) -> Option<i32> {
    let target_point = snake.head().move_to(direction);
    match is_valid_point(board, &target_point) {
        false => None,
        true => {
            bfs(board, &target_point)
        }
    }
}

fn bfs(board: &Board, point: &Point) -> Option<i32> {
    let food_point = board.foods[0].point();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back((*point, 0));
    visited.insert(*point);
    
    const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];
    
    while let Some((current_point, steps)) = queue.pop_front() {
        if current_point == food_point {
            return Some(steps);
        }
        
        for &dir in &DIRECTIONS {
            let next_point = current_point.move_to(dir);
            
            if is_valid_point(board, &next_point) && !visited.contains(&next_point) {
                visited.insert(next_point);
                queue.push_back((next_point, steps + 1));
            }
        }
    }
    
    None
}

fn is_valid_point(board: &Board, point: &Point) -> bool {
    if !(point.x >= 1 && point.y >= 1 && point.x <= board.length && point.y <= board.length) {
        return false;
    }
    for barrier in &board.barriers {
        if barrier.point().x == point.x && barrier.point().y == point.y {
            return false;
        }
    }
    true
}
