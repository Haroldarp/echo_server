use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("No se pudo acceder al puerto");

    for stream in listener.incoming(){
        match stream{
            Ok(stream) =>{

                match handle_client(stream){
                    Ok(message) =>{
                        println!("{}", message);
                    }
                    Err(e) =>{
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) =>{
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<String, Error>{

    let mut buf = [0;1024];

    loop{
        let bytes_len = stream.read(&mut buf)?;
        if bytes_len == 0{
            return Ok("No se pueden leer mas bytes".to_string());
        }
        stream.write(&buf[..bytes_len])?;        
    }
}
