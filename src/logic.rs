use rand::seq::SliceRandom;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;

use log::info;

use crate::{Battlesnake, Board, Coord, Game};

pub fn get_info() -> JsonValue {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "nomad",
        "color": "#F09383",
        "head": "default",
        "tail": "default",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn get_move(game: &Game, _turn: &u32, board: &Board, you: &Battlesnake) -> &'static str {
    let mut possible_moves: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // Step 0: Don't let your Battlesnake move back in on its own neck
    let my_head = &you.head;
    let my_neck = &you.body[1];
    if my_neck.x < my_head.x {
        // my neck is left of my head
        possible_moves.insert("left", false);
    } else if my_neck.x > my_head.x {
        // my neck is right of my head
        possible_moves.insert("right", false);
    } else if my_neck.y < my_head.y {
        // my neck is below my head
        possible_moves.insert("down", false);
    } else if my_neck.y > my_head.y {
        // my neck is above my head
        possible_moves.insert("up", false);
    }

    // TODO: Step 1 - Don't hit walls.
    // Use board information to prevent your Battlesnake from moving beyond the boundaries of the board.
    let left = |head: &Coord| Coord {
        x: head.x - 1,
        y: head.y,
    };
    let right = |head: &Coord| Coord {
        x: head.x + 1,
        y: head.y,
    };
    let up = |head: &Coord| Coord {
        x: head.x,
        y: head.y + 1,
    };
    let down = |head: &Coord| Coord {
        x: head.x,
        y: head.y - 1,
    };
    possible_moves.insert("left", valid_move(&left(my_head), &board));
    possible_moves.insert("right", valid_move(&right(my_head), &board));
    possible_moves.insert("up", valid_move(&up(my_head), &board));
    possible_moves.insert("down", valid_move(&down(my_head), &board));

    // TODO: Step 2 - Don't hit yourself.
    // Use body information to prevent your Battlesnake from colliding with itself.
    let body = &you.body;

    // TODO: Step 3 - Don't collide with others.
    // Use snake vector to prevent your Battlesnake from colliding with others.
    let snakes = &board.snakes;

    // TODO: Step 4 - Find food.
    // Use board information to seek out and find food.
    let food = &board.food;

    // Finally, choose a move from the available safe moves.
    // TODO: Step 5 - Select a move to make based on strategy, rather than random.
    let moves = possible_moves
        .into_iter()
        .filter(|&(_, v)| v == true)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    let chosen = moves.choose(&mut rand::thread_rng()).unwrap();

    info!("{} MOVE {}", game.id, chosen);

    return chosen;
}

fn valid_move(spot: &Coord, board: &Board) -> bool {
    let board_width = board.width;
    let board_height = board.height;

    match spot {
        Coord { y: 0, .. } => false,
        Coord { x: 0, .. } => false,
        Coord { y, .. } if y == &board_width => false,
        Coord { x, .. } if x == &board_height => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use crate::Coord;

    use super::*;

    #[test]
    fn head_will_not_hit_left_wall() {
        let board = Board {
            height: 10,
            width: 10,
            food: vec![],
            snakes: vec![],
            hazards: vec![],
        };
        let spot = Coord { x: 0, y: 5 };
        let valid_move = valid_move(&spot, &board);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn head_will_not_hit_right_wall() {
        let board = Board {
            height: 10,
            width: 10,
            food: vec![],
            snakes: vec![],
            hazards: vec![],
        };
        let spot = Coord { x: 10, y: 5 };
        let valid_move = valid_move(&spot, &board);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn head_will_not_hit_roof() {
        let board = Board {
            height: 10,
            width: 10,
            food: vec![],
            snakes: vec![],
            hazards: vec![],
        };
        let spot = Coord { x: 5, y: 10 };
        let valid_move = valid_move(&spot, &board);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn head_will_not_hit_floor() {
        let board = Board {
            height: 10,
            width: 10,
            food: vec![],
            snakes: vec![],
            hazards: vec![],
        };
        let spot = Coord { x: 5, y: 0 };
        let valid_move = valid_move(&spot, &board);
        assert_eq!(valid_move, false);
    }

    #[test]
    fn head_will_travel() {
        let board = Board {
            height: 10,
            width: 10,
            food: vec![],
            snakes: vec![],
            hazards: vec![],
        };
        let spot = Coord { x: 5, y: 5 };
        let valid_move = valid_move(&spot, &board);
        assert_eq!(valid_move, true);
    }
}
