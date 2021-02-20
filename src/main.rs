use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("No se pudo acceder al puerto");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) =>{
                println!("Esta abierto y recibiendo");
            }
            Err(e) =>{
                eprintln!("Error: {}", e);
            }
        }
    }
}
