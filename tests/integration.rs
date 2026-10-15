use conectastore::{Customer, Product, Store, StoreError};
use std::collections::HashSet;

fn connected_store() -> Store {
    let mut store = Store::new();
    store.add_customer(Customer::new(7, "Cliente 7")).unwrap();
    for id in 1..=6 {
        store
            .add_product(Product::new(
                id,
                format!("Produto {id}"),
                if id <= 4 { "Tecnologia" } else { "Livros" },
                id as f64 * 10.0,
            ))
            .unwrap();
    }
    store.register_purchase(7, 1, 1.0).unwrap();
    store.connect_products(1, 2, 0.9).unwrap();
    store.connect_products(1, 3, 0.8).unwrap();
    store.connect_products(2, 4, 0.7).unwrap();
    store.connect_products(3, 4, 0.6).unwrap();
    store.connect_products(4, 5, 0.5).unwrap();
    store
}

#[test]
fn full_customer_flow_returns_ranked_unique_products() {
    let store = connected_store();
    let recommendations = store.recommend_for_customer(7, 3, 10).unwrap();
    let ids: Vec<_> = recommendations.iter().map(|item| item.product.id).collect();
    let unique: HashSet<_> = ids.iter().copied().collect();

    assert_eq!(ids.len(), unique.len());
    assert!(!ids.contains(&1));
    assert_eq!(ids.first(), Some(&2));
}

#[test]
fn depth_limit_changes_reachability() {
    let store = connected_store();
    let shallow = store.recommend_from_product(1, 1, 10).unwrap();
    let deep = store.recommend_from_product(1, 3, 10).unwrap();
    assert!(shallow.len() < deep.len());
}

#[test]
fn missing_entities_return_explicit_errors() {
    let mut store = connected_store();
    assert_eq!(
        store.register_purchase(999, 1, 1.0),
        Err(StoreError::CustomerNotFound(999))
    );
    assert_eq!(
        store.connect_products(1, 999, 0.5),
        Err(StoreError::ProductNotFound(999))
    );
}

#[test]
fn category_vertices_connect_products_without_duplicates() {
    let store = connected_store();
    let recommendations = store.recommend_from_product(2, 2, 20).unwrap();
    let ids: HashSet<_> = recommendations.iter().map(|item| item.product.id).collect();
    assert_eq!(ids.len(), recommendations.len());
    assert!(ids.contains(&3));
}
