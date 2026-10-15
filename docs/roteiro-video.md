# Roteiro do video pitch

Duracao maxima: 4 minutos. Meta sugerida: 3 minutos e 40 segundos.

Neste computador Windows, use os comandos abaixo com o toolchain `stable-x86_64-pc-windows-gnullvm`, que ja esta instalado. O comando `cargo` sem essa indicacao pode falhar por falta do vinculador `link.exe` do toolchain MSVC padrao.

## 0:00 a 0:25 Problema

Fala: "A MegaStore possui um catalogo amplo, mas recomendacoes genericas dificultam a descoberta e reduzem oportunidades de venda cruzada. O ConectaStore usa as relacoes entre clientes, produtos e categorias para encontrar itens relevantes."

Na tela: titulo do projeto e, em seguida, o diagrama de arquitetura do relatorio.

## 0:25 a 1:05 Modelagem e estruturas

Fala: "Modelei um grafo heterogeneo e ponderado. Clientes, produtos e categorias sao vertices. Compras, interesses, similaridades e pertencimento a categoria sao arestas. Usei lista de adjacencia porque o grafo e esparso, HashMap para acesso medio constante por identificador, HashSet para visitados e duplicidades, e VecDeque como fila da BFS."

Na tela: abra `src/model.rs`, `src/graph.rs` e `src/store.rs`, destacando os tipos correspondentes.

## 1:05 a 2:05 Demonstracao real

Na tela: terminal dentro da pasta do projeto.

Execute:

```bash
cargo +stable-x86_64-pc-windows-gnullvm run --release -- demo
```

Fala: "A demonstracao cadastra um cliente e cinco produtos, consulta um item e cria conexoes. A BFS recomenda a partir do cliente e de um produto. O item ja comprado nao aparece e cada produto aparece uma unica vez."

Mostre a saida completa e a ordem das recomendacoes.

## 2:05 a 2:45 Algoritmo

Fala: "A busca percorre o grafo por niveis ate a profundidade configurada. O HashSet impede ciclos. Produtos ja comprados sao excluidos. A pontuacao soma o peso da aresta dividido pela distancia, favorecendo relacoes fortes e proximas; ao final, os candidatos sao ordenados por score, distancia e ID para manter o resultado deterministico."

Na tela: mostre `bfs_recommend` em `src/store.rs`.

## 2:45 a 3:15 Testes

Execute:

```bash
cargo +stable-x86_64-pc-windows-gnullvm test
```

Fala: "Os testes unitarios e de integracao cobrem cadastro, consulta, validacoes, profundidade, ranking, exclusao de compras e duplicidades. Nesta gravacao, todos os testes passam."

Nao diga que passaram se a sua execucao no momento da gravacao mostrar falha.

## 3:15 a 3:40 Desempenho e encerramento

Execute:

```bash
cargo +stable-x86_64-pc-windows-gnullvm run --release -- benchmark
```

Fala: "O benchmark mede construcao e consulta com 1 mil, 10 mil e 50 mil produtos. A lista de adjacencia ocupa O de V mais E, e a BFS tambem custa O de V mais E no subgrafo visitado. Os tempos mostrados sao desta maquina e podem variar. O repositorio e o relatorio documentam como reproduzir tudo."

Na tela: mostre os tres resultados e finalize exibindo a URL do repositorio. O link do proprio video so existira depois da publicacao.
