pub mod treasure {
    use std::collections::{BTreeSet, HashSet};

    use rand::seq::IteratorRandom;

    use TypeObstacle::{DEAD, FREE, HINT, OBSTACLE, TREASURE};

    use crate::mod_structure::structure::{ContextPosition, HintRule};
    use crate::mod_structure::structure::*;

    type BoardVec = Vec<Vec<ContextPosition>>;

    static MOVEMENTS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn setup() {
        let hint_north_east = HintRule::new(vec![(-1, 0), (0, 1)], 2);
        let hint_north = HintRule::new(vec![(-1, 0)], 2);
        let hint_east = HintRule::new(vec![(0, 1)], 2);

        let p_hint_north_east = ContextPosition::new(HINT(hint_north_east));
        let p_hint_north = ContextPosition::new(HINT(hint_north));
        let p_hint_east = ContextPosition::new(HINT(hint_east));
        let p_free = ContextPosition::new(FREE);
        let p_dead = ContextPosition::new(DEAD);
        let p_obstacle = ContextPosition::new(OBSTACLE);
        let p_treasure = ContextPosition::new(TREASURE);

        let treasure_map: BoardVec = vec![
            vec![p_dead.clone(), p_free.clone(), p_hint_east.clone(), p_hint_east.clone(), p_treasure.clone()],
            vec![p_free.clone(), p_obstacle.clone(), p_hint_north_east.clone(), p_dead.clone(), p_free.clone()],
            vec![p_free.clone(), p_free.clone(), p_hint_north_east.clone(), p_hint_north_east.clone(), p_free.clone()],
            vec![p_free.clone(), p_obstacle.clone(), p_free.clone(), p_hint_north, p_obstacle.clone()],
            vec![p_free.clone(), p_free.clone(), p_free.clone(), p_free.clone(), p_free.clone()],
        ];

        let mut paths = BTreeSet::new();
        let start: (usize, usize) = (treasure_map.len() - 1, 0);
        let mut visited = HashSet::new();
        search_paths(&treasure_map, start, &mut Vec::new(), &mut paths, &mut visited);

        for path in paths {
            println!("Path: {:?}", path);
        }
    }

    fn search_paths(
        matrix: &BoardVec,
        (x, y): (usize, usize),
        current_path: &mut Vec<(usize, usize)>,
        paths: &mut BTreeSet<Vec<(usize, usize)>>,
        visited: &mut HashSet<(usize, usize)>,
    ) {
        if x >= matrix.len() || y >= matrix[0].len() || visited.contains(&(x, y)) || matches!(&matrix[x][y].obstacle, OBSTACLE) {
            return;
        }

        visited.insert((x, y));
        current_path.push((x, y));

        match &matrix[x][y].obstacle {
            DEAD => {}
            TREASURE => {
                paths.insert(current_path.clone());
            }
            _ => {
                let next_moves = match &matrix[x][y].obstacle {
                    HINT(hint_rule) => hint_rule.directions.clone(),
                    _ => MOVEMENTS.to_vec(),
                };


                let mut rng = rand::thread_rng();
                for &(dx, dy) in next_moves.iter().choose_multiple(&mut rng, next_moves.len()) {
                    let new_x = (x as isize + dx) as usize;
                    let new_y = (y as isize + dy) as usize;
                    if new_x < matrix.len() && new_y < matrix[0].len() {
                        search_paths(matrix, (new_x, new_y), current_path, paths, visited);
                    }
                }
            }
        }
        visited.remove(&(x, y));
        current_path.pop();
    }
}
