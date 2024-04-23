use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

static NAME_SERVER_ADDR: &str = "127.0.0.1:8080";

static CLIENTREQUEST: u8 = 0b0000_0001;
// SERVICE_NAME = MATH
static SERVICE_NAME: [u8; 4] = [0b01001101, 0b01000001, 0b01010100, 0b01001000];

fn stub_serach() -> String {
    let mut stream: TcpStream;

    stream = match TcpStream::connect(NAME_SERVER_ADDR) {
        Ok(stream) => {
            println!("Client conectado ao servidor de endereço {NAME_SERVER_ADDR}");
            stream
        }
        Err(e) => {
            eprintln!("Erro durante a tentativa de conexão com o servidor {e}");
            return "".to_string();
        }
    };

    let mut request: [u8; 11] = [0; 11];

    request[0] = CLIENTREQUEST;
    request[1..5].copy_from_slice(&SERVICE_NAME);

    match stream.write(&request[..]) {
        Ok(_) => {
            println!("Mensagem enviada!");
        }
        Err(e) => eprintln!("Erro durante o enviado da mensagem: {e}"),
    }

    let mut response: [u8; 10] = [0; 10];

    let mut address: String = "".to_string();

    match stream.read(&mut response) {
        Ok(_) => {
            if &response[0] == &44_u8 {
                println!("Serviço não encontrado no servidor de nomes!: {:?}", &response);
            } else {
                println!("Servico encontrado: {:?}", &response);
                let service_name = String::from_utf8_lossy(&response[..4]).to_string();

                let ip = &response[4..8]
                    .iter()
                    .map(|&b| b.to_string())
                    .collect::<Vec<String>>()
                    .join(".");

                let port = &response[8..10]
                    .iter()
                    .map(|&b| b.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                address = format!("{}:{}", ip.to_string(), port.to_string());

                println!("Service Name: {}; Address: {}", service_name, address);
            }
        }
        Err(e) => eprintln!("Erro durante a leitura da respostas: {e}"),
    }
    stream.shutdown(Shutdown::Both).unwrap();

    address
}

fn main() {
    let address: String = stub_serach();
    if address == "" {
        eprintln!("Erro ao encontrar o serviço solicitado!");
        return ();
    }

    let mut stream: TcpStream;

    stream = match TcpStream::connect(&address) {
        Ok(stream) => {
            println!("Client conectado ao servidor de endereço {}", address);
            stream
        }
        Err(e) => {
            eprintln!("Erro durante a tentativa de conexão com o servidor {e}");
            return ();
        }
    };

    let request = "1 + 1";

    match stream.write(request.as_bytes()) {
        Ok(_) => {
            println!("Mensagem enviada!");
        }
        Err(e) => eprintln!("Erro durante o enviado da mensagem: {e}"),
    }

    let mut response: [u8; 10] = [0; 10];

    match stream.read(&mut response) {
        Ok(_) => {
            let response_str = String::from_utf8_lossy(&response).to_string();
            println!("Resposta recebida do serviço: {}", response_str);
        }
        Err(e) => eprintln!("Erro durante a leitura da respostas: {e}"),
    }

    stream.shutdown(Shutdown::Both).unwrap();
}
