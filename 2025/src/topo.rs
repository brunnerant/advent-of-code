use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Topo<N> {
    nodes: Vec<N>,
    node_idx: HashMap<N, usize>,
    incoming: HashMap<usize, HashSet<usize>>,
    outgoing: HashMap<usize, HashSet<usize>>,
}

impl<N> Default for Topo<N> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
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

    pub fn add_edge(&mut self, a: N, b: N) {
        let a = self.get_node(a);
        let b = self.get_node(b);
        self.incoming.entry(b).or_default().insert(a);
        self.outgoing.entry(a).or_default().insert(b);
    }

    fn get_node(&mut self, n: N) -> usize {
        *self.node_idx.entry(n).or_insert_with(|| {
            let idx = self.nodes.len();
            self.nodes.push(n);
            idx
        })
    }

    pub fn sort(mut self) -> Option<impl Iterator<Item = N>> {
        let mut result = Vec::with_capacity(self.nodes.len());
        let mut no_incoming: Vec<_> = (0..self.nodes.len())
            .filter(|i| self.incoming.get(i).map(|ns| ns.is_empty()).unwrap_or(true))
            .collect();
        let empty = HashSet::new();
        while let Some(n) = no_incoming.pop() {
            result.push(n);
            for &m in self.outgoing.get(&n).unwrap_or(&empty) {
                if let Some(incoming) = self.incoming.get_mut(&m)
                    && !incoming.is_empty()
                {
                    incoming.remove(&n);
                    if incoming.is_empty() {
                        no_incoming.push(m);
                    }
                };
            }
        }
        (result.len() == self.nodes.len()).then(|| result.into_iter().map(move |i| self.nodes[i]))
    }
}

pub fn sort<N: Copy + Eq + Hash>(
    edges: impl IntoIterator<Item = (N, N)>,
) -> Option<impl Iterator<Item = N>> {
    let mut topo = Topo::new();
    for (a, b) in edges {
        topo.add_edge(a, b);
    }
    topo.sort()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use std::collections::HashSet;

    use crate::topo::sort;

    #[test]
    fn test_empty_topo() {
        assert_eq!(
            sort::<usize>(Vec::<(usize, usize)>::new()).map(|i| i.collect()),
            Some(Vec::<usize>::new())
        );
    }

    macro_rules! topo {
        ($($a:literal -> $b:literal),* sorts_to $($c:literal),*) => {
            assert_eq!(sort([$(($a, $b)),*]).map(|i| i.collect()), Some(vec![$($c),*]))
        };
        ($($a:literal -> $b:literal),* sorts_to_prefix $($c:literal),* and_elements $($d:literal),*) => {
            let prefix = vec![$($c),*];
            let elems = HashSet::from([$($d),*]);
            let result = sort([$(($a, $b)),*]);
            assert!(result.is_some());
            let result: Vec<_> = result.unwrap().collect();
            assert_eq!(result.iter().copied().take(prefix.len()).collect::<Vec<_>>(), prefix);
            assert_eq!(HashSet::from_iter(result.iter().copied()), elems);
        };
        ($($a:literal -> $b:literal),* sorts_to_suffix $($c:literal),* and_elements $($d:literal),*) => {
            let suffix = vec![$($c),*];
            let elems = HashSet::from([$($d),*]);
            let result = sort([$(($a, $b)),*]);
            assert!(result.is_some());
            let result: Vec<_> = result.unwrap().collect();
            assert_eq!(result.iter().copied().tail(suffix.len()).collect::<Vec<_>>(), suffix);
            assert_eq!(HashSet::from_iter(result.iter().copied()), elems);
        };
        ($($a:literal -> $b:literal),* has_cycle) => {
            assert!(sort([$(($a, $b)),*]).is_none())
        };
    }

    #[test]
    fn test_topo() {
        topo!("a" -> "b", "b" -> "c" sorts_to "a", "b", "c");
        topo!("a" -> "b", "a" -> "c" sorts_to_prefix "a" and_elements "a", "b", "c");
        topo!("a" -> "c", "b" -> "c" sorts_to_suffix "c" and_elements "a", "b", "c");
        topo!("a" -> "b", "b" -> "c", "c" -> "a" has_cycle);
    }
}
