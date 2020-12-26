use rand::Rng;
use std::{
    cmp::{max, min},
    fmt::Display,
};

#[derive(Debug)]
pub enum JumpEnvironmentTile {
    Empty,
    Ground,
    Player,
    Wall,
}

pub struct JumpEnvironment {
    pub size: usize,
    ground_height: usize,
    player_col: usize,
    player_vel: i8,
    player_height: usize,
    walls: Vec<usize>,
    pub done: bool,
    wall_height: usize,
}

impl JumpEnvironment {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            ground_height: size / 3,
            player_col: size / 6,
            player_vel: 0,
            player_height: (size / 3) + 1,
            walls: vec![size - 1],
            done: false,
            wall_height: 2,
        }
    }

    pub fn state(&self) -> Vec<Vec<JumpEnvironmentTile>> {
        (0..self.size)
            .map(|x| {
                (0..self.size)
                    .map(|y| {
                        if y == self.player_height && x == self.player_col {
                            JumpEnvironmentTile::Player
                        } else if y == self.ground_height {
                            JumpEnvironmentTile::Ground
                        } else if self.walls.contains(&x)
                            && self.ground_height < y
                            && y <= self.ground_height + self.wall_height
                        {
                            JumpEnvironmentTile::Wall
                        } else {
                            JumpEnvironmentTile::Empty
                        }
                    })
                    .collect()
            })
            .collect()
    }

    pub fn jump(&mut self) {
        if self.player_height == self.ground_height + 1 {
            self.player_vel = 2;
        }
    }

    pub fn update(&mut self) -> i8 {
        self.spawn_walls();
        self.shift_walls();
        self.update_player_height();
        self.update_player_vel();

        self.calculate_reward()
    }

    fn spawn_walls(&mut self) {
        let mut rng = rand::thread_rng();
        let min_offset = 3;
        let max_offset = self.size / 3;
        let mut random_offset = 1;
        if min_offset < max_offset {
            random_offset = rng.gen_range(min_offset..max_offset);
        }
        if let Some(&wall) = self.walls.iter().max() {
            if wall < self.size - random_offset {
                self.walls.push(self.size - 1);
            }
        }
    }

    fn calculate_reward(&mut self) -> i8 {
        if self.walls.contains(&self.player_col) {
            if self.player_height <= self.wall_height + self.ground_height {
                self.done = true;
                -1
            } else {
                1
            }
        } else {
            0
        }
    }

    fn shift_walls(&mut self) {
        self.walls = self
            .walls
            .iter()
            .filter(|&&w| w > 0)
            .map(|w| w - 1)
            .collect();
    }

    fn update_player_height(&mut self) {
        if self.player_vel != 0 {
            self.player_height = max(
                self.ground_height + 1,
                min(
                    self.size - 1,
                    (self.player_height as i8 + self.player_vel).abs() as usize,
                ),
            );
        }
    }

    fn update_player_vel(&mut self) {
        if self.player_height > self.ground_height + 1 {
            self.player_vel -= 1;
        } else {
            self.player_vel = 0;
        }
    }
}

impl Display for JumpEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = vec![];
        let state = self.state();
        for col in state.iter() {
            for (i, tile) in col.iter().enumerate() {
                let tile_str = match tile {
                    JumpEnvironmentTile::Empty => " ",
                    JumpEnvironmentTile::Ground => "#",
                    JumpEnvironmentTile::Wall => "|",
                    JumpEnvironmentTile::Player => {
                        if self.done {
                            "x"
                        } else if self.player_vel == 0 {
                            "O"
                        } else {
                            "o"
                        }
                    }
                };
                if rows.len() < i + 1 {
                    let row = tile_str.to_owned();
                    rows.push(row);
                } else {
                    rows[i].push_str(tile_str);
                }
            }
        }

        rows.reverse();
        let display_str: String = rows.join("\n");

        write!(f, "{}", display_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_is_not_empty() {
        let env = JumpEnvironment::new(5);
        assert!(!env.state().is_empty(), "state is empty");
    }

    #[test]
    fn test_first_col_has_ground_tile() {
        let env = JumpEnvironment::new(5);
        assert!(!env.state().is_empty(), "state is empty");

        let first_col_has_ground_tile = env.state()[0]
            .iter()
            .any(|t| matches!(t, JumpEnvironmentTile::Ground));

        assert!(
            first_col_has_ground_tile,
            "first col doesn't contain ground tile"
        );
    }

    #[test]
    fn test_ground_height() {
        let env = JumpEnvironment::new(5);
        let mut ground_height: Option<usize> = None;
        for (x, tile_col) in env.state().iter().enumerate() {
            for (y, tile) in tile_col.iter().enumerate() {
                if let JumpEnvironmentTile::Ground = tile {
                    if x == 0 {
                        match ground_height {
                            Some(_) => panic!("Two ground tiles foun in first col"),
                            None => ground_height = Some(y),
                        };
                    } else {
                        match ground_height {
                            Some(h) => assert_eq!(h, y),
                            None => panic!("Two ground tiles foun in first col"),
                        };
                    }
                }
            }
        }
    }

    #[test]
    fn test_only_one_player_exists() {
        let env = JumpEnvironment::new(5);
        let player_tile_count: usize = env
            .state()
            .iter()
            .flatten()
            .map(|t| match t {
                JumpEnvironmentTile::Player => 1,
                _ => 0,
            })
            .sum();

        assert_ne!(player_tile_count, 0, "no player was found");
        assert!(
            player_tile_count < 2,
            "more than one player tiles were found"
        );
    }

    #[test]
    fn test_player_can_jump() {
        let mut env = JumpEnvironment::new(5);
        let initial_player_height = env.player_height;
        env.jump();
        env.update();
        assert!(env.player_height > initial_player_height);
    }

    #[test]
    fn test_player_lands_after_jump() {
        let mut env = JumpEnvironment::new(5);
        let initial_player_height = env.player_height;
        env.jump();

        // ignore player death
        for _ in 0..10 {
            env.update();
        }
        assert_eq!(env.player_height, initial_player_height);
    }

    #[test]
    fn test_walls_exist() {
        let env = JumpEnvironment::new(5);

        let wall_tile_count: usize = env
            .state()
            .iter()
            .flatten()
            .map(|t| match t {
                JumpEnvironmentTile::Wall => 1,
                _ => 0,
            })
            .sum();

        assert_ne!(wall_tile_count, 0, "no walls were found");
    }

    #[test]
    fn test_wall_height_can_be_jumped() {
        let mut env = JumpEnvironment::new(5);
        let max_jump_height = get_max_jump_height(&mut env);
        let max_wall_height = get_max_wall_height(&env);

        assert!(
            max_jump_height > max_wall_height,
            "wall taller than player can jump exists"
        );
    }

    fn get_max_wall_height(env: &JumpEnvironment) -> usize {
        env.state()
            .iter()
            .enumerate()
            .filter_map(|(x, t_col)| {
                if env.walls.contains(&x) {
                    Some(t_col)
                } else {
                    None
                }
            })
            .map(|t_col| {
                t_col
                    .iter()
                    .enumerate()
                    .filter_map(|(y, t)| match t {
                        JumpEnvironmentTile::Wall => Some(y),
                        _ => None,
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }

    fn get_max_jump_height(env: &mut JumpEnvironment) -> usize {
        let mut max_player_height = env.player_height;
        env.jump();
        env.update();
        while env.player_height > max_player_height {
            max_player_height = env.player_height;
            env.update();
        }

        max_player_height
    }

    #[test]
    fn test_walls_shift_left() {
        let mut env = JumpEnvironment::new(200);
        for _ in 0..100 {
            let initial_walls = env.walls.clone();
            env.update();
            let new_walls: Vec<&usize> = env.walls.iter().take(initial_walls.len()).collect();

            for (&initial_wall, &&new_wall) in initial_walls.iter().zip(new_walls.iter()) {
                assert_eq!(initial_wall, new_wall + 1);
            }
        }
    }

    #[test]
    fn test_player_dies_on_wall_collide() {
        let mut env = JumpEnvironment::new(8);
        let steps_to_collision = env.walls.iter().min().unwrap() - env.player_col;
        for _ in 0..steps_to_collision - 1 {
            env.update();
        }

        assert!(!env.done);
        env.update();
        assert!(env.done);
    }

    #[test]
    fn test_walls_respawn() {
        let mut env = JumpEnvironment::new(8);

        for _ in 0..100 {
            if env.walls.contains(&(env.player_col + 2)) {
                env.jump();
            }
            env.update();
            assert!(!env.walls.is_empty());
        }
    }

    #[test]
    fn test_jumping_over_wall_yields_positive_reward() {
        let mut env = JumpEnvironment::new(5);
        env.walls[0] = env.player_col + 1;
        env.player_height = env.wall_height + 2;
        let reward = env.update();

        assert!(reward > 0);
    }

    #[test]
    fn test_colliding_with_wall_yield_negative_reward() {
        let mut env = JumpEnvironment::new(5);
        env.walls[0] = env.player_col + 1;
        let reward = env.update();

        assert!(reward < 0);
    }
}
