use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

//Endereço do servidor de nomes
static ADDRNS: &str = "127.0.0.1:8080";

pub fn search_server(mut server: String) -> String{
    let mut stream: TcpStream = match TcpStream::connect(ADDRNS) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Erro durante a tentativa de conexão com o servidor {e}");
            return ();
        }
    };

    println!("Client conecatado ao servidor de endereço {ADDRNS}");

    let mut buffer: [u8; 1500] = [0; 1500];

    match io::stdin().read_line(&mut server) {
        Ok(_) => {
            println!("Enviando mensagem!");

            // Envio da mensagem através da stream
            match stream.write(server.as_bytes()) {
                Ok(_) => {
                    println!("Mensagem enviada!");
                }
                Err(e) => eprintln!("Erro durante o enviado da mensagem: {e}"),
            }
        }
        Err(e) => eprintln!("Erro na captura do input: {e}"),
    }

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Mensagem recebida do server!");
        }
        Err(e) => eprintln!("Erro durante a leitura da mensagem: {e}"),
    }

    let address: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

    return address;

}


pub fn connect_server(mut addr: String){
    let mut stream: TcpStream = match TcpStream::connect(addr) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Erro durante a tentativa de conexão com o servidor {e}");
            return ();
        }
    };

    println!("Client conectado ao servidor de endereço {addr}");

    let mut input: String = String::new();

    // 1500 bytes -> Tamanho máximo do frame Ethernet
    let mut buffer: [u8; 1500] = [0; 1500];

    loop {
        // Captura do input
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Enviando mensagem!");

                // Envio da mensagem através da stream
                match stream.write(input.as_bytes()) {
                    Ok(_) => {
                        println!("Mensagem enviada!");
                    }
                    Err(e) => eprintln!("Erro durante o enviado da mensagem: {e}"),
                }
            }
            Err(e) => eprintln!("Erro na captura do input: {e}"),
        }

        // Receber a resposta do servidor
        // Consome as informações da stream e adiciona no buffer
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Mensagem recebida do server!");
            }
            Err(e) => eprintln!("Erro durante a leitura da mensagem: {e}"),
        }

        let request: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer);

        println!("Requisição recebida: {request}");
    }
}
fn main(){}
