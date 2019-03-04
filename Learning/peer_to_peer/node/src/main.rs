use std::io;
use node::connector::connect;
use node::server::ini_server;

fn main() {
    loop{
        println!("Please enter 1 to initialize server or 2 to connect to server.");
        let mut user_input= String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read from stdin.");

        let user_input: u32 = user_input.trim().parse().expect("Please enter a number");

        if user_input == 1 {
            ini_server();
            break;
        } else if user_input == 2 {
            connect();
            break;
        } else {
            println!("Please enter 1 or 2 only.");
        } // if..else
    } // loop

} // main