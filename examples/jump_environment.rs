use std::{thread, time};

use rl_environments::jump_environment::{JumpEnvironment, JumpEnvironmentTile};

/// always jumps
fn simple_jumper(_env: &JumpEnvironment) -> bool {
    true
}

/// displays tile
fn display_tile(tile: &JumpEnvironmentTile) -> &str {
    match tile {
        JumpEnvironmentTile::Empty => " ",
        JumpEnvironmentTile::Ground => "#",
        JumpEnvironmentTile::Player => "O",
        JumpEnvironmentTile::Wall => "|",
    }
}

/// displays jump environment state
fn display_state(env: &JumpEnvironment) {
    let mut rows = vec![];
    for col in &env.state {
        for (i, tile) in col.iter().enumerate() {
            if rows.len() < i + 1 {
                let row = vec![tile];
                rows.push(row);
            } else {
                rows[i].push(tile);
            }
        }
    }

    // clear console and reset cursor
    print!("\x1B[2J\x1B[1;1H");

    for row in rows.iter().rev() {
        for &tile in row {
            let tile_str = display_tile(tile);
            print!("{}", tile_str);
        }
        println!();
    }
}

fn main() {
    let mut env = JumpEnvironment::new(10);
    let mut rewards = 0;
    for _ in 0..100 {
        display_state(&env);
        let jump = simple_jumper(&env);
        if jump {
            env.jump();
        }
        rewards += env.update();
        println!("Rewards={}", rewards);
        thread::sleep(time::Duration::from_millis(100));
    }
}
