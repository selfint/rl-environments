enum JumpEnvironmentTile {
    Empty,
    Ground,
    Player,
    Wall,
}

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
}

impl JumpEnvironment {
    pub fn new(size: usize) -> Self {
        assert!(size > 5, "size must be greater than 5");

        let w = size - 1;
        let ground_height = 2;
        let walls = vec![(w, ground_height + 1), (w, ground_height + 2)];
        let state = JumpEnvironment::generate_initial_state(size, &walls, ground_height);

        Self { size, state, walls }
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
}
