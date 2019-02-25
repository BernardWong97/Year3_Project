use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect("127.0.0.1:6000").expect("Stream failed to connect.");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking.");

    let (sender, receiver) = mpsc::channel::<String>();

    thread::spawn(move || loop{
        let mut buffer = vec![0; MSG_SIZE];

        match client.read_exact(&mut buffer) {
            Ok(_) => {
                let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();

                println!("Message receive: {:?}", msg);
            },
            Err(ref err) if err.kind() ==  ErrorKind::WouldBlock => (), // if ErrorKind is WouldBlock, just continue
            Err(_) => {
                println!("Connection with server was terminated.");
                break;
            } // OK, Err
        } // match

        match receiver.try_recv(){
            Ok(msg) => {
                let mut buffer = msg.clone().into_bytes(); // convert messages into bytes
                buffer.resize(MSG_SIZE, 0);

                client.write_all(&buffer).expect("Failed to write to socket.");
                println!("Message sent: {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        } // match

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a message: ");

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read from stdin.");
        let msg = buffer.trim().to_string();

        if msg == ":quit" || sender.send(msg).is_err() {break}
    } // loop

    println!("Bye!");
} // main()
