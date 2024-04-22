use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

static NAME_SERVER_ADDR: &str = "127.0.0.1:8080";

static SERVER_ADDR: &str = "127.0.0.2:8181";

static SERVERSUB: u8 = 0b0000_0000;

// SERVICE_NAME = MATH
static SERVICE_NAME: [u8; 4] = [0b01001101, 0b01000001, 0b01010100, 0b01001000];
// 127 0 0 2
static IP: [u8; 4] = [0b01111111, 0b00000000, 0b00000000, 0b00000010];
// 81 81
static PORT: [u8; 2] = [0b01010001, 0b01010001];

fn register() {
    let mut stream: TcpStream;

    stream = match TcpStream::connect(NAME_SERVER_ADDR) {
        Ok(stream) => {
            println!("Client conectado ao servidor de endereço {NAME_SERVER_ADDR}");
            stream
        }
        Err(e) => {
            eprintln!("Erro durante a tentativa de conexão com o servidor {e}");
            return ();
        }
    };

    let mut request: [u8; 11] = [0; 11];

    request[0] = SERVERSUB;
    request[1..5].copy_from_slice(&SERVICE_NAME);
    request[5..9].copy_from_slice(&IP);
    request[9..11].copy_from_slice(&PORT);

    match stream.write(&request[..]) {
        Ok(_) => {
            println!("Mensagem enviada!");
        }
        Err(e) => eprintln!("Erro durante o enviado da mensagem: {e}"),
    }

    let mut response: [u8; 1] = [0];

    match stream.read(&mut response) {
        Ok(_) => {
            if &response == &[201_u8] {
                println!("Serviço cadastrado com sucesso no servidor de nomes!");
            } else {
                println!(
                    "Erro ao cadastrar o serviço no servirdor de nomes!: {:?}",
                    &response
                );
            }
        }
        Err(e) => eprintln!("Erro durante a leitura da respostas: {e}"),
    }

    stream.shutdown(Shutdown::Both).unwrap();
}

pub fn start() {
    match TcpListener::bind(SERVER_ADDR) {
        Ok(listener) => {
            println!("Servidor iniciado. Escutando em {}", SERVER_ADDR);
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("Nova conexão: {}", stream.peer_addr().unwrap());
                        handle_client(stream);
                    }
                    Err(e) => {
                        eprintln!("Erro ao aceitar a conexão: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Erro ao tentar atribuir endereço IP ao servidor: {e}");
        }
    };
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Conexão fechada pelo cliente
                    println!(
                        "Conexão fechada pelo cliente {}",
                        stream.peer_addr().unwrap()
                    );
                    break;
                }
                let request: String = String::from_utf8_lossy(&buffer).to_string();
                println!("Solicitação recebida: {}", request);
                println!("Reenviando solicitação");
                let _ = stream.write(&buffer);
            }
            Err(e) => {
                eprintln!("Erro ao ler da conexão: {}", e);
                break;
            }
        }
    }
}
fn main() {
    register();
    start();
}
