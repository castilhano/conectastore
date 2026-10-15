use crate::graph::{Graph, Relation};
use crate::model::{Customer, Node, Product, Recommendation};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoreError {
    DuplicateProduct(u64),
    DuplicateCustomer(u64),
    ProductNotFound(u64),
    CustomerNotFound(u64),
    InvalidWeight,
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateProduct(id) => write!(f, "produto {id} ja cadastrado"),
            Self::DuplicateCustomer(id) => write!(f, "cliente {id} ja cadastrado"),
            Self::ProductNotFound(id) => write!(f, "produto {id} nao encontrado"),
            Self::CustomerNotFound(id) => write!(f, "cliente {id} nao encontrado"),
            Self::InvalidWeight => write!(f, "peso deve ser finito e maior que zero"),
        }
    }
}

impl Error for StoreError {}

#[derive(Clone, Debug, Default)]
pub struct Store {
    products: HashMap<u64, Product>,
    customers: HashMap<u64, Customer>,
    purchases: HashMap<u64, HashSet<u64>>,
    graph: Graph,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), StoreError> {
        if self.products.contains_key(&product.id) {
            return Err(StoreError::DuplicateProduct(product.id));
        }
        self.graph.add_vertex(Node::Product(product.id));
        let category = Node::Category(product.category.clone());
        self.graph.add_undirected_edge(
            Node::Product(product.id),
            category,
            Relation::BelongsTo,
            0.35,
        );
        self.products.insert(product.id, product);
        Ok(())
    }

    pub fn add_customer(&mut self, customer: Customer) -> Result<(), StoreError> {
        if self.customers.contains_key(&customer.id) {
            return Err(StoreError::DuplicateCustomer(customer.id));
        }
        self.graph.add_vertex(Node::Customer(customer.id));
        self.customers.insert(customer.id, customer);
        Ok(())
    }

    pub fn product(&self, id: u64) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn customer(&self, id: u64) -> Option<&Customer> {
        self.customers.get(&id)
    }

    pub fn connect_products(&mut self, a: u64, b: u64, weight: f64) -> Result<bool, StoreError> {
        self.ensure_product(a)?;
        self.ensure_product(b)?;
        Self::validate_weight(weight)?;
        Ok(self.graph.add_undirected_edge(
            Node::Product(a),
            Node::Product(b),
            Relation::Similar,
            weight,
        ))
    }

    pub fn register_purchase(
        &mut self,
        customer_id: u64,
        product_id: u64,
        weight: f64,
    ) -> Result<bool, StoreError> {
        self.ensure_customer(customer_id)?;
        self.ensure_product(product_id)?;
        Self::validate_weight(weight)?;
        let inserted = self
            .purchases
            .entry(customer_id)
            .or_default()
            .insert(product_id);
        self.graph.add_undirected_edge(
            Node::Customer(customer_id),
            Node::Product(product_id),
            Relation::Purchased,
            weight,
        );
        Ok(inserted)
    }

    pub fn register_interest(
        &mut self,
        customer_id: u64,
        product_id: u64,
        weight: f64,
    ) -> Result<bool, StoreError> {
        self.ensure_customer(customer_id)?;
        self.ensure_product(product_id)?;
        Self::validate_weight(weight)?;
        Ok(self.graph.add_undirected_edge(
            Node::Customer(customer_id),
            Node::Product(product_id),
            Relation::Interested,
            weight,
        ))
    }

    pub fn recommend_for_customer(
        &self,
        customer_id: u64,
        max_depth: usize,
        limit: usize,
    ) -> Result<Vec<Recommendation>, StoreError> {
        self.ensure_customer(customer_id)?;
        let excluded = self
            .purchases
            .get(&customer_id)
            .cloned()
            .unwrap_or_default();
        Ok(self.bfs_recommend(Node::Customer(customer_id), max_depth, limit, &excluded))
    }

    pub fn recommend_from_product(
        &self,
        product_id: u64,
        max_depth: usize,
        limit: usize,
    ) -> Result<Vec<Recommendation>, StoreError> {
        self.ensure_product(product_id)?;
        let excluded = HashSet::from([product_id]);
        Ok(self.bfs_recommend(Node::Product(product_id), max_depth, limit, &excluded))
    }

    pub fn product_count(&self) -> usize {
        self.products.len()
    }

    pub fn customer_count(&self) -> usize {
        self.customers.len()
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    fn bfs_recommend(
        &self,
        start: Node,
        max_depth: usize,
        limit: usize,
        excluded_products: &HashSet<u64>,
    ) -> Vec<Recommendation> {
        if max_depth == 0 || limit == 0 {
            return Vec::new();
        }

        let mut queue = VecDeque::from([(start.clone(), 0usize)]);
        let mut visited = HashSet::from([start]);
        let mut candidates: HashMap<u64, (f64, usize)> = HashMap::new();

        while let Some((node, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }
            for edge in self.graph.neighbors(&node) {
                let next_depth = depth + 1;
                if let Node::Product(product_id) = edge.to {
                    if !excluded_products.contains(&product_id) {
                        let entry = candidates.entry(product_id).or_insert((0.0, next_depth));
                        entry.0 += edge.weight / next_depth as f64;
                        entry.1 = entry.1.min(next_depth);
                    }
                }
                if visited.insert(edge.to.clone()) {
                    queue.push_back((edge.to.clone(), next_depth));
                }
            }
        }

        let mut result: Vec<_> = candidates
            .into_iter()
            .filter_map(|(id, (score, distance))| {
                self.products
                    .get(&id)
                    .cloned()
                    .map(|product| Recommendation {
                        product,
                        score,
                        distance,
                    })
            })
            .collect();
        result.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(Ordering::Equal)
                .then_with(|| a.distance.cmp(&b.distance))
                .then_with(|| a.product.id.cmp(&b.product.id))
        });
        result.truncate(limit);
        result
    }

    fn ensure_product(&self, id: u64) -> Result<(), StoreError> {
        self.products
            .contains_key(&id)
            .then_some(())
            .ok_or(StoreError::ProductNotFound(id))
    }

    fn ensure_customer(&self, id: u64) -> Result<(), StoreError> {
        self.customers
            .contains_key(&id)
            .then_some(())
            .ok_or(StoreError::CustomerNotFound(id))
    }

    fn validate_weight(weight: f64) -> Result<(), StoreError> {
        if weight.is_finite() && weight > 0.0 {
            Ok(())
        } else {
            Err(StoreError::InvalidWeight)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_store() -> Store {
        let mut store = Store::new();
        store.add_customer(Customer::new(1, "Ana")).unwrap();
        store
            .add_product(Product::new(10, "Notebook", "Eletronicos", 3500.0))
            .unwrap();
        store
            .add_product(Product::new(11, "Mouse", "Eletronicos", 120.0))
            .unwrap();
        store
            .add_product(Product::new(12, "Livro Rust", "Livros", 95.0))
            .unwrap();
        store.register_purchase(1, 10, 1.0).unwrap();
        store.connect_products(10, 11, 0.9).unwrap();
        store.connect_products(11, 12, 0.4).unwrap();
        store
    }

    #[test]
    fn product_can_be_registered_and_queried() {
        let store = sample_store();
        assert_eq!(store.product(11).unwrap().name, "Mouse");
    }

    #[test]
    fn duplicate_product_is_rejected() {
        let mut store = sample_store();
        let result = store.add_product(Product::new(10, "Outro", "X", 1.0));
        assert_eq!(result, Err(StoreError::DuplicateProduct(10)));
    }

    #[test]
    fn purchased_product_is_not_recommended() {
        let store = sample_store();
        let recommendations = store.recommend_for_customer(1, 3, 10).unwrap();
        assert!(recommendations.iter().all(|item| item.product.id != 10));
        assert!(recommendations.iter().any(|item| item.product.id == 11));
    }

    #[test]
    fn recommendations_are_unique() {
        let mut store = sample_store();
        store.register_interest(1, 11, 0.8).unwrap();
        let recommendations = store.recommend_for_customer(1, 3, 10).unwrap();
        let ids: HashSet<_> = recommendations.iter().map(|item| item.product.id).collect();
        assert_eq!(ids.len(), recommendations.len());
    }

    #[test]
    fn product_recommendation_excludes_origin() {
        let store = sample_store();
        let recommendations = store.recommend_from_product(11, 2, 10).unwrap();
        assert!(recommendations.iter().all(|item| item.product.id != 11));
    }

    #[test]
    fn empty_limit_returns_empty_result() {
        let store = sample_store();
        assert!(store.recommend_for_customer(1, 3, 0).unwrap().is_empty());
    }
}
