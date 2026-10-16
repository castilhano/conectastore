# ConectaStore

Sistema de recomendação de produtos da MegaStore desenvolvido em Rust para a disciplina Data Structure Strategy and Implementation do curso de ADS da UniFECAF.

Autor: Nicolas Machado Castilhano

## Objetivo e funcionamento

O projeto representa clientes, produtos e categorias como vértices de um grafo. Compras, interesses, similaridades e pertencimento à categoria são arestas ponderadas. A partir de um cliente ou produto, uma busca em largura (BFS) encontra produtos próximos, acumula relevância pelo peso e pela distância e devolve uma lista ordenada sem repetições.

## Tecnologias e estruturas utilizadas

- Rust 2021, sem dependencias externas;
- `HashMap` para catálogo, clientes, lista de adjacência, compras e pontuações;
- `HashSet` para visitados, compras e eliminação de duplicidades;
- `VecDeque` como fila da BFS;
- `Vec` para listas compactas de arestas e resultado ordenado;
- lista de adjacência para um grafo heterogêneo, ponderado, esparso e com relações dirigidas armazenadas nos dois sentidos quando a navegação é bidirecional.

## Arquitetura

```text
src/
|- model.rs      entidades Product, Customer, Node e Recommendation
|- graph.rs      lista de adjacência, arestas, pesos e relações
|- store.rs      casos de uso, validações e algoritmo BFS
|- benchmark.rs  geração determinística e medição por volume
|- lib.rs        API pública da biblioteca
`- main.rs       demonstração e benchmark por linha de comando
tests/
`- integration.rs fluxos completos de integracao
```

Fluxo da recomendação:

```text
Cliente ou produto -> fila BFS -> vizinhos por nivel
                  -> HashSet de visitados
                  -> candidatos nao comprados
                  -> score = soma(peso / distância)
                  -> ordenação -> limite solicitado
```

## Compilação e execução

Requisito: toolchain Rust estável com Cargo.

```bash
cargo build --release
cargo run --release -- demo
```

No Windows com o toolchain MSVC, `link.exe` precisa estar disponível; com o Visual Studio C++ Build Tools configurado, os comandos acima funcionam normalmente. Na máquina usada nesta entrega, `link.exe` não está instalado. Como este projeto não usa bibliotecas C externas, uma cópia limpa foi compilada com o toolchain `stable-x86_64-pc-windows-gnullvm` e o `rust-lld` incluído nele. No PowerShell, dentro da pasta do projeto, execute:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnullvm
rustup component add rust-mingw --toolchain stable-x86_64-pc-windows-gnullvm
$conectastoreSysroot = rustc +stable-x86_64-pc-windows-gnullvm --print sysroot
$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNULLVM_LINKER = Join-Path $conectastoreSysroot 'lib\rustlib\x86_64-pc-windows-gnullvm\bin\rust-lld.exe'
cargo +stable-x86_64-pc-windows-gnullvm build --release
cargo +stable-x86_64-pc-windows-gnullvm run --release -- demo
cargo +stable-x86_64-pc-windows-gnullvm test
cargo +stable-x86_64-pc-windows-gnullvm run --release -- benchmark
```

A verificação foi feita a partir de uma cópia limpa do ZIP, com 12 testes aprovados e demonstração e benchmark executados. A variável de ambiente acima vale apenas para a sessão atual do PowerShell. Essa opção para programas Rust puros é descrita na [documentação oficial do target Windows gnullvm](https://doc.rust-lang.org/rustc/platform-support/windows-gnullvm.html).

Ajuda dos comandos:

```bash
cargo run --release -- help
```

## Testes

```bash
cargo test
```

Os testes unitários ficam junto aos módulos e verificam grafo, validações, cadastro, consulta, exclusão de itens comprados e ausência de duplicidades. Os testes de integração validam fluxos completos, profundidade da BFS, erros e conexões por categoria.

## Benchmark

```bash
cargo run --release -- benchmark
```

O programa mede separadamente a construção do conjunto e uma consulta BFS, nos volumes de 1.000, 10.000 e 50.000 produtos. Os dados são determinísticos, com 50 categorias e até duas relações de similaridade por produto. Tempos variam conforme máquina, sistema operacional e carga.

### Resultados medidos neste ambiente

<!-- BENCHMARK_RESULTS_START -->
Execução real em 14/10/2026, Windows 11, Rust 1.99.0, perfil `release`:

| Produtos | Arestas direcionadas | Construção | Consulta BFS | Recomendações |
|---:|---:|---:|---:|---:|
| 1.000 | 5.986 | 1,428 ms | 55,100 us | 10 |
| 10.000 | 59.986 | 14,979 ms | 651,400 us | 10 |
| 50.000 | 299.986 | 92,243 ms | 3.320,800 us | 10 |

Os valores representam uma execução pontual neste ambiente e não constituem garantia para outras máquinas. A consulta cresce com a parcela do grafo alcançada pela profundidade configurada; a construção cresce de forma aproximadamente linear no conjunto gerado.
<!-- BENCHMARK_RESULTS_END -->

## Evidências de execução

A pasta `docs/evidencias/` contém os logs completos da demonstração, dos testes e do benchmark. As imagens são visualizações produzidas a partir dessas saídas reais, não capturas diretas da tela do terminal.

| Execução | Log completo | Visualização do log |
|---|---|---|
| Demonstração de cadastro, consulta e recomendações | [demo.log](docs/evidencias/demo.log) | [Ver imagem](docs/evidencias/execucao-demo.png) |
| Testes unitários e de integração | [cargo-test.log](docs/evidencias/cargo-test.log) | [Ver imagem](docs/evidencias/execucao-testes.png) |
| Benchmark em três volumes | [benchmark.log](docs/evidencias/benchmark.log) | [Ver imagem](docs/evidencias/execucao-benchmark.png) |

## Exemplos de uso da API

```rust
use conectastore::{Customer, Product, Store, StoreError};

fn main() -> Result<(), StoreError> {
    let mut store = Store::new();
    store.add_customer(Customer::new(1, "Ana"))?;
    store.add_product(Product::new(10, "Notebook", "Eletronicos", 3500.0))?;
    store.add_product(Product::new(11, "Mouse", "Eletronicos", 120.0))?;
    store.register_purchase(1, 10, 1.0)?;
    store.connect_products(10, 11, 0.9)?;

    let itens = store.recommend_for_customer(1, 3, 5)?;
    for item in itens {
        println!("{}: score {:.3}", item.product.name, item.score);
    }
    Ok(())
}
```

## Complexidade resumida

- consulta por ID no catálogo: tempo médio `O(1)`;
- inserção em mapa ou conjunto: tempo médio `O(1)`;
- BFS: `O(V + E)` no subgrafo alcançado;
- memória do grafo em lista de adjacência: `O(V + E)`;
- ordenação de `R` candidatos: `O(R log R)`.

## Links de entrega

- Repositório público: https://github.com/castilhano/conectastore
- Vídeo pitch: https://youtu.be/Rfa0gWaJ-YU

O relatório técnico em PDF é entregue separadamente. Antes da entrega final, confirme que o repositório e o vídeo estão acessíveis pelos links enviados ao professor.

## Limitações e evolução

Esta versão didática mantém os dados em memória e usa pesos definidos no cadastro. Em produção, o armazenamento poderia ser persistente e particionado; eventos poderiam atualizar pesos; e consultas poderiam ser cacheadas. Para grafos muito densos ou recomendação global, técnicas de amostragem, processamento distribuído e modelos de ranking seriam avaliados.
