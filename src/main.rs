use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let server = match TcpListener::bind("localhost:8080").await {
        Ok(server) => server,
        Err(err) => {
            eprintln!("TCPBindingError: Could not start the server at port 8080 \n Details: {err}");
            return;
        }
    };

    loop {
        let (mut socket, _) = match server.accept().await {
            Ok(server) => server,
            Err(err) => {
                eprintln!(
                "SocketAcceptError: Could not accept any new connection in the provided socket. \n Details: {err}");
                return;
            }
        };

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();

                if bytes_read == 0 {
                    break;
                }
                writer.write_all(line.as_bytes()).await.unwrap();

                line.clear();
            }
        });
    }
}
