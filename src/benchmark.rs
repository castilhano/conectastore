use crate::{Customer, Product, Store};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct BenchmarkResult {
    pub products: usize,
    pub directed_edges: usize,
    pub build_time: Duration,
    pub query_time: Duration,
    pub recommendations: usize,
}

pub fn run_volume(products: usize) -> BenchmarkResult {
    let build_start = Instant::now();
    let mut store = Store::new();
    store
        .add_customer(Customer::new(1, "Cliente benchmark"))
        .unwrap();

    for id in 0..products as u64 {
        store
            .add_product(Product::new(
                id,
                format!("Produto {id}"),
                format!("Categoria {}", id % 50),
                10.0 + (id % 500) as f64,
            ))
            .unwrap();
    }

    if products > 0 {
        store.register_purchase(1, 0, 1.0).unwrap();
    }
    for id in 0..products as u64 {
        if id + 1 < products as u64 {
            store.connect_products(id, id + 1, 0.9).unwrap();
        }
        if id + 7 < products as u64 {
            store.connect_products(id, id + 7, 0.6).unwrap();
        }
    }
    let build_time = build_start.elapsed();

    let query_start = Instant::now();
    let recommendations = store
        .recommend_for_customer(1, 4, 10)
        .expect("cliente de benchmark existe");
    let query_time = query_start.elapsed();

    BenchmarkResult {
        products,
        directed_edges: store.graph().edge_count(),
        build_time,
        query_time,
        recommendations: recommendations.len(),
    }
}
