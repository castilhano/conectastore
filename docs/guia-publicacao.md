# Guia de gravacao e publicacao

## Antes de gravar

1. Extraia o ZIP e abra um terminal na pasta `megastore`.
2. Confirme `cargo --version` e execute `cargo test`.
3. Execute `cargo run --release -- demo` e `cargo run --release -- benchmark`.
4. Feche notificacoes e arquivos pessoais; use fonte de terminal legivel.
5. Abra previamente `src/store.rs`, o relatorio e o README.

## Gravacao

1. Grave em 1080p, se disponivel, com o cursor visivel.
2. Siga `docs/roteiro-video.md` e mostre comandos sendo digitados e executados.
3. Nao corte a saida de testes para parecer aprovada. Se houver falha, corrija e grave novamente.
4. Mantenha o video abaixo de 4 minutos.

## YouTube

1. Entre no YouTube Studio e selecione Criar > Enviar videos.
2. Envie o arquivo gravado.
3. Use titulo claro, por exemplo: `ConectaStore - Pitch Data Structure Strategy and Implementation`.
4. Defina a visibilidade como Nao listado ou Publico.
5. Copie a URL, abra em janela anonima e confirme acesso.
6. Envie a URL do vídeo separadamente junto do relatório técnico e do link do repositório.

## GitHub

O projeto está publicado em https://github.com/castilhano/conectastore. Abra a página pública em uma janela anônima para confirmar que o README aparece. Envie essa URL separadamente junto do PDF e do vídeo.

## Checklist final

- Repositorio publico abre sem login.
- Video abre sem solicitar permissao.
- PDF identifica autor, curso e projeto.
- README contem comandos, arquitetura, exemplos, benchmark e links.
- ZIP nao contem a pasta `target` nem arquivos pessoais.
