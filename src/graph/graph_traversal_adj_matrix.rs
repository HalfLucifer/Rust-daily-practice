pub struct Graph {
    num_of_nodes: usize,
    adjacency_matrix: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(matrix: Vec<Vec<usize>>) -> Self {
        Self {
            num_of_nodes: matrix.len(),
            adjacency_matrix: matrix,
        }
    }

    pub fn breadth_first_traverse(&self) -> Vec<usize> {
        if self.num_of_nodes == 0 {
            return vec![];
        }

        let mut result = vec![];
        let mut queue: Vec<usize> = vec![0]; // Start traversing from node #0
        let mut visited: Vec<bool> = vec![false; self.num_of_nodes];

        while !queue.is_empty() {
            let node = queue.remove(0);

            if !visited[node] {
                result.push(node);
                visited[node] = true;

                self.adjacency_matrix[node]
                    .iter()
                    .enumerate()
                    .for_each(|(i, edge)| {
                        if *edge == 1 && node != i {
                            queue.push(i);
                        }
                    });
            }
        }

        result
    }

    pub fn depth_first_traverse(&self) -> Vec<usize> {
        fn depth_first_traverse_recursive(
            node: usize,
            matrix: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            result: &mut Vec<usize>,
        ) {
            result.push(node);
            visited[node] = true;

            matrix[node].iter().enumerate().for_each(|(i, edge)| {
                if *edge == 1 && !visited[i] {
                    depth_first_traverse_recursive(i, matrix, visited, result);
                }
            });
        }

        if self.num_of_nodes == 0 {
            return vec![];
        }

        let mut result = vec![];
        let mut visited: Vec<bool> = vec![false; self.num_of_nodes];

        // Start traversing from node #0
        depth_first_traverse_recursive(0, &self.adjacency_matrix, &mut visited, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        /*
            1    4 -- 6 -- 7
            |    |
            0 -- 3 -- 5
                 |
                 2 -- 8

            BFS: 0,1,3,2,4,5,8,6,7
            DFS: 0,1,3,2,8,4,6,7,5
        */
        let g = Graph::new(vec![
            vec![1, 1, 0, 1, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0, 1],
        ]);

        assert_eq!(g.breadth_first_traverse(), [0, 1, 3, 2, 4, 5, 8, 6, 7]);
        assert_eq!(g.depth_first_traverse(), [0, 1, 3, 2, 8, 4, 6, 7, 5]);
    }

    #[test]
    fn test_edge_cases() {
        let g = Graph::new(vec![]);
        assert_eq!(g.breadth_first_traverse(), []);
        assert_eq!(g.depth_first_traverse(), []);

        let g = Graph::new(vec![vec![0]]);
        assert_eq!(g.breadth_first_traverse(), [0]);
        assert_eq!(g.depth_first_traverse(), [0]);

        let g = Graph::new(vec![vec![1, 1], vec![1, 1]]);
        assert_eq!(g.breadth_first_traverse(), [0, 1]);
        assert_eq!(g.depth_first_traverse(), [0, 1]);

        let g = Graph::new(vec![vec![1, 1], vec![1, 1]]);
        assert_eq!(g.breadth_first_traverse(), [0, 1]);
        assert_eq!(g.depth_first_traverse(), [0, 1]);
    }

    #[test]
    fn test_all_disconnected_cases() {
        let g = Graph::new(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);

        assert_eq!(g.breadth_first_traverse(), [0]);
        assert_eq!(g.depth_first_traverse(), [0]);
    }

    #[test]
    fn test_all_connected_cases() {
        let g = Graph::new(vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ]);

        assert_eq!(g.breadth_first_traverse(), [0, 1, 2, 3, 4]);
        assert_eq!(g.depth_first_traverse(), [0, 1, 2, 3, 4]);
    }
}
