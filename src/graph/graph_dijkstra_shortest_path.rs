use std::cmp::Ordering;
use std::collections::BinaryHeap;

/*
    Dijkstra shortest path algorithm
    - Time complexity: O(V + E*logV)
    - Space complexity: O(V)
*/
pub struct Graph {
    num_of_nodes: usize,
    adjacency_list: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    pub fn new(num_of_nodes: usize) -> Self {
        Self {
            num_of_nodes: num_of_nodes,
            adjacency_list: vec![vec![]; num_of_nodes],
        }
    }

    pub fn add_edge(&mut self, source: usize, dest: usize, cost: usize) {
        assert!(source < self.num_of_nodes && dest < self.num_of_nodes);

        self.adjacency_list[source].push((dest, cost));
    }

    pub fn find_shortest_path(&self, source: usize) -> Vec<usize> {
        assert!(source < self.num_of_nodes);

        if self.adjacency_list.is_empty() {
            return vec![];
        }

        let mut result = vec![usize::MAX; self.num_of_nodes];
        result[source] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Node {
            position: source,
            cost: 0,
        });

        while let Some(node) = heap.pop() {
            let pos = node.position;
            let cost = node.cost;

            self.adjacency_list[pos].iter().for_each(|e| {
                let next_pos = e.0;
                let next_cost = e.1 + cost;

                if result[next_pos] > next_cost {
                    result[next_pos] = next_cost;
                    heap.push(Node {
                        position: next_pos,
                        cost: next_cost,
                    });
                }
            });
        }

        result
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: usize,
    cost: usize,
}

// Explicitly implement Ord & PartialOrd trait so that it becomes a min-heap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        assert_eq!(g.find_shortest_path(0), [0, 3000, usize::MAX]);
    }

    #[test]
    fn test_one_way_cases() {
        let mut g = Graph::new(100);
        for i in 0..99 {
            g.add_edge(i, i + 1, i);
        }

        let expected = (0..100).map(|v|(0..v).sum()).collect::<Vec<_>>();
        assert_eq!(g.find_shortest_path(0), expected);
    }
}
