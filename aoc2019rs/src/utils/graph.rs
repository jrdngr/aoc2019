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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, value: T) -> Result<()> {
        if self.adjacency_map.contains_key(&value) {
            self.add_node_with_adjacency_list(value, Vec::new());
            Ok(())
        } else {
            bail!("Node is already present")
        }
    }

    pub fn add_edge(&mut self, from: &T, to: &'a T) {
        self.adjacency_map
            .get_mut(from)
            .expect("from node not found")
            .push(to);
    }

    pub fn set_adjacency_list(&mut self, node: &T, list: Vec<&'a T>) {
        // match self.adjacency_map.get_mut(node) {
        //     Some(list) => 
        // }
        // self.adjacency_map.insert(node, list);
    }

    pub fn add_node_with_adjacency_list(&mut self, node: T, list: Vec<&'a T>) {
        self.adjacency_map.insert(node, list);
    }

}
