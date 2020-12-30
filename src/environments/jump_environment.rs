use std::cmp;

use rand::Rng;

enum JumpEnvironmentTile {
    Empty,
    Ground,
    Player,
    Wall,
}

const EMPTY_TILE: usize = 0;
const GROUND_TILE: usize = 1;
const PLAYER_TILE: usize = 2;
const WALL_TILE: usize = 3;

impl Into<[u8; 4]> for JumpEnvironmentTile {
    fn into(self) -> [u8; 4] {
        match self {
            JumpEnvironmentTile::Empty => [1, 0, 0, 0],
            JumpEnvironmentTile::Ground => [0, 1, 0, 0],
            JumpEnvironmentTile::Player => [0, 0, 1, 0],
            JumpEnvironmentTile::Wall => [0, 0, 0, 1],
        }
    }
}

impl From<&[u8; 4]> for JumpEnvironmentTile {
    fn from(other: &[u8; 4]) -> Self {
        match other {
            [1, 0, 0, 0] => JumpEnvironmentTile::Empty,
            [0, 1, 0, 0] => JumpEnvironmentTile::Ground,
            [0, 0, 1, 0] => JumpEnvironmentTile::Player,
            [0, 0, 0, 1] => JumpEnvironmentTile::Wall,
            _ => panic!("unknown tile '{:?}'", other),
        }
    }
}

pub struct JumpEnvironment {
    size: usize,
    state: Vec<[u8; 4]>,
    walls: Vec<(usize, usize)>,
    done: bool,
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
    ) -> Vec<[u8; 4]> {
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

    pub fn observe(&self) -> &Vec<[u8; 4]> {
        &self.state
    }

    pub fn step(&mut self, action: usize) {
        let size = self.size;
        let xy_to_i = |xy: &(usize, usize)| xy.0 * size + xy.1;

        // shift walls
        let mut new_walls = Vec::with_capacity(self.walls.len());
        let mut max_wall = None;
        for xy in &self.walls {
            let &(x, y) = xy;
            let i = xy_to_i(xy);

            if x > 0 {
                let new_xy = (x - 1, y);
                let new_i = xy_to_i(&new_xy);

                if self.state[new_i][EMPTY_TILE] == 1 {
                    self.state.swap(i, new_i);
                } else if self.state[new_i][PLAYER_TILE] == 1 {
                    self.state[i] = JumpEnvironmentTile::Empty.into();
                    self.done = true;
                } else {
                    panic!("attempted to set wall on a ground/wall tile");
                }
                new_walls.push(new_xy);

                let (new_x, _) = new_xy;
                if let Some(wall) = max_wall {
                    if new_x > wall {
                        max_wall = Some(new_x);
                    }
                } else {
                    max_wall = Some(new_x);
                }
            } else {
                self.state[i] = JumpEnvironmentTile::Empty.into();
            }
        }

        self.walls = new_walls;

        match max_wall {
            Some(wall) => {
                if wall < self.size - 3 {
                    let offset = rand::thread_rng().gen_range(1..self.size - 3);
                    let w1 = (self.size - 1, self.ground_height + offset);
                    let w2 = (self.size - 1, self.ground_height + offset + 1);
                    self.walls.push(w1);
                    self.walls.push(w2);
                }
            }
            None => {
                let offset = rand::thread_rng().gen_range(1..self.size - 3);
                let w1 = (self.size - 1, self.ground_height + offset);
                let w2 = (self.size - 1, self.ground_height + offset + 1);
                self.walls.push(w1);
                self.walls.push(w2);
            }
        }

        // player actions
        match action {
            0 => {}
            1 => {
                if self.player.1 == self.ground_height + 1 {
                    self.player_vel = 2;
                }
            }
            _ => panic!("got unknown action: '{}' (actions are: 0, 1)", action),
        }

        // update player
        let (px, py) = self.player;
        let player_i = xy_to_i(&self.player);
        let new_py = cmp::max(py as i8 + self.player_vel, (self.ground_height + 1) as i8) as usize;
        let new_player_xy = (px, new_py);
        let new_player_i = xy_to_i(&new_player_xy);

        if self.state[new_player_i][EMPTY_TILE] == 1 {
            self.state.swap(player_i, new_player_i);
        } else if self.state[new_player_i][WALL_TILE] == 1 {
            self.state[player_i] = JumpEnvironmentTile::Empty.into();
            self.state[new_player_i] = JumpEnvironmentTile::Player.into();
            self.done = true;
        }
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
}
