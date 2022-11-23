pub struct UnionFind {
    num_of_nodes: usize,
    parent: Vec<usize>,
    weight: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(num_of_nodes: usize) -> Self {
        assert!(num_of_nodes > 0);

        Self {
            num_of_nodes: num_of_nodes,
            parent: (0..num_of_nodes).collect::<Vec<_>>(),
            weight: vec![1; num_of_nodes],
            count: num_of_nodes,
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        assert!(a < self.num_of_nodes && b < self.num_of_nodes);

        let root_a = self.find_root(a);
        let root_b = self.find_root(b);

        // Attach smaller tree to larger tree to balance
        if self.weight[root_a] >= self.weight[root_b] {
            self.parent[root_b] = root_a;
            self.weight[root_a] += self.weight[root_b];
        } else {
            self.parent[root_a] = root_b;
            self.weight[root_b] += self.weight[root_a];
        }

        if self.count > 1 {
            self.count -= 1;
        }
    }

    pub fn is_connected(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.num_of_nodes && b < self.num_of_nodes);

        let root_a = self.find_root(a);
        let root_b = self.find_root(b);

        root_a == root_b
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn list_parents(&self) -> Vec<usize> {
        self.parent.clone()
    }

    pub fn list_weights(&self) -> Vec<usize> {
        self.weight.clone()
    }

    fn find_root(&mut self, node: usize) -> usize {
        let mut n = node;

        while n != self.parent[n] {
            self.parent[n] = self.parent[self.parent[n]]; // Compress parent nodes
            n = self.parent[n];
        }

        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cases() {
        let mut u = UnionFind::new(9);
        assert_eq!(u.count(), 9);
        assert_eq!(u.is_connected(1, 2), false);

        u.union(1, 2);
        assert_eq!(u.count(), 8);
        assert_eq!(u.is_connected(1, 2), true);

        u.union(2, 3);
        assert_eq!(u.count(), 7);
        assert_eq!(u.is_connected(1, 2), true);
        assert_eq!(u.is_connected(3, 1), true);

        u.union(5, 6);
        u.union(4, 7);
        u.union(7, 8);
        assert_eq!(u.count(), 4);

        assert_eq!(u.list_parents(), [0, 1, 1, 1, 4, 5, 5, 4, 4]);
        assert_eq!(u.list_weights(), [1, 3, 1, 1, 3, 2, 1, 1, 1]);
    }

    #[test]
    fn test_edge_cases() {
        let mut u = UnionFind::new(1);
        u.union(0, 0);
        assert_eq!(u.count(), 1);
        assert_eq!(u.is_connected(0, 0), true);

        let mut u = UnionFind::new(2);
        u.union(0, 1);
        assert_eq!(u.count(), 1);
        assert_eq!(u.is_connected(0, 1), true);
    }

    #[test]
    fn test_balance_tree_cases() {
        let mut u = UnionFind::new(10);
        for i in 0..9 {
            u.union(i, i + 1);
        }

        assert_eq!(u.count(), 1);
        assert_eq!(u.list_parents(), [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(u.list_weights(), [10, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    }
}
