#![feature(mutex_unlock)]
extern crate game_logic;

// use game_logic::game::Game;
// use game_logic::player_enum::*;
use std::io::{BufRead, BufReader, Error};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const MAX_USER_IN_QUEUE: usize = 2;

fn handle_connection(stream: TcpStream, con_struct: Arc<(Mutex<usize>, Condvar)>) {
    let (connections, cvar) = &*con_struct;
    {
        let mut connections = connections.lock().unwrap();
        *connections += 1;
        Mutex::unlock(connections);
    }
    let mut input = String::new();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        match reader.read_line(&mut input) {
            Ok(0) => {
                let mut connections = connections.lock().unwrap();
                *connections -= 1;
                cvar.notify_one();
                break;
            }
            Ok(_) => {
                print!("{}:{}", stream.peer_addr().unwrap(), input);
                input.clear();
            }
            Err(_e) => {
                let mut connections = connections.lock().unwrap();
                *connections -= 1;
                cvar.notify_one();
                break;
            }
        }
    }
}
fn print_connections(con_struct: Arc<(Mutex<usize>, Condvar)>) {
    loop {
        let (connections, _) = &*con_struct;
        let connections = connections.lock().unwrap();
        println!("connections:{}", *connections);
        Mutex::unlock(connections);
        sleep(Duration::new(5, 0));
    }
}

fn main() -> Result<(), Error> {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8080;
    let socket = SocketAddrV4::new(ip, port);
    let listener = TcpListener::bind(socket)?;

    let con_struct = Arc::new((Mutex::new(0), Condvar::new()));

    let con_struct_for_counter = Arc::clone(&con_struct);
    let counter = thread::spawn(move || print_connections(con_struct_for_counter));

    for stream in listener.incoming() {
        let connections_for_children = Arc::clone(&con_struct);
        match stream {
            Ok(stream) => {
                let (connections, cvar) = &*con_struct;
                let mut connections = connections.lock().unwrap();
                while *connections >= MAX_USER_IN_QUEUE {
                    connections = cvar.wait(connections).unwrap();
                }
                thread::spawn(move || {
                    handle_connection(stream, connections_for_children);
                });
            }
            Err(_e) => {}
        }
    }
    counter.join().unwrap();

    Ok(())
}
