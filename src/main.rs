use conectastore::benchmark::run_volume;
use conectastore::{Customer, Product, Store};
use std::env;

fn main() {
    let command = env::args().nth(1).unwrap_or_else(|| "demo".to_string());
    match command.as_str() {
        "demo" => demo(),
        "benchmark" => benchmark(),
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("Comando desconhecido: {other}\n");
            print_help();
            std::process::exit(2);
        }
    }
}

fn demo() {
    let mut store = Store::new();
    store.add_customer(Customer::new(1, "Nicolas")).unwrap();
    let products = [
        Product::new(101, "Notebook Pro", "Eletronicos", 4599.90),
        Product::new(102, "Mouse Ergonomico", "Eletronicos", 149.90),
        Product::new(103, "Teclado Mecanico", "Eletronicos", 329.90),
        Product::new(104, "Livro Rust", "Livros", 99.90),
        Product::new(105, "Suporte para Notebook", "Moveis", 189.90),
    ];
    for product in products {
        store.add_product(product).unwrap();
    }
    store.register_purchase(1, 101, 1.0).unwrap();
    store.register_interest(1, 104, 0.8).unwrap();
    store.connect_products(101, 102, 0.95).unwrap();
    store.connect_products(101, 103, 0.90).unwrap();
    store.connect_products(101, 105, 0.85).unwrap();
    store.connect_products(104, 103, 0.30).unwrap();

    println!("ConectaStore - demonstracao real");
    println!("Produtos cadastrados: {}", store.product_count());
    println!("Clientes cadastrados: {}", store.customer_count());
    println!("Consulta do produto 102: {:?}", store.product(102).unwrap());
    println!("\nRecomendacoes para o cliente 1:");
    for recommendation in store.recommend_for_customer(1, 3, 5).unwrap() {
        println!("- {recommendation}");
    }
    println!("\nRecomendacoes a partir do produto 101:");
    for recommendation in store.recommend_from_product(101, 2, 5).unwrap() {
        println!("- {recommendation}");
    }
}

fn benchmark() {
    println!("ConectaStore - benchmark basico (modo release recomendado)");
    println!("volume_produtos,arestas_direcionadas,construcao_ms,consulta_us,recomendacoes");
    for volume in [1_000usize, 10_000, 50_000] {
        let result = run_volume(volume);
        println!(
            "{},{},{:.3},{:.3},{}",
            result.products,
            result.directed_edges,
            result.build_time.as_secs_f64() * 1_000.0,
            result.query_time.as_secs_f64() * 1_000_000.0,
            result.recommendations
        );
    }
}

fn print_help() {
    println!("Uso: cargo run --release -- <comando>");
    println!("  demo       executa cadastro, consulta e recomendacoes");
    println!("  benchmark  mede construcao e consulta em tres volumes");
}
