#[derive(Debug)]
enum JumpEnvironmentTile {
    Empty,
    Ground,
    Player,
}

struct JumpEnvironment {
    state: Vec<Vec<JumpEnvironmentTile>>,
}

impl JumpEnvironment {
    fn new(size: usize) -> Self {
        let ground_height = size / 3;
        let player_x = size / 3;
        let player_y = ground_height + 1;
        let state = (0..size)
            .map(|x| {
                (0..size)
                    .map(|y| {
                        if y == player_y && x == player_x {
                            JumpEnvironmentTile::Player
                        } else if y == ground_height {
                            JumpEnvironmentTile::Ground
                        } else {
                            JumpEnvironmentTile::Empty
                        }
                    })
                    .collect()
            })
            .collect();
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
}
