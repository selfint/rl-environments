use std::cmp;

use rand::Rng;

enum JumpEnvironmentTile {
    Empty,
    Ground,
    Player,
    Wall,
}

const GROUND_TILE: usize = 0;
const PLAYER_TILE: usize = 1;
const WALL_TILE: usize = 2;

impl Into<[u8; 3]> for JumpEnvironmentTile {
    fn into(self) -> [u8; 3] {
        match self {
            JumpEnvironmentTile::Empty => [0, 0, 0],
            JumpEnvironmentTile::Ground => [1, 0, 0],
            JumpEnvironmentTile::Player => [0, 1, 0],
            JumpEnvironmentTile::Wall => [0, 0, 1],
        }
    }
}

impl From<&[u8; 3]> for JumpEnvironmentTile {
    fn from(other: &[u8; 3]) -> Self {
        if other[PLAYER_TILE] == 1 {
            JumpEnvironmentTile::Player
        } else if other[WALL_TILE] == 1 {
            JumpEnvironmentTile::Wall
        } else if other[GROUND_TILE] == 1 {
            JumpEnvironmentTile::Ground
        } else {
            JumpEnvironmentTile::Empty
        }
    }
}

#[derive(Clone)]
pub struct JumpEnvironment {
    size: usize,
    pub state: Vec<[u8; 3]>,
    walls: Vec<(usize, usize)>,
    pub done: bool,
    player: (usize, usize),
    player_vel: i8,
    ground_height: usize,
}

impl JumpEnvironment {
    pub fn new(size: usize) -> Self {
        assert!(size > 5, "size must be greater than 5");

        let w = size - 1;
        let ground_height = 2;
        let player = (2, ground_height + 1);
        let walls = vec![(w, ground_height + 1), (w, ground_height + 2)];
        let state = JumpEnvironment::generate_initial_state(size, &walls, ground_height, &player);
        let done = false;

        Self {
            size,
            state,
            walls,
            done,
            player,
            player_vel: 0,
            ground_height,
        }
    }

    fn generate_initial_state(
        size: usize,
        walls: &[(usize, usize)],
        ground_height: usize,
        player: &(usize, usize),
    ) -> Vec<[u8; 3]> {
        let mut state = Vec::with_capacity(size * size);
        let i_to_xy = |i| (i / size, i % size);

        for i in 0..size * size {
            let (x, y) = i_to_xy(i);
            if x == player.0 && y == player.1 {
                state.push(JumpEnvironmentTile::Player.into());
            } else if walls.contains(&(x, y)) {
                state.push(JumpEnvironmentTile::Wall.into());
            } else if y == ground_height {
                state.push(JumpEnvironmentTile::Ground.into());
            } else {
                state.push(JumpEnvironmentTile::Empty.into());
            }
        }

        state
    }

    pub fn observe(&self) -> &Vec<[u8; 3]> {
        &self.state
    }

    fn update_walls(&mut self) {
        let mut new_walls = Vec::with_capacity(self.walls.len());
        let mut max_wall = None;
        let size = self.size;
        let xy_to_i = |xy: &(usize, usize)| xy.0 * size + xy.1;
        for xy in &self.walls {
            let &(x, y) = xy;
            let i = xy_to_i(xy);

            self.state[i][WALL_TILE] = 0;
            if x > 0 {
                let new_xy = (x - 1, y);
                let new_i = xy_to_i(&new_xy);
                self.state[new_i][WALL_TILE] = 1;
                new_walls.push(new_xy);

                let (new_x, _) = new_xy;
                if let Some(wall) = max_wall {
                    if new_x > wall {
                        max_wall = Some(new_x);
                    }
                } else {
                    max_wall = Some(new_x);
                }
            }
        }

        self.walls = new_walls;
        self.spawn_walls(max_wall);
    }

    fn spawn_walls(&mut self, max_wall: Option<usize>) {
        if let Some(wall) = max_wall {
            if wall < self.size - 5 {
                self.add_wall();
            }
        } else {
            self.add_wall()
        }
    }

    fn add_wall(&mut self) {
        let offset = rand::thread_rng().gen_range(1..self.size - 3);
        let w1 = (self.size - 1, self.ground_height + offset);
        let w2 = (self.size - 1, self.ground_height + offset + 1);
        self.walls.extend([w1, w2].iter());
    }

    fn apply_action(&mut self, action: usize) {
        match action {
            0 => {}
            1 => {
                if self.player.1 == self.ground_height + 1 {
                    self.player_vel = 2;
                }
            }
            _ => panic!("got unknown action: '{}' (actions are: 0, 1)", action),
        }
    }

    fn update_player(&mut self) {
        let size = self.size;
        let xy_to_i = |xy: &(usize, usize)| xy.0 * size + xy.1;
        let (px, py) = self.player;
        let player_i = xy_to_i(&self.player);
        let new_py = cmp::max(py as i8 + self.player_vel, (self.ground_height + 1) as i8) as usize;
        let new_player_xy = (px, new_py);
        let new_player_i = xy_to_i(&new_player_xy);

        self.state[player_i][PLAYER_TILE] = 0;
        self.state[new_player_i][PLAYER_TILE] = 1;

        self.player = new_player_xy;
        self.player_vel -= 1;
    }

    fn calculate_reward(&self) -> i8 {
        let walls_on_player_col: Vec<&(usize, usize)> = self
            .walls
            .iter()
            .filter(|&(wx, _)| *wx == self.player.0)
            .collect();
        let passed_wall = !walls_on_player_col.is_empty()
            && walls_on_player_col
                .iter()
                .all(|&(_, wy)| *wy != self.player.1);
        let crashed = !(walls_on_player_col.is_empty() || passed_wall);

        if crashed {
            -1
        } else if passed_wall {
            1
        } else {
            0
        }
    }

    fn check_done(&mut self) {
        if !self.done {
            let size = self.size;
            let xy_to_i = |xy: &(usize, usize)| xy.0 * size + xy.1;

            if self.state[xy_to_i(&self.player)][WALL_TILE] == 1 {
                self.done = true;
            }
        }
    }

    pub fn step(&mut self, action: usize) -> i8 {
        self.update_walls();
        self.apply_action(action);
        self.update_player();
        self.check_done();

        self.calculate_reward()
    }
}

impl std::fmt::Display for JumpEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = vec![];
        for (i, tile) in self.state.iter().enumerate() {
            let tile_str = match JumpEnvironmentTile::from(tile) {
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

            let y = i % self.size;
            if rows.len() < y + 1 {
                let row = tile_str.to_owned();
                rows.push(row);
            } else {
                rows[y].push_str(tile_str);
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
    fn test_size() {
        let size = 6;
        let env = JumpEnvironment::new(size);
        assert_eq!(env.observe().len(), size * size);
    }

    #[test]
    fn test_inaction_kills_player() {
        let mut env = JumpEnvironment::new(6);
        assert!(!env.done);
        for _ in 0..6 {
            env.step(0);
        }

        assert!(env.done);
    }

    #[test]
    fn test_constant_action_kills_player() {
        let mut env = JumpEnvironment::new(7);

        assert!(!env.done);
        for _ in 0..100 {
            env.step(1);
        }

        assert!(env.done);
    }

    #[test]
    fn test_action_moves_player() {
        let mut env = JumpEnvironment::new(7);
        let player_pos = env
            .observe()
            .iter()
            .position(|tile| tile[PLAYER_TILE] == 1)
            .unwrap();
        env.step(1);
        let new_player_pos = env
            .observe()
            .iter()
            .position(|tile| tile[PLAYER_TILE] == 1)
            .unwrap();

        assert_ne!(player_pos, new_player_pos);
    }

    #[test]
    fn test_player_always_exists() {
        let mut env = JumpEnvironment::new(10);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let action = rng.gen_range(0..2);
            env.step(action);
            let mut player_exists = false;
            for tile in env.state.iter() {
                if tile[PLAYER_TILE] == 1 {
                    player_exists = true;
                    break;
                }
            }

            assert!(player_exists);
        }
    }

    #[test]
    fn test_wall_always_exists() {
        let mut env = JumpEnvironment::new(10);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let action = rng.gen_range(0..2);
            env.step(action);
            let mut wall_exists = false;
            for tile in env.state.iter() {
                if tile[WALL_TILE] == 1 {
                    wall_exists = true;
                    break;
                }
            }

            assert!(wall_exists);
        }
    }

    #[test]
    fn test_passing_wall_yields_reward() {
        let mut env = JumpEnvironment::new(10);
        let mut rewards = 0;
        for i in 0..9 {
            let mut action = 0;
            if i == 6 {
                action = 1;
            }
            rewards += env.step(action);
        }

        assert_eq!(rewards, 1);
    }

    #[test]
    fn test_dying_yields_negative_reward() {
        let mut env = JumpEnvironment::new(10);
        let mut rewards = 0;
        for _ in 0..8 {
            let reward = env.step(0);
            rewards += reward;
        }

        assert_eq!(rewards, -1);
    }
}
