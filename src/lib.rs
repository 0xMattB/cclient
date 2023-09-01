pub mod modules;

use modules::ip::IP;
use modules::console;
use modules::console::Color;
use modules::command::{self, CommandType};

use std::io;
use std::process;
use std::error::Error;
use std::thread;
use std::sync::mpsc::{self, Receiver};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

enum ClientState {
	LogIn,
	LoggedIn,
	Disconnect,
}

pub async fn run(ip: &str) -> Result<(), Box<dyn Error>> {
	let ip = ip_get(ip);

    let mut stream = TcpStream::connect(ip.get()).await?;
	let stdin_channel = spawn_stdin_channel();
	let mut state = ClientState::LogIn;
	let mut allow_read = true;
	let mut user = String::default();
	let mut save_username = true;
	
	loop {
		match state {
			ClientState::LogIn => {
				if allow_read {
					if let Some(data) = receive(&mut stream).await {
						allow_read = false;
						
						if data == "Username: " || data == "Password: " {
							console::output(&format!["{data}"], false, Color::Yellow);
						} else {
							console::output(&format!["{data}"], false, Color::Default);
						}
						
						if &data[0..=6] == "Invalid" {
							console::output("", true, Color::Default);
							state = ClientState::Disconnect;
						} else if data == "Logged in" {
							console::output("", true, Color::Default);
							state = ClientState::LoggedIn;
						}
					}
				}
				
				match stdin_channel.try_recv() {
					Ok(key) => {
						allow_read = true;
						
						if save_username {
							user = String::from(key.clone());
							user = clean_string(&user);
							user = trim_null(&user);
							save_username = false;
						}
						
						let _ = stream.write(key.as_bytes()).await;
					},
					Err(_) => {},
				}
			},
			
			ClientState::LoggedIn => {
				if let Some(data) = receive(&mut stream).await {
					match command::command(&data) {
						CommandType::None => {
							if &data[0..=2] == "***" {
								console::output(&format!["{data}"], true, Color::Blue);
							} else {
								print_formatted_broadcast(&data, &user);
							}
						},
						CommandType::Exit => {
							return Ok(());
						},
					}	
				}
				
				match stdin_channel.try_recv() {
					Ok(key) => {
						let _ = stream.write(key.as_bytes()).await;
					},
					Err(_) => {},
				}
			},
			
			ClientState::Disconnect	=> {
				console::output("Disconnected", true, Color::Default);
				break;
			},
		}
	}
	
	Ok(())
}

fn ip_get(ip_str: &str) -> IP {
	let ip = IP::new(ip_str);
	
	match ip {
		Ok(_) => {},
		Err(e) => {
			console::output(&format!["Error: {e}"], true, Color::Default);
			process::exit(0);
		}
	}
	
	ip.unwrap()
}

pub fn input() -> Result<String, &'static str> {
	let mut read = String::new();
	
	if let Ok(_) = io::stdin().read_line(&mut read) {
		Ok(read)
	} else {
		Err("console::input(): error reading from input")
	}
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    
	thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    
	rx
}

async fn receive(stream: &mut TcpStream) -> Option<String> {
	let mut buf = vec![0; 1024];

	if let Ok(_) = stream.try_read(&mut buf) {

		let s = match std::str::from_utf8(&buf) {
			Ok(v) => { v },
			Err(_) => { return None; },
		};
			
		let s = clean_string(&String::from(s));
		let s = trim_null(&s);

		return Some(s);
	}

	None
}

fn clean_string(s: &String) -> String {
	let s = s.replace("\r", "\0");
	let s = s.replace("\n", "\0");
	
	s
}

fn trim_null(s: &String) -> String {
	String::from(s.trim_matches(char::from(0)))
}

fn print_formatted_broadcast(src: &str, username: &str) {
	let mut color = Color::Green;
	
	if let Some(n) = src.find(":") {
		if &src[0..n] == username {
			color = Color::Red;
		}
		
		let s1 = &src[0..=n];
		let s2 = &src[n+1..];
		
		console::output(&format!["{s1}"], false, color);
		console::output(&format!["{s2}"], true, Color::Default);
	}
}