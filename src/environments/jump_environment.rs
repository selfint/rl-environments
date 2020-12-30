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

pub struct JumpEnvironment {
    size: usize,
    state: Vec<[u8; 4]>,
    walls: Vec<(usize, usize)>,
    done: bool,
}

impl JumpEnvironment {
    pub fn new(size: usize) -> Self {
        assert!(size > 5, "size must be greater than 5");

        let w = size - 1;
        let ground_height = 2;
        let walls = vec![(w, ground_height + 1), (w, ground_height + 2)];
        let state = JumpEnvironment::generate_initial_state(size, &walls, ground_height);
        let done = false;

        Self {
            size,
            state,
            walls,
            done,
        }
    }

    fn generate_initial_state(
        size: usize,
        walls: &[(usize, usize)],
        ground_height: usize,
    ) -> Vec<[u8; 4]> {
        let mut state = Vec::with_capacity(size * size);
        let i_to_xy = |i| (i / size, i % size);

        for i in 0..size * size {
            let (x, y) = i_to_xy(i);
            if x == 1 && y == ground_height + 1 {
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
        let mut new_walls = Vec::with_capacity(self.walls.len());
        let size = self.size;
        let xy_to_i = |xy: (usize, usize)| xy.0 * size + xy.1;
        for &xy in &self.walls {
            let (x, y) = xy;
            let i = xy_to_i(xy);

            if x > 0 {
                let new_xy = (x - 1, y);
                let new_i = xy_to_i(new_xy);

                if self.state[new_i][EMPTY_TILE] == 1 {
                    self.state.swap(i, new_i);
                } else if self.state[new_i][PLAYER_TILE] == 1 {
                    self.state[i] = JumpEnvironmentTile::Empty.into();
                    self.done = true;
                } else {
                    panic!("attempted to set wall on a ground/wall tile");
                }
                new_walls.push(new_xy);
            }
        }

        self.walls = new_walls;
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
        for _ in 0..7 {
            env.step(1);
        }

        assert!(env.done);
    }
}
