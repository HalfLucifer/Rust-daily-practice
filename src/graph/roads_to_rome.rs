/*
    Find out whether all roads lead to Rome (one and only one destination). 
    If Rome exists, return its node index, otherwise returns -1.

    Given that no cycle is existed.
 */
use std::collections::VecDeque;

pub fn all_roads_lead_to_rome(num_of_nodes: i32, source: Vec<i32>, dest: Vec<i32>) -> i32 {
    // Build adjacency & indegree list
    let mut adj = vec![vec![]; num_of_nodes as usize];
    let mut indegree = vec![0; num_of_nodes as usize];

    for i in 0..source.len() {
        let src = source[i];
        let dest = dest[i];
        adj[src as usize].push(dest);

        if i != dest as usize {
            indegree[dest as usize] += 1;
        }
    }

    // Push 0 indegree nodes to queue
    let mut queue = VecDeque::new();

    for i in 0..indegree.len() {
        if indegree[i] == 0 {
            queue.push_back(i);
        }
    }

    // BFS: going from edges to Rome
    let mut unvisited = num_of_nodes;

    while !queue.is_empty() {
        let count = queue.len();

        for _ in 0..count {
            let node = queue.pop_front().unwrap();
            unvisited -= 1;

            // Decrease indegree for each adjacent node
            adj[node].iter().for_each(|e| {
                indegree[*e as usize] -= 1;

                // Push to queue when indegree goes to 0
                if indegree[*e as usize] == 0 {
                    queue.push_back(*e as usize);
                }
            });
        }

        // Exit condition: when unvisted nodes equals to the length of queue,
        // it reaches the end of traversal
        //   case 1) If there is 1 node remaining, it has to be Rome
        //   case 2) If there are multiple nodes, no Rome is existed
        if unvisited == queue.len() as i32 {
            if queue.len() == 1 {
                return queue[0] as i32;
            } else {
                return -1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(all_roads_lead_to_rome(5, vec![0, 1, 2, 3, 4], vec![0, 0, 0, 0, 0]), 0);
        assert_eq!(all_roads_lead_to_rome(6, vec![0, 1, 2, 3, 4, 5], vec![2, 2, 3, 3, 3, 3]), 3);
        assert_eq!(all_roads_lead_to_rome(6, vec![1, 2, 3, 4, 4, 5], vec![0, 0, 1, 1, 2, 2]), 0);
        assert_eq!(all_roads_lead_to_rome(6, vec![0, 0, 0, 0, 0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(all_roads_lead_to_rome(1, vec![0], vec![0]), -1);
        assert_eq!(all_roads_lead_to_rome(3, vec![0, 1, 1, 2], vec![0, 0, 2, 2]), -1);
        assert_eq!(all_roads_lead_to_rome(5, vec![0, 2, 2, 4], vec![1, 1, 3, 3]), -1);
        assert_eq!(all_roads_lead_to_rome(5, vec![0, 1, 2, 3, 4], vec![0, 1, 2, 3, 4]), -1);
    }
}
