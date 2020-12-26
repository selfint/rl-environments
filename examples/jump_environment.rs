use std::{thread, time};

use rl_environments::jump_environment::JumpEnvironment;

/// always jumps
fn simple_jumper(_env: &JumpEnvironment) -> bool {
    true
}

fn main() {
    let mut env = JumpEnvironment::new(12);
    let mut score = 0;
    while !env.done {
        let jump = simple_jumper(&env);
        if jump {
            env.jump();
        }
        score += env.update();

        // clear console and reset cursor
        print!("\x1B[2J\x1B[1;1H");

        println!("{}", &env);
        println!("Score={} dead={}", score, env.done);
        thread::sleep(time::Duration::from_millis(100));
    }
}
