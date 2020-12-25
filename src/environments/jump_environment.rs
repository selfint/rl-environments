#[derive(Debug)]
enum JumpEnvironmentTile {
    Empty,
    Ground,
}

struct JumpEnvironment {
    state: Vec<Vec<JumpEnvironmentTile>>,
}

impl JumpEnvironment {
    fn new(size: usize) -> Self {
        let ground_height = size / 3;
        let mut state = vec![];
        for _ in 0..size {
            let mut col = vec![];
            for y in 0..size {
                if y == ground_height {
                    col.push(JumpEnvironmentTile::Ground);
                } else {
                    col.push(JumpEnvironmentTile::Empty);
                }
            }
            state.push(col);
        }
        Self { state }
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
    fn test_jump_environment_has_one_ground_height() {
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
}
