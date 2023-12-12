// TODO: error handling :D

fn main() {
    #[derive(clap::Parser, Debug)]
    struct Args {
        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        command: String,
    }

    let args = <Args as clap::Parser>::parse();
    let url: String = format!("ws://localhost:28016/{}", args.password);
    let sendable_message: String = format!("{{\"Identifier\":1,\"Message\":\"{}\"}}", args.command);

    let result_connect = tungstenite::connect(&url);
    let mut socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>;
    match result_connect {
        Ok(v) => {
            let (s, _) = v;
            socket = s;
        }
        Err(_) => todo!(),
    }

    let message = tungstenite::Message::Text(sendable_message.into());
    socket.send(message).expect("Failed to send message");

    // TODO: wait till some timeout for a response matching the Identifier
    let receive_thread = std::thread::spawn(move || loop {
        match socket.read() {
            Ok(msg) => match msg {
                tungstenite::Message::Text(text) => {
                    println!("Received message: {}", text);
                }
                tungstenite::Message::Binary(data) => {
                    println!("Received binary data: {:?}", data);
                }
                tungstenite::Message::Close(_) => {
                    println!("WebSocket closed by the server");
                    break;
                }
                _ => {}
            },
            Err(err) => {
                eprintln!("Error reading message: {:?}", err);
                break;
            }
        }
    });

    receive_thread.join().unwrap();
}
