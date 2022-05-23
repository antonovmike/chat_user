// USER
#![allow(unused)]
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use colored::Colorize;

const LOCAL: &str = "127.0.0.1:6000"; // Connection address
const MESSAGE_SIZE: usize = 32; // Limit of chars. Extra chars would not be printed
const USER_NAME_SIZE: usize = 16;
// Example: If MESSAGE_SIZE = 8 and user send "123456789", server will print "12345678"

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("Failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff_name = vec![0;USER_NAME_SIZE];
        let mut buff_message = vec![0;MESSAGE_SIZE];
// USER'S NAME
        match client.read_exact(&mut buff_name) {
            Ok(_) => {
                println!("{}", "Your name received".bold().on_blue());
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed");
                break;
            }
        }
// USER'S MESSAGE
        match client.read_exact(&mut buff_message) {
            Ok(_) => {
                println!("{}", "Your message received".bold().on_blue());
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server was severed");
                break;
            }
        }
// USER'S NAME   
        match rx.try_recv() {
            Ok(users_name) => {
                let mut buff_name = users_name.clone().into_bytes();
                buff_name.resize(MESSAGE_SIZE, 0);
                client.write_all(&buff_name).expect("Writing to socket failed");
                println!("{} {:?}", "Name sent".bold().on_blue(), users_name);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
// USER'S MESSAGE   
        match rx.try_recv() {
            Ok(users_message) => {
                let mut buff_message = users_message.clone().into_bytes();
                buff_message.resize(MESSAGE_SIZE, 0);
                client.write_all(&buff_message).expect("Writing to socket failed");
                println!("{} {:?}", "Message sent".bold().on_blue(), users_message);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });
// USER'S NAME
    println!("{}", "Enter your name:".bold().on_blue());
    loop {
        let mut buff_name = String::new();
        io::stdin().read_line(&mut buff_name).expect("Reading from stdin failed");
        let users_name = buff_name.trim().to_string();
        if users_name == ":quit" || tx.send(users_name).is_err() { break }
        // USER'S MESSAGE
        let mut buff_message = String::new();
        io::stdin().read_line(&mut buff_message).expect("Reading from stdin failed");
        let users_message = buff_message.trim().to_string();
        if users_message == ":quit" || tx.send(users_message).is_err() { break }
    }
// USER'S MESSAGE
    //println!("{}", "Write a message:".bold().on_blue());
    //loop {
        //let mut buff_message = String::new();
        //io::stdin().read_line(&mut buff_message).expect("Reading from stdin failed");
        //let users_message = buff_message.trim().to_string();
        //if users_message == ":quit" || tx.send(users_message).is_err() { break }
    //}
    println!("bye");
}
