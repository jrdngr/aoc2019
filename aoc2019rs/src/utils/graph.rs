use anyhow::{bail, Result};

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub struct Graph<'a, T> {
    adjacency_map: HashMap<T, Vec<&'a T>>,
}

impl <'a, T> Default for Graph<'a, T> 
where T: Hash + Eq,
{
    fn default() -> Self {
        Self { adjacency_map: HashMap::new() }
    }
}

impl <'a, T> Graph<'a, T>
where T: Hash + Eq, 
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
        if self.adjacency_map.contains_key(&value) {
            self.add_node_with_adjacency_list(value, Vec::new());
        } else {
            panic!("Node is already present")
        }
    }

    pub fn add_nodes<I>(&mut self, nodes: I) 
    where I: IntoIterator<Item = T>
    {
        nodes.into_iter().for_each(|node| self.add_node(node))
    }

    pub fn add_edge(&mut self, from: &T, to: &'a T) {
        self.adjacency_map
            .get_mut(from)
            .expect("from node not found")
            .push(to);
    }

    pub fn add_edges<I>(&mut self, edges: I) 
    where I: IntoIterator<Item = (&'a T, &'a T)>
    {
        edges.into_iter().for_each(|(from, to)| self.add_edge(from, to))
    }

    pub fn add_node_with_adjacency_list(&mut self, node: T, list: Vec<&'a T>) {
        self.adjacency_map.insert(node, list);
    }

    /*
     *
     * <-- Algorithms -->
     *
     */

    pub fn bfs(&self, start: &T, target: &'a T) -> Option<&'a T> {
        use std::collections::VecDeque;
        use std::collections::HashSet;

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        while let Some(next) = queue.pop_front() {
            if next == target {
                return Some(target)
            }

            visited.insert(next);
            self.adjacency_map
                .get(next)
                .expect("Node not found")
                .into_iter()
                .for_each(|item| queue.push_back(item));
        }

        None
    }

    pub fn dfs(&self, start: &T, target: &'a T) -> Option<&'a T> {
        use std::collections::HashSet;

        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        stack.push(start);
        while let Some(next) = stack.pop() {
            if next == target {
                return Some(target)
            }

            visited.insert(next);
            self.adjacency_map
                .get(next)
                .expect("Node not found")
                .into_iter()
                .for_each(|item| stack.push(item));
        }

        None
    }

    pub fn path_bfs(&self, start: &'a T, target: &'a T) -> Option<(&'a T, Vec<&'a T>)> {
        use std::collections::HashSet;

        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        stack.push((start, Vec::new()));
        while let Some((next, path)) = stack.pop() {
            if next == target {
                return Some((target, path))
            }

            visited.insert(next);
            self.adjacency_map
                .get(next)
                .expect("Node not found")
                .into_iter()
                .for_each(|item| {
                    let mut item_path = path.clone();
                    item_path.push(next);
                    stack.push((item, item_path));
                });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let mut graph = Graph::new();
        graph.add_nodes(&[1, 2, 3, 4, 5, 6, 7, 8]);

    }
}
