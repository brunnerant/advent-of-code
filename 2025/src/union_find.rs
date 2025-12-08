use std::cell::RefCell;

use itertools::Itertools;

pub struct UnionFind {
    parent: RefCell<Vec<usize>>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: RefCell::new((0..n).collect()),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    pub fn group_idx(&self, i: usize) -> usize {
        let parent = self.parent.borrow_mut()[i];
        if parent == i {
            i
        } else {
            let new_parent = self.group_idx(parent);
            self.parent.borrow_mut()[i] = new_parent;
            new_parent
        }
    }

    pub fn group_size(&self, i: usize) -> usize {
        self.size[self.group_idx(i)]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.group_idx(a);
        let b = self.group_idx(b);
        if a == b {
            return;
        }
        if self.rank[a] < self.rank[b] {
            self.parent.borrow_mut()[a] = b;
            self.size[b] += self.size[a];
        } else if self.rank[a] > self.rank[b] {
            self.parent.borrow_mut()[b] = a;
            self.size[a] += self.size[b];
        } else {
            self.parent.borrow_mut()[b] = a;
            self.size[a] += self.size[b];
            self.rank[a] += 1;
        }
    }
}

pub fn connected_components(
    num_nodes: usize,
    edges: impl IntoIterator<Item = (usize, usize)>,
) -> Vec<Vec<usize>> {
    let mut uf = UnionFind::new(num_nodes);
    for (a, b) in edges {
        uf.union(a, b);
    }
    let map = (0..num_nodes)
        .into_grouping_map_by(|&n| uf.group_idx(n))
        .collect::<Vec<_>>();
    map.into_values().collect()
}
