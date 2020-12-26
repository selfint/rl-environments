use std::{thread, time};

use rl_environments::jump_environment::JumpEnvironment;

/// always jumps
fn simple_jumper(_env: &JumpEnvironment) -> bool {
    true
}

fn main() {
    let mut env = JumpEnvironment::new(11);
    let mut rewards = 0;
    while !env.done {
        // clear console and reset cursor
        print!("\x1B[2J\x1B[1;1H");

        println!("{}", &env);
        let jump = simple_jumper(&env);
        if jump {
            env.jump();
        }
        rewards += env.update();
        println!("Rewards={} dead={}", rewards, env.done);
        thread::sleep(time::Duration::from_millis(300));
    }
}
