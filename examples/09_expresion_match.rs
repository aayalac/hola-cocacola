enum ConnectionState{
    Connected,
    Disconnected,
    Connecting,
}

fn main(){
describe_state(ConnectionState::Connected);
}

fn describe_state(state: ConnectionState) {
    match state {
        ConnectionState::Connected => println!("Estado: Conectado"),
        ConnectionState::Disconnected => println!("Estado: Desconectado"),
        ConnectionState::Connecting => println!("Estado: Conectando"),
    }
}