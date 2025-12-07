use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Topo<N> {
    nodes: Vec<N>,
    no_deps: HashSet<usize>,
    node_idx: HashMap<N, usize>,
    incoming: Vec<HashSet<usize>>,
    outgoing: Vec<HashSet<usize>>,
}

impl<N> Default for Topo<N> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            no_deps: Default::default(),
            node_idx: Default::default(),
            incoming: Default::default(),
            outgoing: Default::default(),
        }
    }
}

impl<N> Topo<N>
where
    N: Copy + Eq + Hash,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_edges(edges: impl IntoIterator<Item = (N, N)>) -> Self {
        let mut result = Self::new();
        for (a, b) in edges {
            result.add_edge(a, b);
        }
        result
    }

    pub fn add_edge(&mut self, a: N, b: N) {
        let a = self.get_node(a);
        let b = self.get_node(b);
        self.incoming[b].insert(a);
        self.outgoing[a].insert(b);
        self.no_deps.remove(&b);
    }

    fn get_node(&mut self, n: N) -> usize {
        *self.node_idx.entry(n).or_insert_with(|| {
            let idx = self.nodes.len();
            self.nodes.push(n);
            self.no_deps.insert(idx);
            self.incoming.push(HashSet::new());
            self.outgoing.push(HashSet::new());
            idx
        })
    }

    pub fn num_nodes(&self) -> usize {
        self.node_idx.len()
    }

    /// Pops the elements that have no predecessors from the graph
    pub fn pop(&mut self) -> Vec<N> {
        let no_incoming: Vec<_> = self.no_deps.iter().copied().collect();
        for &n in no_incoming.iter() {
            for &m in &self.outgoing[n] {
                self.incoming[m].remove(&n);
                if self.incoming[m].is_empty() {
                    self.no_deps.insert(m);
                }
            }
            self.no_deps.remove(&n);
        }
        no_incoming
            .into_iter()
            .map(|i| {
                let node = self.nodes[i];
                self.node_idx.remove(&node);
                node
            })
            .collect()
    }

    /// Topologically sorts the graph. The output is grouped in batches
    /// such that a node in a batch has all its predecessors in previous batches.
    pub fn sort(mut self) -> Option<Vec<Vec<N>>> {
        let mut result = Vec::new();
        while self.num_nodes() > 0 {
            let batch = self.pop();
            if batch.is_empty() {
                return None;
            } else {
                result.push(batch);
            }
        }
        Some(result)
    }

    /// Same as `sort` but flattens the nested batches
    pub fn sort_flat(self) -> Option<Vec<N>> {
        self.sort().map(|b| b.into_iter().flatten().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::topo::Topo;

    #[test]
    fn test_empty_topo() {
        assert_eq!(
            Topo::<usize>::from_edges(Vec::<(usize, usize)>::new()).sort_flat(),
            Some(Vec::<usize>::new())
        );
    }

    macro_rules! topo {
        ($($a:literal -> $b:literal),* sorts_to $($($c:literal),*)->*) => {
            let result = Topo::from_edges([$(($a, $b)),*]).sort();
            assert!(result.is_some());

            let result = result.unwrap();
            let expected = vec![$(HashSet::from([$($c),*])),*];
            assert_eq!(result.len(), expected.len());
            for (a, b) in result.into_iter().zip(expected) {
                let a = HashSet::from_iter(a.into_iter());
                assert_eq!(a, b);
            }
        };
        ($($a:literal -> $b:literal),* has_cycle) => {
            assert!(Topo::from_edges([$(($a, $b)),*]).sort().is_none())
        };
    }

    #[test]
    fn test_topo() {
        topo!("a" -> "b", "b" -> "c" sorts_to "a" -> "b" -> "c");
        topo!("a" -> "b", "a" -> "c" sorts_to "a" -> "b", "c");
        topo!("a" -> "c", "b" -> "c" sorts_to "a", "b" -> "c");
        topo!("a" -> "b", "a" -> "c", "b" -> "d", "c" -> "d" sorts_to "a" -> "b", "c" -> "d");
        topo!("a" -> "b", "a" -> "c", "b" -> "d" sorts_to "a" -> "b", "c" -> "d");
        topo!("a" -> "b", "a" -> "c", "b" -> "d", "c" -> "e" sorts_to "a" -> "b", "c" -> "d", "e");
        topo!("a" -> "b", "a" -> "c", "b" -> "d", "c" -> "e", "d" -> "e" sorts_to "a" -> "b", "c" -> "d" -> "e");
        topo!("a" -> "b", "b" -> "c", "c" -> "a" has_cycle);
    }
}
