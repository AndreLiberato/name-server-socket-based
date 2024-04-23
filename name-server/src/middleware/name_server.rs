use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Clone)]
pub struct Server {
    ip: String,
    port: String,
}

#[derive(Clone)]
pub struct ServerDescription {
    service_name: String,
    server: Server,
}

type ServerList = Vec<ServerDescription>;

pub struct ClientRequest {
    service_name: String,
}

impl ClientRequest {
    pub fn new(ct: &[u8]) -> ClientRequest {
        let service_name: String = String::from_utf8_lossy(&ct[..4]).to_string();

        ClientRequest { service_name }
    }
}

impl ServerDescription {
    pub fn new(ct: &[u8]) -> ServerDescription {
        let service_name: String = String::from_utf8_lossy(&ct[..4]).to_string();

        let ip = &ct[4..8]
            .iter()
            .map(|&b| b.to_string())
            .collect::<Vec<String>>()
            .join(".");

        let port = &ct[8..10]
            .iter()
            .map(|&b| b.to_string())
            .collect::<Vec<String>>()
            .join("");

        let sd: ServerDescription = ServerDescription {
            service_name,
            server: Server {
                ip: ip.to_string(),
                port: port.to_string(),
            },
        };

        sd
    }

    pub fn to_bytes(sd: ServerDescription) -> [u8; 10] {
        let mut response: [u8; 10] = [0; 10];

        let name_b: &[u8] = sd.service_name.as_bytes();

        let ip: Vec<u8> = sd
            .server
            .ip
            .split('.')
            .map(|s| s.parse().unwrap())
            .collect();

        let port: [u8; 2] = [
            sd.server.port[0..2].parse().unwrap(), // Converta os primeiros dois caracteres em u8
            sd.server.port[2..4].parse().unwrap(), // Converta os próximos dois caracteres em u8
        ];

        response[..4].copy_from_slice(name_b);
        response[4..8].copy_from_slice(&ip);
        response[8..10].copy_from_slice(&port);

        response
    }
}

pub struct Payload {
    payload_type: PayloadType,
    content: [u8; 10],
}

impl Payload {
    pub fn from_bytes(bytes: [u8; 11]) -> Result<Payload, String> {
        let payload_type: PayloadType = match bytes[0] {
            0b0000_0000 => PayloadType::ServerSubscription,
            0b0000_0001 => PayloadType::ClientRequestServer,
            _ => PayloadType::Unknown,
        };

        let mut content: [u8; 10] = [0; 10];

        content.clone_from_slice(&bytes[1..]);

        let pl: Payload = Payload {
            payload_type,
            content,
        };

        Ok(pl)
    }
}

enum PayloadType {
    ServerSubscription = 0b0000_0000,
    ClientRequestServer = 0b0000_0001,
    Unknown,
}

impl Server {
    pub fn new(server_ip: &str, server_port: &str) -> Server {
        Server {
            ip: String::from(server_ip),
            port: String::from(server_port),
        }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn start(&self) {
        let mut sl: ServerList = Vec::new();
        match TcpListener::bind(self.bind_address()) {
            Ok(listener) => {
                println!("Servidor iniciado. Escutando em {}", self.bind_address());
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            println!("Nova conexão: {}", stream.peer_addr().unwrap());
                            self.handle_client(stream, &mut sl);
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

    fn handle_client(&self, mut stream: TcpStream, service_list: &mut ServerList) {
        let mut buffer: [u8; 11] = [0; 11];
        loop {
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        // Conexão fechada
                        println!("Conexão fechada");
                        break;
                    }

                    let pl: Payload = Payload::from_bytes(buffer).unwrap();
                    println!("Mensagem recebida:");

                    match pl.payload_type {
                        PayloadType::ServerSubscription => {
                            let server_d: ServerDescription =
                                ServerDescription::new(&pl.content[..]);
                            service_list.push(server_d);
                            println!(
                                "New Service Registred - service_name: {}; ip: {}; port: {}",
                                service_list.get(0).unwrap().service_name,
                                service_list.get(0).unwrap().server.ip,
                                service_list.get(0).unwrap().server.port
                            );

                            let _ = stream.write(&[201_u8]);
                        }
                        PayloadType::ClientRequestServer => {
                            let client_r: ClientRequest = ClientRequest::new(&pl.content[..]);
                            let mut not_find: bool = true;
                            println!(
                                "Solicitação de buscar de serviço: {}",
                                client_r.service_name
                            );
                            for server in service_list.clone() {
                                if server.service_name == client_r.service_name {
                                    let b: [u8; 10] = ServerDescription::to_bytes(server);
                                    let _ = stream.write(&b);
                                    not_find = false;
                                    break;
                                }
                            }
                            if not_find {
                                let _ = stream.write(&[44]);
                            }
                        }
                        PayloadType::Unknown => {
                            eprintln!("Payload desconhecido. Impossível de processar.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Erro ao ler da conexão: {}", e);
                    break;
                }
            }
        }
    }
}
