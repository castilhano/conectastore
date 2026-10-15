# Guia de gravação e publicação do vídeo (Windows)

O relatório técnico e o repositório público estão prontos. Falta gravar um vídeo de até 4 minutos com a execução real, publicá-lo em um endereço acessível e acrescentar esse endereço ao README. Neste computador, Rust/Cargo e OBS Studio já estão instalados; não é necessário instalar outro programa.

## 1. Abrir o projeto e conferir o terminal

1. Abra a pasta `megastore` que contém `Cargo.toml`. Se estiver usando o ZIP, extraia-o antes; não execute comandos dentro do arquivo ZIP.
2. No Explorador de Arquivos, clique na barra de endereço, digite `powershell` e pressione Enter. O terminal abrirá nessa pasta.
3. Digite `Test-Path Cargo.toml` e pressione Enter. O resultado deve ser `True`. Se aparecer `False`, volte à pasta que contém `Cargo.toml`.
4. Digite `cargo --version`. Neste computador, o Rust/Cargo já está instalado.

O toolchain MSVC padrão deste computador não encontra `link.exe`. Use os comandos completos abaixo, com `+stable-x86_64-pc-windows-gnullvm`; esse toolchain já está instalado. Não é necessário alterar a configuração global do Rust.

```powershell
cargo +stable-x86_64-pc-windows-gnullvm test
cargo +stable-x86_64-pc-windows-gnullvm run --release -- demo
cargo +stable-x86_64-pc-windows-gnullvm run --release -- benchmark
```

Faça essa execução de ensaio antes de gravar. Os testes devem terminar sem falhas; a demonstração deve mostrar recomendações para um cliente e para um produto; o benchmark deve mostrar três linhas, com 1.000, 10.000 e 50.000 produtos. Os tempos do benchmark podem mudar entre execuções e computadores.

## 2. Preparar o OBS Studio

1. Abra o menu Iniciar do Windows, procure `OBS Studio` e abra o aplicativo. Se aparecer o assistente de configuração, escolha a opção voltada a gravação, não transmissão ao vivo.
2. Em **Fontes**, clique em `+` e adicione **Captura de tela** (ou **Display Capture**). Selecione o monitor em que mostrará o terminal. Confirme que a prévia exibe a tela correta.
3. Fale algumas palavras e observe o medidor **Mic/Aux** no **Mixer de áudio**. Se ele não se mover, abra **Configurações > Áudio** e selecione o microfone correto. Use um fone com microfone se o computador não tiver um microfone funcional.
4. Em **Configurações > Saída > Gravação**, escolha uma pasta fácil de localizar. Se houver a opção **Hybrid MP4**, ela gera um arquivo adequado para envio. Se o OBS gravar em `.mkv`, depois use **Arquivo > Remux Recordings** para gerar `.mp4`.
5. Grave um teste de 10 a 15 segundos, pare, abra o arquivo salvo e confira se a tela e a sua voz aparecem. Ajuste o tamanho da fonte do terminal para que o professor consiga ler.

Antes da gravação definitiva, feche notificações, conversas e arquivos pessoais. Deixe abertos o terminal na pasta do projeto, o relatório e, se desejar, os arquivos `src/model.rs`, `src/graph.rs` e `src/store.rs`.

## 3. Gravar o pitch

No OBS, clique em **Iniciar gravação**. Siga os tempos e falas de `docs/roteiro-video.md`. Mostre comandos sendo digitados e executados no terminal; a rubrica exige a execução real do projeto, não apenas slides ou leitura do PDF.

Na demonstração, execute `demo`, depois mostre rapidamente o algoritmo em `src/store.rs`, execute `test` e encerre com `benchmark`. Fale apenas de resultados que realmente aparecem na sua gravação. Ao terminar, clique em **Parar gravação** e abra o vídeo para conferir áudio, legibilidade e duração menor ou igual a 4 minutos. Se houver falha ou passar do tempo, grave outra tomada.

## 4. Publicar e verificar o vídeo

1. Entre em https://studio.youtube.com com sua conta do YouTube.
2. Clique em **Criar > Enviar vídeos** e selecione o arquivo `.mp4` gravado.
3. Use um título claro, como `ConectaStore - Pitch Data Structure Strategy and Implementation`. Na descrição, pode colocar o repositório: https://github.com/castilhano/conectastore.
4. Responda à pergunta de público do YouTube de acordo com o conteúdo do vídeo. Na etapa **Visibilidade**, selecione **Não listado** ou **Público** e conclua em **Salvar/Publicar**. Não deixe **Privado**, pois o avaliador precisa abrir o link.
5. Copie a URL do vídeo. Abra-a em uma janela anônima do navegador e confirme que reproduz sem entrar na sua conta.
6. Envie a URL do vídeo para atualizar o campo `Vídeo pitch` no README do GitHub. O link do vídeo precisa aparecer no README antes da entrega.

## 5. Entregar ao professor

Envie os três itens pedidos na rubrica: o relatório técnico em PDF, o link público https://github.com/castilhano/conectastore e o link acessível do vídeo. O PDF não precisa conter esses links; eles podem ser enviados separadamente na plataforma da disciplina.
