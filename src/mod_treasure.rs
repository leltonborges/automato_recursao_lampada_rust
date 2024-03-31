pub mod treasure {
    use std::collections::VecDeque;

    use rand::{Rng, thread_rng};

    use ContextPosition;
    use TypeObstacle::{DEAD, FREE, OBSTACLE, TREASURE};
    use ObstacleRule;
    use crate::mod_structure::structure::*;

    type BoardVec = Vec<Vec<ContextPosition>>;
    type VisitedVec = Vec<Vec<bool>>;

    static MOVEMENTS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn setup() {
        let p_free = ContextPosition::new(FREE, Box::new(NoneRule));
        let p_dead = ContextPosition::new(DEAD, Box::new(NoneRule));
        let p_obstacle = ContextPosition::new(OBSTACLE, Box::new(RegressMoveRule));
        let p_treasure = ContextPosition::new(TREASURE, Box::new(NoneRule));

        let start: (usize, usize) = (4, 0);

        let treasure_map: BoardVec = vec![
            vec![p_free.clone(), p_free.clone(), p_treasure.clone(), p_free.clone()],
            vec![p_free.clone(), p_dead.clone(), p_free.clone(), p_free.clone()],
            vec![p_free.clone(), p_free.clone(), p_dead.clone(), p_free.clone()],
            vec![p_free.clone(), p_free.clone(), p_free.clone(), p_free.clone()],
            vec![p_free.clone(), p_obstacle.clone(), p_free.clone(), p_obstacle.clone()],
        ];

        let (shortest, paths) = search_for_treasure(&treasure_map, start);
        if let Some(short) = shortest {
            println!("Menor caminho ${:?}", short);
        }

        for path in paths {
            println!("Caminhos: ${:?}", path);
        }
    }

    fn search_for_treasure(matrix: &BoardVec, start: (usize, usize)) -> (Option<(Vec<(usize, usize)>)>,
                                                                         Vec<Vec<(usize, usize)>>) {
        let mut queue = VecDeque::new();
        let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut shortest_path: Vec<(usize, usize)> = Vec::new();
        let mut is_visited = vec![vec![false; matrix[0].len()]; matrix.len()];

        queue.push_back((start, vec![start]));
        is_visited[start.0][start.1] = true;

        while let Some((pos, path)) = queue.pop_front() {
            if matrix[pos.0][pos.1].obstacle == TREASURE {
                paths.push(path.clone());
                if shortest_path.is_empty() || path.len() < shortest_path.len() {
                    shortest_path = path.clone();
                }
            }

            for (dx, dy) in MOVEMENTS.iter() {
                let nx = (pos.0 as isize + dx) as usize;
                let ny = (pos.1 as isize + dy) as usize;

                if nx < matrix.len() && ny < matrix[0].len() && matrix[nx][ny].obstacle != OBSTACLE && !is_visited[nx][ny] {
                    let mut new_path = path.clone();
                    new_path.push((nx, ny));
                    queue.push_back(((nx, ny), new_path));
                    is_visited[nx][ny] = true;
                }
            }
        }

        if !paths.is_empty() {
            (Some(shortest_path), paths)
        } else {
            (None, Vec::new())
        }
    }
}