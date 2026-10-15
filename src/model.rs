use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: u64, name: impl Into<String>, category: impl Into<String>, price: f64) -> Self {
        Self {
            id,
            name: name.into(),
            category: category.into(),
            price,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Customer {
    pub id: u64,
    pub name: String,
}

impl Customer {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Node {
    Product(u64),
    Customer(u64),
    Category(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Recommendation {
    pub product: Product,
    pub score: f64,
    pub distance: usize,
}

impl fmt::Display for Recommendation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (id {}, score {:.3}, distancia {})",
            self.product.name, self.product.id, self.score, self.distance
        )
    }
}
