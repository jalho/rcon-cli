fn main() {
    // TODO: get RCON password as CLI arg
    let url = "ws://localhost:28016/SET_ME";

    // TODO: get sendable message as CLI arg
    let sendable_message = "{\"Identifier\":1,\"Message\":\"playerlist\"}";

    // connect
    let (mut socket, _) =
        tungstenite::connect(url).expect(&format!("Failed to connect to {}", url));
    println!("Failed to connect to {}", url);

    // send message
    let message = tungstenite::Message::Text(sendable_message.into());
    socket.send(message).expect("Failed to send message.");

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
                    println!("WebSocket closed by the server.");
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
