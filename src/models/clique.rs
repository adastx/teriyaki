use std::collections::{HashMap, VecDeque};

use crate::util::set_ops::{intersection, intersects};

use super::dataset::Dataset;
use super::meta::Meta;

#[derive(Clone)]
pub struct Clique {
    pub preds: Vec<u32>,
    pub nodes: Vec<u32>,
}

impl Clique {
    /// Creates a new `Clique` with the given `preds` and `nodes`.
    pub fn new(preds: &Vec<u32>, nodes: &Vec<u32>) -> Self {
        Self {
            preds: preds.clone(),
            nodes: nodes.clone(),
        }
    }

    /// Removes `node` from the nodes of the `Clique`.
    pub fn remove_node(&mut self, node: &u32) {
        self.nodes.retain(|n| *n != *node);
    }

    /// Returns a `Vec` of all nodes contained both in `self` and `c`.
    pub fn node_intersection(&self, c: &Clique) -> Vec<u32> {
        let mut intersection: Vec<u32> = Vec::new();

        for node in &self.nodes {
            if c.nodes.contains(&node) {
                intersection.push(node.clone());
            }
        }

        return intersection;
    }

    pub fn get_all_edges(
        &self,
        is_source: bool,
        meta: &mut Meta,
    ) -> (Vec<u32>, Vec<Vec<u32>>, Vec<Vec<u32>>) {
        let mut singlenodes: Vec<u32> = Vec::new();
        let mut supernodes: Vec<Vec<u32>> = Vec::new();
        let mut edges: Vec<Vec<u32>> = Vec::new();

        for n in &self.nodes {
            if let Some(child_nodes) = meta.get_supernode(&n) {
                supernodes.push(child_nodes.to_vec());

                for c in child_nodes {
                    singlenodes.push(*c);
                    edges.push(meta.get_preds(c, is_source));
                }
            } else {
                singlenodes.push(*n);
                edges.push(meta.get_preds(n, is_source));
            }
        }
        return (singlenodes, supernodes, edges);
    }
}

pub struct CliqueCollection {
    cliques: Vec<Clique>,
    queue: VecDeque<usize>,
    index_map: HashMap<u32, usize>,
}

impl CliqueCollection {
    /// Creates an empty `CliqueCollection`.
    ///
    /// Initially there is only the empty clique.
    pub fn new() -> Self {
        Self {
            cliques: vec![Clique::new(&vec![], &vec![])],
            queue: VecDeque::new(),
            index_map: HashMap::new(),
        }
    }

    /// Adds the `node` and `pred` of a new triple to the `CliqueCollection`.
    ///
    /// `node` and `pred` do not have to been previosly known.
    pub fn new_triple(&mut self, node: &u32, pred: &u32) {
        let node_exists = self.contains_node(&node);
        let pred_exists = self.contains_pred(&pred);

        if !node_exists && !pred_exists {
            self.new_clique(&vec![*pred], &vec![*node]);
        } else if !node_exists && pred_exists {
            self.add_node_to_clique(node, pred);
        } else if node_exists && !pred_exists {
            self.add_pred_to_clique(node, pred);
        } else {
            if self.index_map.get(node).unwrap() != self.index_map.get(pred).unwrap() {
                self.merge_cliques(node, pred);
            }
        }
    }

    /// Merges the cliques containing the ids `a` and `b`, which can either be nodes or preds.
    ///
    /// `b`'s clique is merged into `a`'s clique, leaving `b`'s clique empty.
    pub fn merge_cliques(&mut self, a: &u32, b: &u32) {
        let a_index = *self.index_map.get(a).unwrap();
        let b_index = *self.index_map.get(b).unwrap();

        let b_clique = self.cliques[b_index].clone();
        self.set_index(&b_clique.preds, &b_clique.nodes, a_index);

        let a_clique = &mut self.cliques[a_index];

        a_clique.nodes.extend(b_clique.nodes);
        a_clique.preds.extend(b_clique.preds);

        self.remove_clique_by_index(b_index);
    }

    /// Adds `pred` to the clique containing `node`.
    ///
    /// # Panics
    ///
    /// Panics if `node` is in the empty clique.
    fn add_pred_to_clique(&mut self, node: &u32, pred: &u32) {
        let index = *self.index_map.get(node).unwrap();
        if index == 0 {
            panic!("Attempting to add new pred to empty clique. wtf?")
        }

        self.cliques[index].preds.push(*pred);
        self.index_map.insert(*pred, index);
    }

    /// Adds `node` to the clique containing `pred`.
    fn add_node_to_clique(&mut self, node: &u32, target: &u32) {
        let index = *self.index_map.get(target).unwrap();
        self.cliques[index].nodes.push(*node);
        self.index_map.insert(*node, index);
    }

    /// Adds the node `node` to the empty clique.
    pub fn add_node_to_empty_clique(&mut self, node: &u32) {
        self.cliques[0].nodes.push(*node);
        self.index_map.insert(*node, 0);
    }

    /// Returns a mutable reference to the clique containing `pred`.
    ///
    /// # Panics
    ///
    /// Panics if the `CliqueCollection` does not contain a clique with `pred`.
    // fn clique_by_pred_mut(&mut self, pred: &u32) -> &mut Clique {
    //     if let Some(index) = self.index_map.get(pred) {
    //         return &mut self.cliques[*index];
    //     }
    //     panic!("No clique found for predicate {}", pred);
    // }

    /// Adds a new clique to the `CliqueCollection` containing `nodes` and `preds`.
    pub fn new_clique(&mut self, preds: &Vec<u32>, nodes: &Vec<u32>) {
        if let Some(index) = self.queue.pop_front() {
            self.cliques[index] = Clique::new(&preds, &nodes);
            self.set_index(preds, nodes, index);
        } else {
            self.cliques.push(Clique::new(&preds, &nodes));
            self.set_index(preds, nodes, self.cliques.len() - 1);
        }
    }

    /// Sets the indices of `nodes` and `preds` to `index`.
    fn set_index(&mut self, preds: &Vec<u32>, nodes: &Vec<u32>, index: usize) {
        for p in preds {
            self.index_map.insert(*p, index);
        }
        for n in nodes {
            self.index_map.insert(*n, index);
        }
    }

    /// Returns true if the `CliqueCollection` contains a clique with `pred`.
    pub fn contains_pred(&self, pred: &u32) -> bool {
        return self.index_map.contains_key(pred);
    }

    /// Returns true if the `CliqueCollection` contains a clique with `node`.
    pub fn contains_node(&self, node: &u32) -> bool {
        return self.index_map.contains_key(node);
    }

    pub fn new_pred(&mut self, pred: &u32) {
        if self.contains_pred(pred) {
            panic!(
                "Attempting to add new pred {} that already exists. wtf?",
                pred
            );
        }
        self.new_clique(&vec![*pred], &vec![]);
    }

    pub fn in_same_clique(&self, a: &u32, b: &u32) -> bool {
        return self.index_map.get(a).unwrap() == self.index_map.get(b).unwrap();
    }

    pub fn in_empty_clique(&self, node: &u32) -> bool {
        return *self.index_map.get(node).unwrap() == 0;
    }

    pub fn get_index(&self, id: &u32) -> usize {
        return *self.index_map.get(id).unwrap();
    }

    pub fn get_nodes(&self, index: usize) -> Vec<u32> {
        return self.cliques[index].nodes.clone();
    }

    pub fn get_clique_by_node(&self, id: &u32) -> Clique {
        return self.cliques[self.get_index(id)].clone();
    }

    pub fn get_clique_by_index(&self, index: usize) -> Clique {
        return self.cliques[index].clone();
    }

    pub fn clique_len(&self, index: usize) -> usize {
        return self.cliques[index].nodes.len();
    }

    pub fn move_node(&mut self, node: &u32, target: &u32) {
        self.remove_node(node);
        self.add_node_to_clique(node, target);
    }

    pub fn move_node_to_empty_clique(&mut self, node: &u32) {
        self.remove_node(node);
        self.add_node_to_empty_clique(node);
    }

    pub fn remove_node(&mut self, node: &u32) {
        let index = self.get_index(node);
        self.cliques[index].remove_node(node);
        self.index_map.remove(node);

        if index != 0 && self.cliques[index].nodes.is_empty() {
            self.queue.push_back(index);
            // panic!("You just removed the last node from a clique. wtf?");
        }
    }

    pub fn snode_split_and_move(&mut self, node: &u32, target: &u32) {
        self.add_node_to_clique(node, target);
    }

    pub fn snode_split(&mut self, node: &u32, parent: &u32) {
        self.add_node_to_clique(node, &parent);
    }

    pub fn to_single_node(&mut self, snode: &u32, single: &u32) {
        self.add_node_to_clique(single, snode);
        self.remove_node(snode);
    }

    pub fn new_snode(&mut self, old: &Vec<u32>, new: &u32) {
        self.add_node_to_clique(new, &old[0]);
        for n in old {
            self.remove_node(n);
        }
    }

    pub fn get_all_edges(
        &self,
        target: &u32,
        is_source: bool,
        meta: &mut Meta,
    ) -> (Vec<u32>, Vec<Vec<u32>>, Vec<Vec<u32>>) {
        return self
            .get_clique_by_node(target)
            .get_all_edges(is_source, meta);
    }

    pub fn remove_clique_by_index(&mut self, index: usize) {
        self.cliques[index].nodes = vec![];
        self.cliques[index].preds = vec![];
        self.queue.push_back(index);
    }

    pub fn remove_supernode(&mut self, p: &u32, meta: &Meta) {
        for n in meta.get_supernode(p).unwrap() {
            self.add_node_to_clique(n, p);
        }
        self.remove_node(p);
    }
}

#[derive(Clone)]
pub struct CliqueChange {
    pub clique_index: usize,
    pub new_nodes: Vec<u32>,
    pub is_source: bool,
}

impl CliqueChange {
    pub fn new(clique_index: usize, new_nodes: Vec<u32>, is_source: bool) -> Self {
        Self {
            clique_index,
            new_nodes,
            is_source,
        }
    }

    pub fn new_merge(cc: &CliqueCollection, a: &u32, b: &u32, is_source: bool) -> Self {
        let a_index = cc.get_index(a);
        let b_index = cc.get_index(b);

        let change = Self::new(
            a_index,
            if cc.clique_len(a_index) < cc.clique_len(b_index) {
                cc.get_nodes(a_index)
            } else {
                cc.get_nodes(b_index)
            },
            is_source,
        );

        return change;
    }

    pub fn get_super_nodes(
        self,
        sc: &mut CliqueCollection,
        tc: &mut CliqueCollection,
    ) -> Vec<Vec<u32>> {
        let mut super_nodes: Vec<Vec<u32>> = Vec::new();

        let c1 = if self.is_source {
            sc.get_clique_by_index(self.clique_index)
        } else {
            tc.get_clique_by_index(self.clique_index)
        };

        for node in self.new_nodes {
            let c2 = if self.is_source {
                tc.get_clique_by_node(&node)
            } else {
                sc.get_clique_by_node(&node)
            };

            let intersect = c1.node_intersection(&c2);
            if intersect.len() >= 2 {
                super_nodes.push(intersect);
            }
        }
        return super_nodes;
    }
}
