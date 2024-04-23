## Pré-requisitos
Para executar este projeto, você precisa ter o Rust instalado em sua máquina. Você pode instalar o Rust seguindo as instruções em https://www.rust-lang.org/tools/install.

## Executando o projeto
Para executar o projeto, você precisa entrar em cada subprojeto e executar o comando `cargo run`. Será necessário três terminais. A ordem de execução é importante:

- Acesse o diretório name-server e execute cargo run.
```
    cd name-server &&
    cargo run
```

- Acesse o diretório server-math e execute cargo run.
```
    cd server-math &&
    cargo run
```
Acesse o diretório client e execute cargo run.
```
    cd client &&
    cargo run
```

## Observações
Os subprojetos name-server e server-math precisam estar em execução antes que o client possa se conectar a eles.
O name-server estará escutando em `127.0.0.1:8080`.
O server-math estará escutando em `127.0.0.2:8181`.
O client se conectará ao name-server e depois ao server-math.