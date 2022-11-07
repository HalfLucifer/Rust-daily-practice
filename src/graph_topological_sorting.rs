pub struct Graph {
    num_of_nodes: usize,
    adjacency_list: Vec<Vec<usize>>,
    inbound_list: Vec<isize>,
}

impl Graph {
    pub fn new(num_of_nodes: usize, dep_list: Vec<(usize, usize)>) -> Self {
        // Build one-way adjacency list
        let mut adj = vec![vec![]; num_of_nodes];

        dep_list.iter().for_each(|(source, dest)| {
            adj[*source].push(*dest);
        });

        let mut inbound = vec![0; num_of_nodes];

        adj.iter().enumerate().for_each(|(i, e)| {
            e.iter().for_each(|index| {
                if i != *index {
                    inbound[*index] += 1;
                }
            });
        });

        Self {
            num_of_nodes: num_of_nodes,
            adjacency_list: adj,
            inbound_list: inbound,
        }
    }

    pub fn topological_sorting(&self) -> Option<Vec<usize>> {
        fn proceed_zero_inbound_nodes(inbounds: &mut Vec<isize>, queue: &mut Vec<usize>) {
            inbounds.iter_mut().enumerate().for_each(|(i, count)| {
                if *count == 0 {
                    *count = -1;
                    queue.push(i);
                }
            });
        }

        let mut result = vec![];
        let mut queue = vec![];
        let mut ins = self.inbound_list.clone();

        proceed_zero_inbound_nodes(&mut ins, &mut queue);

        while !queue.is_empty() {
            let node = queue.remove(0);
            result.push(node);

            self.adjacency_list[node].iter().for_each(|e| {
                ins[*e] -= 1;
            });

            proceed_zero_inbound_nodes(&mut ins, &mut queue);

            // Sorting to ensure proceeding from lowest to highest index
            queue.sort();
        }

        // There is a cycle dependency if result length is lesser
        match self.num_of_nodes == result.len() {
            true => Some(result),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        /*
             |--> 2 <---------|
             |--> 3 --> 0 --> 1
           5 |    |
             |    v
             |--> 4

           adjacency list: [[1], [2], [], [0, 4], [], [2, 3, 4]]
        */
        let g = Graph::new(
            6,
            vec![(5, 2), (5, 3), (5, 4), (0, 1), (1, 2), (3, 0), (3, 4)],
        );
        assert_eq!(g.topological_sorting(), Some(vec![5, 3, 0, 1, 2, 4]));

        let g = Graph::new(
            8,
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)],
        );
        assert_eq!(g.topological_sorting(), Some(vec![0, 1, 2, 3, 4, 5, 6, 7]));
    }

    #[test]
    fn test_edge_cases() {
        let g = Graph::new(1, vec![(0, 0)]);
        assert_eq!(g.topological_sorting(), Some(vec![0]));

        let g = Graph::new(2, vec![(0, 1)]);
        assert_eq!(g.topological_sorting(), Some(vec![0, 1]));
    }

    #[test]
    fn test_cycle_cases() {
        let g = Graph::new(2, vec![(0, 1), (1, 0)]);
        assert_eq!(g.topological_sorting(), None);

        let g = Graph::new(3, vec![(0, 1), (1, 2), (2, 0)]);
        assert_eq!(g.topological_sorting(), None);

        let g = Graph::new(
            6,
            vec![(5, 2), (5, 3), (4, 5), (0, 1), (1, 2), (3, 0), (3, 4)],
        );
        assert_eq!(g.topological_sorting(), None);

        let g = Graph::new(
            6,
            vec![(5, 2), (5, 3), (5, 4), (0, 1), (1, 3), (3, 0), (3, 4)],
        );
        assert_eq!(g.topological_sorting(), None);
    }
}
