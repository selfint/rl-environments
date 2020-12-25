use std::cmp::{max, min};

#[derive(Debug)]
enum JumpEnvironmentTile {
    Empty,
    Ground,
    Player,
    Wall,
}

struct JumpEnvironment {
    state: Vec<Vec<JumpEnvironmentTile>>,
    size: usize,
    ground_height: usize,
    player_col: usize,
    player_vel: i8,
    player_height: usize,
    walls: Vec<usize>,
}

impl JumpEnvironment {
    fn new(size: usize) -> Self {
        let ground_height = size / 3;
        let player_col = size / 3;
        let player_height = ground_height + 1;
        let walls: Vec<usize> = vec![size - 1];
        let wall_height = 2;
        let state = (0..size)
            .map(|x| {
                (0..size)
                    .map(|y| {
                        if y == player_height && x == player_col {
                            JumpEnvironmentTile::Player
                        } else if y == ground_height {
                            JumpEnvironmentTile::Ground
                        } else if walls.contains(&x) && y <= wall_height {
                            JumpEnvironmentTile::Wall
                        } else {
                            JumpEnvironmentTile::Empty
                        }
                    })
                    .collect()
            })
            .collect();

        Self {
            state,
            size,
            ground_height,
            player_col,
            player_vel: 0,
            player_height,
            walls,
        }
    }

    fn jump(&mut self) {
        if self.player_height == self.ground_height + 1 {
            self.player_vel = 2;
        }
    }

    fn update(&mut self) {
        let mut new_walls = vec![];
        for i in 0..self.walls.len() {
            let wall = self.walls[i];
            if wall != self.player_col {
                self.state.swap(wall, wall - 1);
                new_walls.push(wall - 1);
            }
        }

        self.walls = new_walls;

        self.update_player_height();
        self.update_player_vel();
    }

    fn update_player_height(&mut self) {
        let new_player_height = max(
            self.ground_height + 1,
            min(
                self.size - 1,
                (self.player_height as i8 + self.player_vel).abs() as usize,
            ),
        );

        self.state[self.player_col].swap(self.player_height, new_player_height);
        self.player_height = new_player_height;
    }

    fn update_player_vel(&mut self) {
        if self.player_height > self.ground_height + 1 {
            self.player_vel -= 1;
        } else {
            self.player_vel = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_is_not_empty() {
        let env = JumpEnvironment::new(5);
        assert!(!env.state.is_empty(), "state is empty");
    }

    #[test]
    fn test_first_col_has_ground_tile() {
        let env = JumpEnvironment::new(5);
        assert!(!env.state.is_empty(), "state is empty");

        let first_col_has_ground_tile = env.state[0]
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
        for (x, tile_col) in env.state.iter().enumerate() {
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
            .state
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
        for _ in 0..10 {
            env.update();
        }
        assert_eq!(env.player_height, initial_player_height);
    }

    #[test]
    fn test_walls_exist() {
        let env = JumpEnvironment::new(5);

        let wall_tile_count: usize = env
            .state
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
        env.state
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
}
