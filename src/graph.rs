use crate::model::Node;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relation {
    Purchased,
    Interested,
    Similar,
    BelongsTo,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub to: Node,
    pub relation: Relation,
    pub weight: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Graph {
    adjacency: HashMap<Node, Vec<Edge>>,
    edge_count: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, node: Node) -> bool {
        if self.adjacency.contains_key(&node) {
            return false;
        }
        self.adjacency.insert(node, Vec::new());
        true
    }

    pub fn add_directed_edge(
        &mut self,
        from: Node,
        to: Node,
        relation: Relation,
        weight: f64,
    ) -> bool {
        if !weight.is_finite() || weight <= 0.0 {
            return false;
        }
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());
        let edges = self
            .adjacency
            .get_mut(&from)
            .expect("vertice de origem criado");
        if edges
            .iter()
            .any(|edge| edge.to == to && edge.relation == relation)
        {
            return false;
        }
        edges.push(Edge {
            to,
            relation,
            weight,
        });
        self.edge_count += 1;
        true
    }

    pub fn add_undirected_edge(
        &mut self,
        a: Node,
        b: Node,
        relation: Relation,
        weight: f64,
    ) -> bool {
        let first = self.add_directed_edge(a.clone(), b.clone(), relation, weight);
        let second = self.add_directed_edge(b, a, relation, weight);
        first || second
    }

    pub fn neighbors(&self, node: &Node) -> &[Edge] {
        self.adjacency.get(node).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn contains_vertex(&self, node: &Node) -> bool {
        self.adjacency.contains_key(node)
    }

    pub fn vertex_count(&self) -> usize {
        self.adjacency.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_edge_is_rejected() {
        let mut graph = Graph::new();
        let a = Node::Product(1);
        let b = Node::Product(2);
        assert!(graph.add_directed_edge(a.clone(), b.clone(), Relation::Similar, 0.8));
        assert!(!graph.add_directed_edge(a, b, Relation::Similar, 0.9));
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn invalid_weight_is_rejected() {
        let mut graph = Graph::new();
        assert!(!graph.add_directed_edge(
            Node::Product(1),
            Node::Product(2),
            Relation::Similar,
            0.0
        ));
    }
}
