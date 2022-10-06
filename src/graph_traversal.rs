pub struct Graph {
    num_of_nodes: usize,
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(num_of_nodes: usize) -> Self {
        Self {
            num_of_nodes,
            adjacency_list: vec![vec![]; num_of_nodes],
        }
    }

    pub fn add_adjacency(&mut self, source: usize, dest: usize) {
        if source > self.num_of_nodes - 1 || dest > self.num_of_nodes - 1 {
            return;
        }

        self.adjacency_list[source].push(dest);
        self.adjacency_list[dest].push(source);

        // Sorting to guarantee min-to-max index traversal
        self.adjacency_list[source].sort();
        self.adjacency_list[dest].sort();
    }

    pub fn breadth_first_traverse(&self) -> Vec<usize> {
        if self.num_of_nodes == 0 {
            return vec![];
        }

        let mut result = vec![];
        let mut queue: Vec<usize> = vec![0];
        let mut visited: Vec<bool> = vec![false; self.num_of_nodes];

        while !queue.is_empty() {
            let node = queue.remove(0);

            if !visited[node] {
                result.push(node);
                visited[node] = true;

                self.adjacency_list[node].iter().for_each(|e| {
                    queue.push(*e);
                });
            }
        }

        result
    }

    pub fn depth_first_traverse(&self) -> Vec<usize> {
        fn depth_first_traverse_recursive(node: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, result: &mut Vec<usize>) {
            result.push(node);
            visited[node] = true;

            adj_list[node].iter().for_each(|e| {
                if !visited[*e] {
                    depth_first_traverse_recursive(*e, adj_list, visited, result);
                }
            });
        }

        if self.num_of_nodes == 0 {
            return vec![];
        }

        let mut result = vec![];
        let mut visited: Vec<bool> = vec![false; self.num_of_nodes];
        depth_first_traverse_recursive(0, &self.adjacency_list, &mut visited, &mut result);

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
        let mut g = Graph::new(9);
        g.add_adjacency(0, 1);
        g.add_adjacency(0, 3);
        g.add_adjacency(2, 3);
        g.add_adjacency(3, 4);
        g.add_adjacency(3, 5);
        g.add_adjacency(4, 6);
        g.add_adjacency(2, 8);
        g.add_adjacency(6, 7);

        assert_eq!(g.breadth_first_traverse(), [0, 1, 3, 2, 4, 5, 8, 6, 7]);
        assert_eq!(g.depth_first_traverse(), [0, 1, 3, 2, 8, 4, 6, 7, 5]);
    }

    #[test]
    fn test_edge_cases() {
        let g = Graph::new(0);
        assert_eq!(g.breadth_first_traverse(), []);
        assert_eq!(g.depth_first_traverse(), []);

        let g = Graph::new(1);
        assert_eq!(g.breadth_first_traverse(), [0]);
        assert_eq!(g.depth_first_traverse(), [0]);

        let mut g = Graph::new(2);
        g.add_adjacency(0, 1);
        assert_eq!(g.breadth_first_traverse(), [0, 1]);
        assert_eq!(g.depth_first_traverse(), [0, 1]);
    }
}
