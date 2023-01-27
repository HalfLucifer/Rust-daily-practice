/*
   Check whether a graph is a valid tree

   - input:
     n: count of nodes
     edges: undirected edges between nodes
   - output: tree or not
   - criterion of a valid tree
     1. all nodes are connected
     2. no cycle
*/
use std::collections::HashSet;
use std::collections::VecDeque;

/*
    Union find method
*/
pub fn graph_valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    fn find(root: &Vec<i32>, node: i32) -> i32 {
        let mut node = node;
        while node != root[node as usize] {
            node = root[node as usize];
        }
        node
    }

    let mut root = vec![0; n as usize];

    for i in 0..root.len() {
        root[i] = i as i32;
    }

    for i in 0..edges.len() {
        let x = find(&root, edges[i][0]);
        let y = find(&root, edges[i][1]);

        if x == y {
            return false;
        }

        root[x as usize] = y;
    }

    edges.len() as i32 == n - 1
}

/*
    BFS method
*/
pub fn graph_valid_tree_bfs(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let mut adj = vec![HashSet::new(); n as usize];
    let mut visited = HashSet::new();

    for i in 0..edges.len() {
        adj[edges[i][0] as usize].insert(edges[i][1]);
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        if visited.contains(&curr) {
            return false;
        }

        visited.insert(curr);

        for e in adj[curr].iter() {
            queue.push_back(*e as usize);
        }
    }

    visited.len() == n as usize
}

/*
    DFS method
*/
pub fn graph_valid_tree_dfs(n: i32, edges: Vec<Vec<i32>>) -> bool {
    fn dfs(adj: &Vec<Vec<i32>>, visited: &mut Vec<bool>, curr: i32, prev: i32) -> bool {
        if visited[curr as usize] {
            return false;
        }

        visited[curr as usize] = true;

        for e in adj[curr as usize].iter() {
            if *e != prev {
                if !dfs(adj, visited, *e, curr) {
                    return false;
                }
            }
        }

        true
    }

    let mut adj = vec![vec![]; n as usize];
    let mut visited = vec![false; n as usize];

    for i in 0..edges.len() {
        adj[edges[i][0] as usize].push(edges[i][1]);
        adj[edges[i][1] as usize].push(edges[i][0]);
    }

    if !dfs(&adj, &mut visited, 0, -1) {
        return false;
    }

    visited.iter().all(|v| *v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        assert_eq!(graph_valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]), true);
        assert_eq!(graph_valid_tree_bfs(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]), true);
        assert_eq!(graph_valid_tree_dfs(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]), true);

        assert_eq!(graph_valid_tree(5, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![1, 4]]), true);
        assert_eq!(graph_valid_tree_bfs(5, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![1, 4]]), true);
        assert_eq!(graph_valid_tree_dfs(5, vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![1, 4]]), true);

        assert_eq!(graph_valid_tree(5, vec![vec![0, 1], vec![1 ,2], vec![2, 3], vec![1, 3], vec![1, 4]]), false);
        assert_eq!(graph_valid_tree_bfs(5, vec![vec![0, 1], vec![1 ,2], vec![2, 3], vec![1, 3], vec![1, 4]]), false);
        assert_eq!(graph_valid_tree_dfs(5, vec![vec![0, 1], vec![1 ,2], vec![2, 3], vec![1, 3], vec![1, 4]]), false);

        assert_eq!(graph_valid_tree(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![4, 5]]), false);
        assert_eq!(graph_valid_tree_bfs(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![4, 5]]), false);
        assert_eq!(graph_valid_tree_dfs(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![4, 5]]), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(graph_valid_tree(1, vec![]), true);
        assert_eq!(graph_valid_tree(2, vec![vec![0, 1]]), true);  
        assert_eq!(graph_valid_tree(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]]), false);  
    }
}
