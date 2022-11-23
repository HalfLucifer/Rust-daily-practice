pub struct Graph {
    num_of_nodes: usize,
    adjacency_list: Vec<Vec<isize>>,
}

/*
    Bellman-Ford shortest path algorithm
    - Time complexity: O(V * E)
    - Space complexity: O(V)
*/
impl Graph {
    pub fn new(num_of_nodes: usize) -> Self {
        Self {
            num_of_nodes: num_of_nodes,
            adjacency_list: vec![],
        }
    }

    pub fn add_edge(&mut self, source: isize, dest: isize, cost: isize) {
        assert!(source < self.num_of_nodes as isize && dest < self.num_of_nodes as isize);

        self.adjacency_list.push(vec![source, dest, cost]);
    }

    pub fn find_shortest_path(&self, source: isize) -> Vec<isize> {
        assert!(source < self.num_of_nodes as isize);

        if self.adjacency_list.is_empty() {
            return vec![];
        }

        let mut result = vec![isize::MAX; self.num_of_nodes];
        result[source as usize] = 0;

        // Update cost by adjacency list V-1 times
        for _ in 1..self.num_of_nodes {
            self.update_cost(&mut result);
        }

        result
    }

    pub fn has_negative_cycle(&self) -> bool {
        let v1 = self.find_shortest_path(0);
        let mut v2 = v1.clone();

        self.update_cost(&mut v2);
        v1 != v2
    }

    fn update_cost(&self, result: &mut Vec<isize>) {
        self.adjacency_list.iter().for_each(|adj| {
            let source = adj[0] as usize;
            let dest = adj[1] as usize;
            let cost = adj[2];

            // Prevent from integer overflow
            if result[source] < isize::MAX {
                if result[dest] > result[source] + cost {
                    result[dest] = result[source] + cost;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        /*
           Masterpiece: draw the diagram with ASCII characters
                    5
           |-----------------|
           v   9         3   |
          [0]----> [1] <----[2]
           |       ^ |       ^
           |       | |       |
         2 |     4 | | 1     |
           | ------  |       |
           v |       v     7 |
          [3]------>[4]-------
                6
        */
        let mut g = Graph::new(5);

        g.add_edge(0, 1, 9);
        g.add_edge(0, 3, 2);
        g.add_edge(1, 4, 1);
        g.add_edge(2, 0, 5);
        g.add_edge(2, 1, 3);
        g.add_edge(3, 1, 4);
        g.add_edge(3, 4, 6);
        g.add_edge(4, 2, 7);

        assert_eq!(g.find_shortest_path(0), [0, 6, 14, 2, 7]);
        assert_eq!(g.find_shortest_path(1), [13, 0, 8, 15, 1]);
        assert_eq!(g.find_shortest_path(2), [5, 3, 0, 7, 4]);
        assert_eq!(g.find_shortest_path(3), [17, 4, 12, 0, 5]);
        assert_eq!(g.find_shortest_path(4), [12, 10, 7, 14, 0]);
    }

    #[test]
    fn test_edge_cases() {
        let mut g = Graph::new(1);
        g.add_edge(0, 0, 100);
        assert_eq!(g.find_shortest_path(0), [0]);

        let mut g = Graph::new(2);
        g.add_edge(0, 1, 1000);
        assert_eq!(g.find_shortest_path(0), [0, 1000]);

        let mut g = Graph::new(3);
        g.add_edge(0, 1, 500);
        g.add_edge(1, 2, 500);
        assert_eq!(g.find_shortest_path(0), [0, 500, 1000]);

        let mut g = Graph::new(3);
        g.add_edge(0, 1, 3000);
        assert_eq!(g.find_shortest_path(0), [0, 3000, isize::MAX]);
    }

    #[test]
    fn test_one_way_cases() {
        let mut g = Graph::new(100);
        for i in 0..99 {
            g.add_edge(i, i + 1, i);
        }

        let expected = (0..100).map(|v| (0..v).sum()).collect::<Vec<_>>();
        assert_eq!(g.find_shortest_path(0), expected);
    }

    #[test]
    fn test_negative_cost_cases() {
        let mut g = Graph::new(5);
        g.add_edge(0, 3, 2);
        g.add_edge(0, 1, 9);
        g.add_edge(3, 1, -4);
        g.add_edge(1, 4, -3);
        g.add_edge(3, 4, 6);
        g.add_edge(4, 2, 7);
        g.add_edge(2, 1, 3);
        g.add_edge(2, 0, 5);
        assert_eq!(g.find_shortest_path(0), [0, -2, 2, 2, -5]);
    }

    #[test]
    fn test_negative_cycle_cases() {
        let mut g = Graph::new(2);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, -1);
        assert_eq!(g.has_negative_cycle(), false);

        let mut g = Graph::new(2);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, -2);
        assert_eq!(g.has_negative_cycle(), true);

        let mut g = Graph::new(3);
        g.add_edge(0, 1, 5);
        g.add_edge(1, 2, -2);
        g.add_edge(2, 0, -3);
        assert_eq!(g.has_negative_cycle(), false);

        let mut g = Graph::new(3);
        g.add_edge(0, 1, 4);
        g.add_edge(1, 2, -2);
        g.add_edge(2, 0, -3);
        assert_eq!(g.has_negative_cycle(), true);
    }
}
