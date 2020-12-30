use rand::Rng;
use std::{thread, time};

use rl_environments::jump_environment::JumpEnvironment;

fn main() {
    let mut env = JumpEnvironment::new(12);
    let mut score = 0;
    let mut rng = rand::thread_rng();
    while !env.done {
        let action = rng.gen_range(0..2);
        score += env.step(action);

        // clear console and reset cursor
        print!("\x1B[2J\x1B[1;1H");

        println!("{}", &env);
        println!("Score={} dead={}", score, env.done);
        thread::sleep(time::Duration::from_millis(100));
    }
}
