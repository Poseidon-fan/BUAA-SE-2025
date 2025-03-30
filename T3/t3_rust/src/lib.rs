use board::{Board, Food};
use point::{Direction, Point};
use snake::Snake;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

mod point;
mod board;
mod snake;

#[wasm_bindgen]
pub fn greedy_snake_step(
    length: i32,
    my_snake_pos: &[i32],
    other_snake_num: i32,
    other_snake_pos: &[i32],
    food_num: i32,
    foods_pos: &[i32],
    _round: i32,
) -> i32 {
    // 初始化
    let my_snake = Snake::from(my_snake_pos);
    let mut other_snakes = Vec::new();
    for i in 0..other_snake_num {
        let start = i * 8;
        let end = start + 8;
        let snake = Snake::from(&other_snake_pos[start as usize..end as usize]);
        other_snakes.push(snake);
    }
    let mut foods = Vec::new();
    for i in 0..food_num {
        let start = i * 2;
        let end = start + 1;
        let food = Food::from((
            foods_pos[start as usize],
            foods_pos[end as usize],
        ));
        foods.push(food);
    }
    let board = Board::new(length, other_snakes, foods);

    // 收集可能的方向
    const ALL_DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Left, Direction::Down, Direction::Right];
    let posible_directions: Vec<Direction> = ALL_DIRECTIONS
        .iter()
        .filter(|&&d| d != Direction::from(
            (my_snake.head(), my_snake.body_segment(1).unwrap())
        ))
        .cloned()
        .collect();

    let mut choices = Vec::new();
    for d in &posible_directions {
        let new_head = my_snake.head().move_to(*d);
        if !(new_head.x >= 1 && new_head.x <= board.length && new_head.y >= 1 && new_head.y <= board.length) {
            continue;
        }
        if !board.possible_barriers().contains(&my_snake.head().move_to(*d)) {
            choices.push(*d);
        }
    }

    if !choices.is_empty() {
        return choices
            .iter()
            .map(|&d| (d, calc_distance(&board, &my_snake.head().move_to(d))))
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0 as i32;      
    }
    
    for d in &posible_directions {
        let new_head = my_snake.head().move_to(*d);
        if !(new_head.x >= 1 && new_head.x <= board.length && new_head.y >= 1 && new_head.y <= board.length) {
            continue;
        }
        if !board.absolute_barriers().contains(&my_snake.head().move_to(*d)) {
            return *d as i32;
        }
    }
    0
}

fn calc_distance(board: &Board, target: &Point) -> i32 {
    board.foods
        .iter()
        .map(|f| f.point().distance(target))
        .min() 
        .unwrap()
}
