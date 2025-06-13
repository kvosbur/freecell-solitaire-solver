use freecell_game_engine::game_state::Move;
use freecell_game_engine::GameState;
use std::time::Instant;

struct Counter {
    count: u64,
    start: Instant,
}

/// Attempts to solve the given FreeCell game state using recursive DFS.
pub fn solve(game: GameState) {
    println!("Solving FreeCell game using strategy 1...");
    let mut path = Vec::new();
    let mut counter = Counter {
        count: 0,
        start: Instant::now(),
    };
    if dfs(game, &mut path, &mut counter) {
        println!("Solution found!");
        for m in path {
            println!("{:?}", m);
        }
    } else {
        println!("No solution found.");
    }
}

fn dfs(game: GameState, path: &mut Vec<Move>, counter: &mut Counter) -> bool {
    if game.is_game_won() {
        return true;
    }
    if path.len() > 200 {
        // Limit the depth to prevent excessive recursion
        return false;
    }
    let moves = game.get_available_moves();
    for m in moves {
        let mut cloned = game.clone();
        if cloned.execute_move(&m).is_ok() {
            path.push(m.clone());
            if dfs(cloned, path, counter) {
                return true;
            }
            path.pop();
        }
    }
    counter.count += 1;
    if counter.count % 100000 == 0 {
        println!(
            "Checked {} game states, time:{:?}",
            counter.count,
            counter.start.elapsed()
        );
    }
    false
}
