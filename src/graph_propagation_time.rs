pub struct Graph {
    num_of_nodes: usize,
    adjacency_list: Vec<Vec<usize>>,
    propagation_time: Vec<u32>,
}

impl Graph {
    const ROOT_NODE: i32 = -1;

    pub fn new(root_id: usize, parent_id: Vec<i32>, prop_time: Vec<u32>) -> Self {
        assert!(parent_id.len() == prop_time.len());

        // Build one-way adjacency list
        let mut adj = vec![vec![]; parent_id.len()];
        parent_id.iter().enumerate().for_each(|(id, parent)| {
            if *parent != Graph::ROOT_NODE {
                adj[*parent as usize].push(id);
            }
        });

        Self {
            num_of_nodes: parent_id.len(),
            adjacency_list: adj,
            propagation_time: prop_time,
        }
    }

    pub fn get_max_propagation_time(&self, source_id: usize) -> u32 {
        fn get_max_propagation_time_recursive(
            source_id: usize,
            adjacency_list: &Vec<Vec<usize>>,
            propagation_time: &Vec<u32>,
        ) -> u32 {
            let mut max_time = 0;

            adjacency_list[source_id].iter().for_each(|id| {
                max_time = max_time.max(get_max_propagation_time_recursive(*id, adjacency_list, propagation_time));
            });

            max_time + propagation_time[source_id]
        }

        if source_id >= self.num_of_nodes {
            return 0;
        }

        get_max_propagation_time_recursive(source_id, &self.adjacency_list, &self.propagation_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        /*
                     0
                  /  |  \
                 1   2   3
                / \  |   |
               4  5  6   7
               |  |
               8  9

               prop time sum: [ 10 ] + [ 1 ] + [ 5 ]  = 16
               graph node:       0 --->  1  -->  5  ---> 9
        */
        let g = Graph::new(
            0,
            vec![-1, 0, 0, 0, 1, 1, 2, 3, 4, 5],
            vec![10, 1, 2, 3, 4, 5, 0, 0, 0, 0],
        );

        assert_eq!(g.get_max_propagation_time(0), 16);
        assert_eq!(g.get_max_propagation_time(1), 6);
        assert_eq!(g.get_max_propagation_time(2), 2);
        assert_eq!(g.get_max_propagation_time(3), 3);
        assert_eq!(g.get_max_propagation_time(4), 4);
        assert_eq!(g.get_max_propagation_time(5), 5);
        assert_eq!(g.get_max_propagation_time(6), 0);
        assert_eq!(g.get_max_propagation_time(7), 0);
        assert_eq!(g.get_max_propagation_time(8), 0);
        assert_eq!(g.get_max_propagation_time(9), 0);
    }

    #[test]
    fn test_edge_cases() {
        let g = Graph::new(0, vec![-1], vec![0]);
        assert_eq!(g.get_max_propagation_time(0), 0);
        assert_eq!(g.get_max_propagation_time(1), 0);
        assert_eq!(g.get_max_propagation_time(100), 0);

        let g = Graph::new(0, vec![-1, 0], vec![0, 1]);
        assert_eq!(g.get_max_propagation_time(0), 1);
        assert_eq!(g.get_max_propagation_time(1), 1);
        assert_eq!(g.get_max_propagation_time(2), 0);

        let mut parent_id = vec![-1];
        let mut prop_time = vec![1];
        for i in 0..100 {
            parent_id.push(i);
            prop_time.push(1);
        }

        let g = Graph::new(0, parent_id, prop_time);
        assert_eq!(g.get_max_propagation_time(0), 101);
    }
}
