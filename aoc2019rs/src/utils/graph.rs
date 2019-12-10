#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub struct Graph<T> {
    adjacency_map: HashMap<T, Vec<T>>,
}

impl <T> Default for Graph<T> 
where T: Hash + Eq,
{
    fn default() -> Self {
        Self { adjacency_map: HashMap::new() }
    }
}

impl <T> Graph<T>
where T: Hash + Eq + Copy
{
    /*
     *
     * <-- Creatation -->
     *
     */

    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, value: T) {
        if !self.adjacency_map.contains_key(&value) {
            self.add_node_with_adjacency_list(value, Vec::new());
        }
    }

    pub fn add_nodes<I>(&mut self, nodes: I) 
    where I: IntoIterator<Item = T>
    {
        nodes.into_iter().for_each(|node| self.add_node(node))
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        self.adjacency_map
            .get_mut(&from)
            .expect("from node not found")
            .push(to);
    }

    pub fn add_edges<I>(&mut self, edges: I) 
    where I: IntoIterator<Item = (T, T)>
    {
        edges.into_iter().for_each(|(from, to)| self.add_edge(from, to))
    }

    pub fn add_node_with_adjacency_list(&mut self, node: T, list: Vec<T>) {
        self.adjacency_map.insert(node, list);
    }

    pub fn nodes(&self) -> Vec<&T> {
        self.adjacency_map.keys().collect()
    }

    pub fn edges(&self, node: T) -> &[T] {
        &self.adjacency_map[&node]
    }

    /*
     *
     * <-- Algorithms -->
     *
     */
    pub fn bfs(&self, start: T, target: T) -> Option<T> {
        use std::collections::VecDeque;
        use std::collections::HashSet;

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        while let Some(next) = queue.pop_front() {
            if next == target {
                return Some(target)
            }

            if !visited.contains(&next) {
                visited.insert(next);
                self.adjacency_map
                    .get(&next)
                    .expect("Node not found")
                    .into_iter()
                    .for_each(|item| queue.push_back(*item));
            }
        }

        None
    }

    pub fn dfs(&self, start: T, target: T) -> Option<T> {
        use std::collections::HashSet;

        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        stack.push(start);
        while let Some(next) = stack.pop() {
            if next == target {
                return Some(target)
            }

            if !visited.contains(&next) {
                visited.insert(next);
                self.adjacency_map
                    .get(&next)
                    .expect("Node not found")
                    .into_iter()
                    .for_each(|item| stack.push(*item));
            }
        }

        None
    }

    pub fn path_between(&self, start: T, target: T) -> Option<Vec<T>> {
        use std::collections::HashSet;

        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        stack.push((start, Vec::new()));
        while let Some((next, mut path)) = stack.pop() {
            if next == target {
                path.push(next);
                return Some(path)
            }

            if !visited.contains(&next) {
                visited.insert(next);
                self.adjacency_map
                    .get(&next)
                    .expect("Node not found")
                    .into_iter()
                    .for_each(|item| {
                        let mut item_path = path.clone();
                        item_path.push(next);
                        stack.push((*item, item_path));
                    });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let mut graph: Graph<i32> = Graph::new();
        graph.add_nodes(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let edges = vec![
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 8),
        ];
        graph.add_edges(edges);

        let path = graph.path_between(3, 6).unwrap();
        assert_eq!(path, vec![3, 4, 5, 6]);
    }
}
