use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;

fn main() -> std::io::Result<()> {
    let echo = async {
        let listener = match TcpListener::bind("0.0.0.0:8080").await {
            Ok(listener) => listener,
            Err(_) => return (),
        };
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error {}", e);
                    continue;
                }
            };
            let (reader, writer) = &mut (&stream, &stream);
            match io::copy(reader, writer).await {
                Ok(_) => continue,
                Err(e) => {
                    eprintln!("Error {}", e);
                    continue;
                }
            };
        }
    };

    futures::executor::block_on(echo);

    Ok(())
}
