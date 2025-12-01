use std::collections::HashSet;

const INPUT: &str = include_str!("../assets/day05.txt");

struct PageOrdering {
    constraints: HashSet<(usize, usize)>,
}

struct TopologicalSort {
    incoming: Vec<HashSet<usize>>,
    outgoing: Vec<HashSet<usize>>,
}

impl TopologicalSort {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            incoming: vec![HashSet::new(); num_nodes],
            outgoing: vec![HashSet::new(); num_nodes],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.incoming[to].insert(from);
        self.outgoing[from].insert(to);
    }

    pub fn sort(mut self) -> Option<(Vec<usize>, bool)> {
        let mut result = Vec::with_capacity(self.incoming.len());
        let mut unique = true;
        let mut no_incoming = self
            .incoming
            .iter()
            .enumerate()
            .filter_map(|(i, e)| e.is_empty().then_some(i))
            .collect::<HashSet<_>>();

        while !no_incoming.is_empty() {
            if no_incoming.len() > 1 {
                unique = false;
            }

            let i = *no_incoming.iter().next().unwrap();
            result.push(i);
            no_incoming.remove(&i);
            for &j in self.outgoing[i].iter() {
                self.incoming[j].remove(&i);
                if self.incoming[j].is_empty() {
                    no_incoming.insert(j);
                }
            }
        }

        if result.len() == self.incoming.len() {
            Some((result, unique))
        } else {
            None
        }
    }
}

impl PageOrdering {
    #[allow(dead_code)]
    fn from_slice(constraints: &[(usize, usize)]) -> Self {
        Self {
            constraints: constraints.iter().copied().collect(),
        }
    }

    pub fn new() -> Self {
        Self {
            constraints: HashSet::new(),
        }
    }

    pub fn add_constraint(&mut self, a: usize, b: usize) {
        self.constraints.insert((a, b));
    }

    pub fn satisfies_list(&self, list: &[usize]) -> bool {
        for i in 0..list.len() {
            for j in i + 1..list.len() {
                if self.constraints.contains(&(list[j], list[i])) {
                    return false;
                }
            }
        }
        true
    }

    pub fn reorder_list(&self, list: &[usize]) -> Option<Vec<usize>> {
        let mut graph = TopologicalSort::new(list.len());
        for i in 0..list.len() {
            for j in i + 1..list.len() {
                if self.constraints.contains(&(list[i], list[j])) {
                    graph.add_edge(i, j);
                }
                if self.constraints.contains(&(list[j], list[i])) {
                    graph.add_edge(j, i);
                }
            }
        }

        if let Some((ordered, true)) = graph.sort() {
            Some(ordered.iter().map(|&i| list[i]).collect())
        } else {
            None
        }
    }
}

fn parse(input: &str) -> (PageOrdering, Vec<Vec<usize>>) {
    let mut ordering = PageOrdering::new();
    let mut lists = Vec::new();

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let [a, b] = line
            .split("|")
            .map(|p| p.parse::<usize>().unwrap())
            .collect::<Vec<_>>()[..]
        else {
            panic!("expected two elements in a constraint");
        };
        ordering.add_constraint(a, b);
    }

    while let Some(line) = lines.next() {
        let list: Vec<usize> = line
            .split(",")
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        assert_eq!(list.len() % 2, 1);
        lists.push(list);
    }

    (ordering, lists)
}

pub fn part1() -> usize {
    let (ordering, lists) = parse(INPUT);
    let mut result = 0;
    for list in lists {
        if ordering.satisfies_list(&list) {
            result += list[list.len() / 2];
        }
    }
    result
}

pub fn part2() -> usize {
    let (ordering, lists) = parse(INPUT);
    let mut result = 0;
    for list in lists {
        if !ordering.satisfies_list(&list) {
            let order = ordering
                .reorder_list(&list)
                .expect("the list doesn't have a unique re-ordering");
            result += order[order.len() / 2];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::day05::PageOrdering;

    #[test]
    fn test_ordering() {
        let ordering = PageOrdering::from_slice(&[
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ]);

        assert!(ordering.satisfies_list(&[75, 47, 61, 53, 29]));
        assert!(ordering.satisfies_list(&[97, 61, 53, 29, 13]));
        assert!(ordering.satisfies_list(&[75, 29, 13]));
        assert!(!ordering.satisfies_list(&[75, 97, 47, 61, 53]));
        assert!(!ordering.satisfies_list(&[61, 13, 29]));
        assert!(!ordering.satisfies_list(&[97, 13, 75, 29, 47]));

        assert_eq!(ordering.reorder_list(&[61, 13, 29]), Some(vec![61, 29, 13]));
        assert_eq!(
            ordering.reorder_list(&[75, 97, 47, 61, 53]),
            Some(vec![97, 75, 47, 61, 53])
        );
        assert_eq!(
            ordering.reorder_list(&[97, 13, 75, 29, 47]),
            Some(vec![97, 75, 47, 29, 13])
        );
    }
}
